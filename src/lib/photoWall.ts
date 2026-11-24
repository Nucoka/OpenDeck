export type PhotoWallMode = "cover" | "fit";

export type PhotoWallOptions = {
	columns: number;
	rows: number;
	tileWidth: number;
	tileHeight: number;
	mode: PhotoWallMode;
	gap?: number;
	background?: string;
};

export type PhotoWallResult = {
	preview: HTMLCanvasElement;
	tiles: string[];
};

const isDev = (import.meta as ImportMeta & { env: { DEV: boolean } }).env.DEV;

export function loadImage(file: File): Promise<HTMLImageElement> {
	if (!file.type.startsWith("image/")) return Promise.reject(new Error("Selected file is not an image"));

	return new Promise((resolve, reject) => {
		const reader = new FileReader();
		reader.onerror = () => reject(reader.error);
		reader.onload = () => {
			const image = document.createElement("img");
			image.onload = () => resolve(image);
			image.onerror = () => reject(new Error("Failed to decode image"));
			image.src = reader.result?.toString() ?? "";
		};
		reader.readAsDataURL(file);
	});
}

export function slicePhoto(image: HTMLImageElement, opts: PhotoWallOptions): PhotoWallResult {
	const { columns, rows, tileWidth, tileHeight } = opts;
	const gap = opts.gap ?? 0;
	const background = opts.background ?? "#000000";

	// The grid canvas includes the physical gaps; tiles skip those strips so the photo stays continuous across buttons.
	const width = columns * tileWidth + (columns - 1) * gap;
	const height = rows * tileHeight + (rows - 1) * gap;

	const preview = document.createElement("canvas");
	preview.width = width;
	preview.height = height;
	const context = preview.getContext("2d");
	if (!context) throw new Error("Failed to create photo wall canvas");

	context.fillStyle = background;
	context.fillRect(0, 0, width, height);

	const scale =
		opts.mode === "cover"
			? Math.max(width / image.naturalWidth, height / image.naturalHeight)
			: Math.min(width / image.naturalWidth, height / image.naturalHeight);
	const scaledWidth = image.naturalWidth * scale;
	const scaledHeight = image.naturalHeight * scale;

	context.imageSmoothingQuality = "high";
	context.drawImage(image, (width - scaledWidth) / 2, (height - scaledHeight) / 2, scaledWidth, scaledHeight);

	if (isDev) {
		console.assert(preview.width === width && preview.height === height, "Photo wall preview has wrong dimensions");
	}

	const tiles: string[] = [];
	for (let r = 0; r < rows; r++) {
		for (let c = 0; c < columns; c++) {
			const tile = document.createElement("canvas");
			tile.width = tileWidth;
			tile.height = tileHeight;
			const tileContext = tile.getContext("2d");
			if (!tileContext) throw new Error("Failed to create photo wall tile");

			tileContext.fillStyle = background;
			tileContext.fillRect(0, 0, tileWidth, tileHeight);
			tileContext.imageSmoothingQuality = "high";
			tileContext.drawImage(preview, c * (tileWidth + gap), r * (tileHeight + gap), tileWidth, tileHeight, 0, 0, tileWidth, tileHeight);
			tiles.push(tile.toDataURL("image/jpeg", 0.9));

			if (isDev) {
				console.assert(tile.width === tileWidth && tile.height === tileHeight, "Photo wall tile has wrong dimensions");
			}
		}
	}

	if (isDev) {
		console.assert(tiles.length === rows * columns, "Photo wall tile count does not match the grid");
	}

	return { preview, tiles };
}
