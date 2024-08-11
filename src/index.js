const { invoke } = window.__TAURI__.tauri;
const { appWindow } = window.__TAURI__.window;

const musicList = [
	"..\\src-tauri\\songs\\DESH - APÁLY (Official Music Video) [AnqYO0TCSG8].mp3",
	"..\\src-tauri\\songs\\T. Danny - FURA VAGYOK (feat. Huzugha) (Official Music Video) [mltWrZ0nX0o].mp3",
];

const getElement = (element) => document.querySelector(element);
const getElements = (element) => document.querySelectorAll(element);

function playAudio(filePath) {
	invoke("play_playlist", { filePath }).catch((error) => {
		console.error("Error playing audio:", error);
	});
}

playAudio(musicList);

function pauseAudio() {
	invoke("pause_audio").catch((error) => {
		console.error("Error playing audio:", error);
	});
}

function resumeAudio() {
	invoke("resume_audio").catch((error) => {
		console.error("Error playing audio:", error);
	});
}

const nextTrack = () => {
	getElement("#next-track").onclick = async () => {
		let nextTrackIndex = await getCurrentMusicIndex();
		if ((await getCurrentMusicIndex()) + 1 != musicList.length) {
			nextTrackIndex = (await getCurrentMusicIndex()) + 1;
		}

		getElement("#volume-bar").value = (await getVolume()) * 100;

		changeLinearGradient("volume-bar", (await getVolume()) * 100);
		const nextMusicLenght = await getAudioLenght(nextTrackIndex);
		setMaxMinuteText(nextMusicLenght);
		invoke("next_track").catch((error) => {
			console.error("Error playing next track:", error);
		});
	};
};
const previousTrack = () => {
	getElement("#previous-track").onclick = async () => {
		let previousTrackIndex = await getCurrentMusicIndex();
		if ((await getCurrentMusicIndex()) - 1 >= 0) {
			previousTrackIndex = (await getCurrentMusicIndex()) - 1;
		}

		getElement("#volume-bar").value = await getVolume();
		changeLinearGradient("volume-bar", await getVolume());
		const previousMusicLenght = await getAudioLenght(previousTrackIndex);
		setMaxMinuteText(previousMusicLenght);
		invoke("previous_track").catch((error) => {
			console.error("Error playing next track:", error);
		});
	};
};

const getCurrentPosition = () => {
	return invoke("get_current_position");
};

const volumeInitialize = () => {
	setVolume();
	volumeButtonStates();
	toggleMuteButton();
};

const getAudioLenght = (musicIndex) => {
	return invoke("get_audio_length", { musicIndex: musicIndex });
};

const getCurrentMusicIndex = () => {
	return invoke("get_current_music_index");
};

const getVolume = () => {
	return invoke("get_volume");
};
const setVolume = () => {
	$("#volume-bar").on("input", (e) => {
		invoke("set_volume", { volume: +e.target.value / 100 }).catch((error) => {
			console.error("Error setting volume:", error);
		});
	});
};

const volumeButtonStates = () => {
	const muteButton = getElement("#mute-button");

	$("#volume-bar").on("input", (e) => {
		if (e.target.value <= 100 && e.target.value > 75) {
			muteButton.classList.add("bi-volume-up-fill");
			muteButton.classList.remove("bi-volume-down-fill");
			muteButton.classList.remove("bi-volume-off-fill");
			muteButton.classList.remove("bi-volume-mute-fill");
		} else if (e.target.value >= 25 && e.target.value <= 75) {
			muteButton.classList.remove("bi-volume-up-fill");
			muteButton.classList.add("bi-volume-down-fill");
			muteButton.classList.remove("bi-volume-off-fill");
			muteButton.classList.remove("bi-volume-mute-fill");
		} else if (e.target.value >= 0 && e.target.value < 25) {
			muteButton.classList.remove("bi-volume-up-fill");
			muteButton.classList.remove("bi-volume-down-fill");
			muteButton.classList.add("bi-volume-off-fill");
			muteButton.classList.remove("bi-volume-mute-fill");
		}
	});
};

const toggleMuteButton = () => {
	const muteButton = getElement("#mute-button");

	// If the volume is changed then change back the icon to unmuted state
	muteButton.addEventListener("click", (e) => {
		const volumeBar = getElement("#volume-bar");

		if (volumeBar.value == 0) {
			// Change value, and refresh the css to it
			volumeBar.value = 100;
			changeLinearGradient(volumeBar.id, volumeBar.value);

			// Change icons
			muteButton.classList.add("bi-volume-up-fill");
			muteButton.classList.remove("bi-volume-mute-fill");
		} else {
			// Change value, and refresh the css to it
			volumeBar.value = 0;
			changeLinearGradient(volumeBar.id, volumeBar.value);

			// Change icons
			muteButton.classList.add("bi-volume-mute-fill");
			muteButton.classList.remove("bi-volume-up-fill");
		}
	});
};

