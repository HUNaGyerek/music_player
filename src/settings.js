const { invoke } = window.__TAURI__.tauri;
const { appWindow } = window.__TAURI__.window;

const toggleSettings = async () => {
	appWindow.close(); // Close the current window
};

window.onload = () => {
	document
		.querySelector("#backButton")
		.addEventListener("click", toggleSettings);
};
