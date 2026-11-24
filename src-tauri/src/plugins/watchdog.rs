use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write as _;
use std::sync::LazyLock;
use std::sync::Mutex as StdMutex;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use tauri::{Emitter, Manager};

use crate::store::{NotProfile, Store};

pub const MAX_RESTART_ATTEMPTS: u32 = 5;
pub const HEALTHY_UPTIME_SECS: u64 = 60;
const RESTART_DELAYS_SECS: [u64; MAX_RESTART_ATTEMPTS as usize] = [1, 2, 4, 8, 16];

pub static SHUTDOWN: AtomicBool = AtomicBool::new(false);

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
struct WatchdogSettings {
	auto_restart: HashMap<String, bool>,
}

impl NotProfile for WatchdogSettings {}

#[derive(Clone, Serialize)]
pub struct PluginStatus {
	pub id: String,
	pub status: String,
	pub attempts: u32,
	pub exit_code: Option<i32>,
}

struct Entry {
	generation: u64,
	started: Instant,
	attempts: u32,
	status: PluginStatus,
}

static SETTINGS: LazyLock<StdMutex<Store<WatchdogSettings>>> = LazyLock::new(|| StdMutex::new(Store::new_concurrent("watchdog", &crate::shared::config_dir(), WatchdogSettings::default())));
static STATES: LazyLock<StdMutex<HashMap<String, Entry>>> = LazyLock::new(|| StdMutex::new(HashMap::new()));
static GENERATION: AtomicU64 = AtomicU64::new(0);

fn settings() -> std::sync::MutexGuard<'static, Store<WatchdogSettings>> {
	SETTINGS.lock().unwrap_or_else(|error| error.into_inner())
}

fn states() -> std::sync::MutexGuard<'static, HashMap<String, Entry>> {
	STATES.lock().unwrap_or_else(|error| error.into_inner())
}

fn next_generation() -> u64 {
	GENERATION.fetch_add(1, Ordering::Relaxed) + 1
}

pub fn auto_restart_enabled(id: &str) -> bool {
	settings().value.auto_restart.get(id).copied().unwrap_or(true)
}

pub fn set_auto_restart(id: &str, enabled: bool) {
	settings().value.auto_restart.insert(id.to_owned(), enabled);
	if let Err(error) = settings().save() {
		log::warn!("Failed to save watchdog settings: {error}");
	}
}

pub fn get_status(id: &str) -> PluginStatus {
	states().get(id).map(|entry| entry.status.clone()).unwrap_or(PluginStatus {
		id: id.to_owned(),
		status: "running".to_owned(),
		attempts: 0,
		exit_code: None,
	})
}

fn emit(status: &PluginStatus) {
	if let Some(app) = crate::APP_HANDLE.get()
		&& let Some(window) = app.get_webview_window("main")
	{
		let _ = window.emit("plugin_status", status);
	}
}

fn log_exit(id: &str, exit_code: Option<i32>) {
	let path = crate::shared::log_dir().join("plugins").join(format!("{id}.log"));
	if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
		let code = exit_code.map_or_else(|| "unknown".to_owned(), |code| code.to_string());
		let _ = writeln!(file, "[watchdog] process exited with code {code}");
	}
}

pub fn on_spawn(id: &str) {
	let generation = next_generation();
	let mut map = states();
	let entry = map.entry(id.to_owned()).or_insert_with(|| Entry {
		generation,
		started: Instant::now(),
		attempts: 0,
		status: PluginStatus {
			id: id.to_owned(),
			status: "running".to_owned(),
			attempts: 0,
			exit_code: None,
		},
	});
	entry.generation = generation;
	entry.started = Instant::now();
	entry.status.status = "running".to_owned();
	entry.status.exit_code = None;
	entry.status.attempts = entry.attempts;
	emit(&entry.status);
}

