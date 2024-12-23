export function convertSecondsToMinuteText(seconds) {
	var date = new Date(0);
	date.setSeconds(seconds);
	return date.toISOString().substring(11, 19);
}

export const convertMinuteTextToSeconds = (time) =>
	time.split(':').reduce((s, t) => s * 60 + +t, 0);