const toggleFavourite = () => {
	const favouriteButton = getElement("#favourite-button");
	favouriteButton.addEventListener("click", (e) => {
		favouriteButton.classList.toggle("bi-heart");
		favouriteButton.classList.toggle("bi-heart-fill");
	});
};

const togglePlayButton = () => {
	const playButton = getElement("#play-button");

	playButton.addEventListener("click", async () => {
		if (playButton.classList.contains("bi-play-fill")) {
			// audioElement.play();
			resumeAudio();
			// startTimer();
			playButton.classList.remove("bi-play-fill");
			playButton.classList.add("bi-pause-fill");
		} else {
			// audioElement.pause();
			pauseAudio();
			playButton.classList.remove("bi-pause-fill");
			playButton.classList.add("bi-play-fill");
		}
	});
};

const togglePlaylistMenu = () => {
	document
		.getElementById("playlistButton")
		.addEventListener("click", async (event) => {
			event.preventDefault();
			const menuVisibility = getElement(".play-list");
			menuVisibility.classList.toggle("d-none");

			await invoke("resize_window");
		});
};

const refreshProgressbarStyle = (element) => {
	$(element).on("input", (e) => {
		changeLinearGradient(e.target.id, e.target.value / (e.target.max / 100));
	});
};

const changeLinearGradient = (elementId, value) => {
	$(":root").css(`--${elementId}-value`, `${value}%`);
};

const refreshCurrentTimeValueToText = () => {
	const progressBar = getElement("#progress-bar");
	progressBar.addEventListener("input", (e) => {
		setCurrentMinuteText(convertSecondsToMinuteText(e.target.value));
	});
};

const mergeProgressBarWithMusic = () => {
	const progressBar = getElement("#progress-bar");

	// progressBar.addEventListener("input", (e) => {
	//     musicElement.currentTime = e.target.value;
	// });

	setInterval(async () => {
		const currentTime = await getCurrentPosition();
		// console.log(currentTime);

		setCurrentMinuteText(convertSecondsToMinuteText(currentTime));
		changeLinearGradient(
			"progress-bar",
			currentTime /
				(convertMinuteTextToSeconds(getElement("#musicMaxMinute").textContent) /
					100)
		);
		progressBar.value = currentTime;
	}, 500);
};

// const startTimer = () => {
// 	var timer = parseInt(getElement("#progress-bar").value, 10);
// 	let asd = setInterval(() => {
// 		timer += 1;
// 		getElement("#progress-bar").value = timer;

// 		changeLinearGradient(
// 			"progress-bar",
// 			timer /
// 				(convertMinuteTextToSeconds(getElement("#musicMaxMinute").textContent) /
// 					100)
// 		);
// 	}, 1000);
// 	getElement("#play-button").onclick = () => {
// 		clearTimeout(asd);
// 	};
// };

const setMaxMinuteText = async (music_length) => {
	let maxValue = getElement("#musicMaxMinute");
	maxValue.textContent = convertSecondsToMinuteText(music_length);
};

const setProgressbarValueToMusicMinutes = async (musicIndex) => {
	const progressBar = getElement("#progress-bar");
	const musicMaxValue = await getAudioLenght(musicIndex);
	progressBar.max = musicMaxValue;
};

const setCurrentMinuteText = (text) => {
	getElement("#current-minute").textContent = text;
};

const convertSecondsToMinuteText = (seconds) => {
	var date = new Date(0);
	date.setSeconds(seconds);
	return date.toISOString().substring(11, 19);
};

const convertMinuteTextToSeconds = (time) =>
	time.split(":").reduce((s, t) => s * 60 + +t, 0);

window.onload = async () => {
	document.getElementById("settings").addEventListener("click", async (e) => {
		e.preventDefault();
		await invoke("create_settings");
	});

	document.getElementById("minimize").addEventListener("click", (e) => {
		e.preventDefault();
		appWindow.minimize();
	});

	document.getElementById("close").addEventListener("click", (e) => {
		e.preventDefault();
		appWindow.close();
	});

	setProgressbarValueToMusicMinutes(0);
	refreshProgressbarStyle("#volume-bar");
	refreshProgressbarStyle("#progress-bar");
	togglePlaylistMenu();
	nextTrack();
	previousTrack();
	// startTimer();
	mergeProgressBarWithMusic();
	setMaxMinuteText(await getAudioLenght(await getCurrentMusicIndex()));
	refreshCurrentTimeValueToText();
	volumeInitialize();
	togglePlayButton();
	toggleFavourite();
};