fn reset_healthy(live: &[String]) {
	let mut changed = Vec::new();
	{
		let mut map = states();
		for (id, entry) in map.iter_mut() {
			if live.contains(id) && entry.attempts > 0 && entry.started.elapsed() >= Duration::from_secs(HEALTHY_UPTIME_SECS) {
				entry.attempts = 0;
				entry.status.attempts = 0;
				entry.status.status = "running".to_owned();
				changed.push(entry.status.clone());
			}
		}
	}
	for status in changed {
		emit(&status);
	}
}

async fn handle_crash(id: String, generation: u64, exit_code: Option<i32>) {
	if SHUTDOWN.load(Ordering::Relaxed) {
		return;
	}

	let enabled = auto_restart_enabled(&id);
	let attempts = {
		let mut map = states();
		let Some(entry) = map.get_mut(&id) else {
			return;
		};
		if entry.generation != generation {
			return;
		}
		entry.attempts += 1;
		entry.status.status = "crashed".to_owned();
		entry.status.attempts = entry.attempts;
		entry.status.exit_code = exit_code;
		entry.attempts
	};
	emit(&get_status(&id));

	if !enabled || attempts > MAX_RESTART_ATTEMPTS {
		let mut map = states();
		if let Some(entry) = map.get_mut(&id) {
			entry.status.status = "failed".to_owned();
		}
		emit(&get_status(&id));
		return;
	}

	{
		let mut map = states();
		if let Some(entry) = map.get_mut(&id) {
			entry.status.status = "restarting".to_owned();
		}
	}
	emit(&get_status(&id));

	let delay = RESTART_DELAYS_SECS[(attempts - 1) as usize];
	tokio::spawn(async move {
		tokio::time::sleep(Duration::from_secs(delay)).await;
		restart(&id, false).await;
	});
}

pub async fn restart(id: &str, reset_attempts: bool) {
	if SHUTDOWN.load(Ordering::Relaxed) || !crate::shared::config_dir().join("plugins").join(id).exists() {
		return;
	}

	{
		let mut map = states();
		let generation = next_generation();
		let entry = map.entry(id.to_owned()).or_insert_with(|| Entry {
			generation,
			started: Instant::now(),
			attempts: 0,
			status: PluginStatus {
				id: id.to_owned(),
				status: "restarting".to_owned(),
				attempts: 0,
				exit_code: None,
			},
		});
		entry.generation = generation;
		if reset_attempts {
			entry.attempts = 0;
		}
		entry.status.status = "restarting".to_owned();
		entry.status.attempts = entry.attempts;
		entry.status.exit_code = None;
	}
	emit(&get_status(id));

	crate::events::clear_plugin_queue(id).await;
	if let Some(app) = crate::APP_HANDLE.get() {
		crate::events::frontend::plugins::reload_plugin(app.clone(), id.to_owned()).await;
	}

	{
		let mut map = states();
		if let Some(entry) = map.get_mut(id) {
			entry.status.status = "running".to_owned();
		}
	}
	emit(&get_status(id));
}

pub fn start_monitor() {
	tokio::spawn(async {
		loop {
			tokio::time::sleep(Duration::from_secs(1)).await;
			if SHUTDOWN.load(Ordering::Relaxed) {
				break;
			}

			let mut crashes = Vec::new();
			let mut live = Vec::new();
			{
				let mut instances = crate::plugins::INSTANCES.lock().await;
				for (id, instance) in instances.iter_mut() {
					let Some(child) = instance.child_mut() else {
						continue;
					};
					live.push(id.clone());
					match child.try_wait() {
						Ok(Some(exit)) => {
							let code = exit.code();
							log_exit(id, code);
							let generation = states().get(id).map(|entry| entry.generation).unwrap_or(0);
							crashes.push((id.clone(), generation, code));
						}
						Ok(None) => {}
						Err(error) => log::warn!("Failed to check plugin {id}: {error}"),
					}
				}
				for (id, _, _) in &crashes {
					instances.remove(id);
				}
			}

			reset_healthy(&live);
			for (id, generation, code) in crashes {
				handle_crash(id, generation, code).await;
			}
		}
	});
}
