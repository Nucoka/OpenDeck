export const ACCENTS = ["#4F6BFF", "#1FB58A", "#FF5C7A", "#A66BFF"] as const;

export type Accent = (typeof ACCENTS)[number];

export type Appearance = {
	accent: Accent;
	blur: number;
	motion: boolean;
};

const STORAGE_KEY = "opendeck.v2.appearance";

const DEFAULT_APPEARANCE: Appearance = {
	accent: "#4F6BFF",
	blur: 12,
	motion: false,
};

export function loadAppearance(): Appearance {
	try {
		const raw = localStorage.getItem(STORAGE_KEY);
		if (!raw) return DEFAULT_APPEARANCE;
		const parsed = JSON.parse(raw) as Partial<Appearance>;
		return {
			accent: ACCENTS.includes(parsed.accent as Accent) ? (parsed.accent as Accent) : DEFAULT_APPEARANCE.accent,
			blur: typeof parsed.blur === "number" ? Math.max(0, Math.min(24, parsed.blur)) : DEFAULT_APPEARANCE.blur,
			motion: typeof parsed.motion === "boolean" ? parsed.motion : DEFAULT_APPEARANCE.motion,
		};
	} catch {
		return DEFAULT_APPEARANCE;
	}
}

export function saveAppearance(value: Appearance) {
	try {
		localStorage.setItem(STORAGE_KEY, JSON.stringify(value));
	} catch {
		// Ignore unavailable local storage.
	}
	applyAppearance(value);
}

export function applyAppearance(value: Appearance) {
	const root = document.documentElement;
	root.style.setProperty("--accent", value.accent);
	root.style.setProperty("--glass-blur", `${value.blur}px`);
	document.body.classList.toggle("motion-enabled", value.motion);
}
