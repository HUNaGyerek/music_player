export function convertSecondsToMinuteText(seconds) {
	if (typeof +seconds !== Number) return;
	var date = new Date(0);
	date.setSeconds(seconds);
	return date.toISOString().substring(11, 19);
}

export const convertMinuteTextToSeconds = (time) =>
	time.split(':').reduce((s, t) => s * 60 + +t, 0);

export function disableContextMenu() {
	if (window.location.hostname !== 'tauri.localhost') {
		return;
	}

	document.addEventListener(
		'contextmenu',
		(e) => {
			e.preventDefault();
			return false;
		},
		{ capture: true }
	);

	document.addEventListener(
		'selectstart',
		(e) => {
			e.preventDefault();
			return false;
		},
		{ capture: true }
	);
}
