const { invoke } = window.__TAURI__.tauri;
const { appWindow } = window.__TAURI__.window;

const playList = [
  ".\\songs\\DESH - APÁLY (Official Music Video) [AnqYO0TCSG8].mp3",
  ".\\songs\\T. Danny - FURA VAGYOK (feat. Huzugha) (Official Music Video) [mltWrZ0nX0o].mp3",
];

const getElement = (element) => document.querySelector(element);
const getElements = (element) => document.querySelectorAll(element);

async function playPlaylist(filePath) {
  invoke("play_playlist", { filePath }).catch((error) => {
    console.error("Error playing audio:", error);
  });

  setProgressbarValueToMusicMinutes(await getCurrentMusicIndex());
  setMaxMinuteAndProgressBarValue(await getMusicLength());
  mergeProgressBarWithMusic();
}

const pauseAudio = () => {
  invoke("pause_audio").catch((error) => {
    console.error("Error playing audio:", error);
  });
};

function resumeAudio() {
  invoke("resume_audio").catch((error) => {
    console.error("Error playing audio:", error);
  });
}

const setCurrentTime = () => {
  let progressBar = getElement("#progress-bar");

  progressBar.addEventListener("change", () => {
    invoke("set_current_time", { position: +progressBar.value }).catch(
      (error) => {
        console.error("Error setting current time:", error);
      }
    );
  });
};

const nextTrack = async () => {
  if ((await getCurrentMusicIndex()) + 1 == playList.length) {
    return;
  }

  invoke("next_track").catch((error) => {
    console.error("Error playing next track:", error);
  });
  setMaxMinuteAndProgressBarValue(await getMusicLength());
  mergeProgressBarWithMusic();
};

const nextTrackButton = () => {
  getElement("#next-track").onclick = async () => {
    nextTrack();
  };
};

const previousTrack = async () => {
  if ((await getCurrentMusicIndex()) - 1 < 0) {
    return;
  }

  invoke("previous_track").catch((error) => {
    console.error("Error playing next track:", error);
  });
  setMaxMinuteAndProgressBarValue(await getMusicLength());
  mergeProgressBarWithMusic();
};

const previousTrackButton = () => {
  getElement("#previous-track").onclick = () => {
    previousTrack();
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

const getMusicLengthByIndex = (musicIndex) => {
  return invoke("get_audio_length", { musicIndex });
};

const getMusicLength = async () => {
  return invoke("get_audio_length", {
    musicIndex: await getCurrentMusicIndex(),
  });
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
    if (e.target.value < 100 && e.target.value > 75) {
      muteButton.classList.add("bi-volume-up-fill");
      muteButton.classList.remove("bi-volume-down-fill");
      muteButton.classList.remove("bi-volume-off-fill");
      muteButton.classList.remove("bi-volume-mute-fill");
    } else if (e.target.value >= 25 && e.target.value <= 75) {
      muteButton.classList.remove("bi-volume-up-fill");
      muteButton.classList.add("bi-volume-down-fill");
      muteButton.classList.remove("bi-volume-off-fill");
      muteButton.classList.remove("bi-volume-mute-fill");
    } else if (e.target.value > 0 && e.target.value < 25) {
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

  let id = setInterval(loop, 500);
  async function loop() {
    const currentTime = await getCurrentPosition();

    if (currentTime == (await getMusicLength())) {
      clearInterval(id);
      nextTrack();
    }

    setCurrentMinuteText(convertSecondsToMinuteText(currentTime));
    changeLinearGradient(
      "progress-bar",
      currentTime /
        (convertMinuteTextToSeconds(
          getElement("#music-max-minute").textContent
        ) /
          100)
    );
    progressBar.value = currentTime;
  }
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

const setMaxMinuteAndProgressBarValue = async (music_length) => {
  let maxValue = getElement("#music-max-minute");
  let progressBar = getElement("#progress-bar");
  progressBar.max = music_length;
  maxValue.textContent = convertSecondsToMinuteText(music_length);
};

const setProgressbarValueToMusicMinutes = async (musicIndex) => {
  const progressBar = getElement("#progress-bar");
  const musicMaxValue = await getMusicLengthByIndex(musicIndex);
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

  refreshProgressbarStyle("#volume-bar");
  refreshProgressbarStyle("#progress-bar");
  togglePlaylistMenu();
  nextTrackButton();
  previousTrackButton();
  // startTimer();
  //   refreshCurrentTimeValueToText();
  volumeInitialize();
  setCurrentTime();
  togglePlayButton();
  toggleFavourite();

  playPlaylist(playList);
};
