use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use tauri::Manager;
use zip::write::{FileOptions, ZipWriter};

use crate::shared::{PRODUCT_NAME, config_dir};

pub const BACKUP_KEEP: usize = 10;
pub const BACKUP_MIN_INTERVAL_MINUTES: u64 = 30;

fn is_transient(path: &Path) -> bool {
	matches!(path.extension().and_then(|extension| extension.to_str()), Some("temp" | "bak" | "part"))
}

fn collect_transient_paths(dir: &Path, backups: &Path, paths: &mut Vec<PathBuf>) {
	let Ok(entries) = fs::read_dir(dir) else {
		return;
	};
	for entry in entries.flatten() {
		let path = entry.path();
		if path == backups || path.file_name().and_then(|name| name.to_str()) == Some("temp") {
			continue;
		}
		if path.is_dir() {
			collect_transient_paths(&path, backups, paths);
		} else if is_transient(&path) {
			paths.push(path);
		}
	}
}

fn builtin_plugin_paths(config: &Path) -> Vec<PathBuf> {
	let Some(app) = crate::APP_HANDLE.get() else {
		return Vec::new();
	};
	let Ok(resources) = app.path().resolve("plugins", tauri::path::BaseDirectory::Resource) else {
		return Vec::new();
	};
	let Ok(entries) = fs::read_dir(resources) else {
		return Vec::new();
	};
	entries.flatten().map(|entry| config.join("plugins").join(entry.file_name())).collect()
}

fn should_backup(backups: &Path) -> bool {
	let latest = fs::read_dir(backups)
		.into_iter()
		.flatten()
		.flatten()
		.map(|entry| entry.path())
		.filter(|path| path.extension().and_then(|extension| extension.to_str()) == Some("zip"))
		.filter_map(|path| fs::metadata(&path).and_then(|metadata| metadata.modified()).ok())
		.max();
	let Some(latest) = latest else {
		return true;
	};
	match SystemTime::now().duration_since(latest) {
		Ok(elapsed) => elapsed >= Duration::from_secs(BACKUP_MIN_INTERVAL_MINUTES * 60),
		Err(_) => false,
	}
}

fn rotate(backups: &Path) {
	let mut archives: Vec<PathBuf> = fs::read_dir(backups)
		.into_iter()
		.flatten()
		.flatten()
		.map(|entry| entry.path())
		.filter(|path| path.extension().and_then(|extension| extension.to_str()) == Some("zip"))
		.collect();
	archives.sort();

	while archives.len() > BACKUP_KEEP {
		let old = archives.remove(0);
		if let Err(error) = fs::remove_file(&old) {
			log::warn!("Failed to remove old backup {}: {error}", old.display());
		}
	}
}

pub fn create_startup_backup() -> anyhow::Result<()> {
	let config = config_dir();
	let backups = config.join("backups");
	fs::create_dir_all(&backups)?;
	if !should_backup(&backups) {
		return Ok(());
	}

	let name = format!("{}_config_{}.zip", PRODUCT_NAME, chrono::Local::now().format("%Y%m%d_%H%M%S"));
	let path = backups.join(&name);
	let temp_path = backups.join(format!("{name}.part"));

	let result = (|| -> anyhow::Result<()> {
		let file = File::create(&temp_path)?;
		let mut paths = vec![temp_path.clone(), backups.clone(), config.join("temp")];
		collect_transient_paths(&config, &backups, &mut paths);
		paths.extend(builtin_plugin_paths(&config));

		let mut zip = ZipWriter::new(file);
		let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
		crate::events::frontend::settings::add_dir_to_zip(&mut zip, &config, &config, options, &paths)?;
		zip.finish()?;
		fs::rename(&temp_path, &path)?;
		Ok(())
	})();

	if result.is_err() {
		let _ = fs::remove_file(&temp_path);
		return result;
	}

	rotate(&backups);
	Ok(())
}
