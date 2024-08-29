const { invoke } = window.__TAURI__.tauri;
const { appWindow } = window.__TAURI__.window;

const getElement = (element) => document.querySelector(element);
const getElements = (element) => document.querySelectorAll(element);
const getPlaylistLen = () => invoke("get_playlist_len");

// ***************************************** Logic ************************************************
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

const nextTrack = async () => {
  if ((await getCurrentMusicIndex()) + 1 == getPlaylistLen()) {
    return;
  }

  invoke("next_track").catch((error) => {
    console.error("Error playing next track:", error);
  });

  getElement("#music-name").textContent = await getCurrentMusicName();
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

  getElement("#music-name").textContent = await getCurrentMusicName();
  setMaxMinuteAndProgressBarValue(await getMusicLength());
  mergeProgressBarWithMusic();
};

const previousTrackButton = () => {
  getElement("#previous-track").onclick = () => {
    previousTrack();
  };
};
// ************************************************************************************************

// **************************************** UI Functions ******************************************
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
// ************************************************************************************************

const getCurrentPosition = () => invoke("get_current_position");

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

const getCurrentMusicName = () => invoke("get_current_track_name");

const getVolume = () => invoke("get_volume");

const setDefaultVolume = async () => {
  // refreshProgressbarStyle("#volume-bar");
  changeLinearGradient("volume-bar", await getVolume());
  getElement("#volume-bar").value = await getVolume();
};

const volumeInitialize = () => {
  setVolumeOnInput();
  volumeButtonStates();
  toggleMuteButton();
};

const setVolume = (volume) => {
  invoke("set_volume", { volume: volume / 100 }).catch((error) => {
    console.error("Error setting volume:", error);
  });
};

const setVolumeOnInput = () => {
  $("#volume-bar").on("input", (e) => {
    setVolume(+e.target.value);
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
    } else if (e.target.value > 0 && e.target.value < 25) {
      muteButton.classList.remove("bi-volume-up-fill");
      muteButton.classList.remove("bi-volume-down-fill");
      muteButton.classList.add("bi-volume-off-fill");
      muteButton.classList.remove("bi-volume-mute-fill");
    } else {
      muteButton.classList.remove("bi-volume-up-fill");
      muteButton.classList.remove("bi-volume-down-fill");
      muteButton.classList.remove("bi-volume-off-fill");
      muteButton.classList.add("bi-volume-mute-fill");
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
      setVolume(volumeBar.value);
      changeLinearGradient(volumeBar.id, volumeBar.value);

      // Change icons
      muteButton.classList.add("bi-volume-up-fill");
      muteButton.classList.remove("bi-volume-down-fill");
      muteButton.classList.remove("bi-volume-off-fill");
      muteButton.classList.remove("bi-volume-mute-fill");
    } else {
      // Change value, and refresh the css to it
      volumeBar.value = 0;
      setVolume(volumeBar.value);
      changeLinearGradient(volumeBar.id, volumeBar.value);

      // Change icons
      muteButton.classList.remove("bi-volume-up-fill");
      muteButton.classList.remove("bi-volume-down-fill");
      muteButton.classList.remove("bi-volume-off-fill");
      muteButton.classList.add("bi-volume-mute-fill");
    }
  });
};

// const toggleFavourite = () => {
// 	const favouriteButton = getElement("#favourite-button");
// 	favouriteButton.addEventListener("click", (e) => {
// 		favouriteButton.classList.toggle("bi-heart");
// 		favouriteButton.classList.toggle("bi-heart-fill");
// 	});
// };

const togglePlayButton = () => {
  const playButton = getElement("#play-button");
  playButton.addEventListener("click", async () => {
    if (playButton.classList.contains("bi-play-fill")) {
      resumeAudio();

      playButton.classList.remove("bi-play-fill");
      playButton.classList.add("bi-pause-fill");
    } else {
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

const onInputRefreshProgressBarStyle = (element) => {
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
  let isSeeking = false; // Flag to track if the user is manually changing the progress bar

  // Event listener for manual change of the progress bar
  progressBar.addEventListener("input", () => {
    isSeeking = true;
    const newTime = progressBar.value;

    setCurrentMinuteText(convertSecondsToMinuteText(newTime)); // Update the displayed time
    changeLinearGradient(
      "progress-bar",
      newTime /
        (convertMinuteTextToSeconds(
          getElement("#music-max-minute").textContent
        ) /
          100)
    );
  });

  // Event listener for when the user releases the progress bar
  progressBar.addEventListener("change", () => {
    isSeeking = false;
  });

  async function loop() {
    if (isSeeking) return; // Skip updating the progress bar if the user is manually changing it

    const currentTime = await getCurrentPosition();
    if ((await getCurrentMusicIndex()) == (await getPlaylistLen()) - 1)
      clearInterval(id);

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
  let id = setInterval(loop, 100);
};

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

  onInputRefreshProgressBarStyle("#volume-bar");
  onInputRefreshProgressBarStyle("#progress-bar");
  togglePlaylistMenu();
  nextTrackButton();
  previousTrackButton();
  // startTimer();
  //   refreshCurrentTimeValueToText();
  volumeInitialize();
  setCurrentTime();
  togglePlayButton();
  //   toggleFavourite();

  setProgressbarValueToMusicMinutes(await getCurrentMusicIndex());
  setMaxMinuteAndProgressBarValue(await getMusicLength());
  mergeProgressBarWithMusic();
  setDefaultVolume();
  getElement("#music-name").textContent = await getCurrentMusicName();

  // playPlaylist(playList);
};
