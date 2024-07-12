const { invoke } = window.__TAURI__.tauri;
const { appWindow } = window.__TAURI__.window;

window.onload = () => {
  document.getElementById("minimize").addEventListener("click", () => {
    appWindow.minimize();
  });

  document.getElementById("close").addEventListener("click", () => {
    appWindow.close();
  });

  setProgressbarValueToMusicMinutes();
  refreshProgressbarStyle();
  togglePlaylistMenu();
};

const togglePlaylistMenu = async () => {
  document
    .getElementById("playlistButton")
    .addEventListener("click", async (event) => {
      event.preventDefault();
      const menuVisibility = document.querySelector(".play-list");
      menuVisibility.classList.toggle("d-none");

      await invoke("resize_window");
    });
};

const refreshProgressbarStyle = () => {
  $('input[type="range"]').on("input", (e) => {
    var sheets = document.styleSheets[2].cssRules;
    for (var i = 0; i < sheets.length; i++) {
      if (
        sheets[i].selectorText ===
        `#${e.target.id}::-webkit-slider-runnable-track`
      ) {
        var date = new Date(0);
        date.setSeconds(e.target.value); // specify value for SECONDS here
        var timeString = date.toISOString().substring(14, 19);
        setCurrentMinuteText(timeString);

        const val = e.target.value / (e.target.max / 100);
        sheets[
          i
        ].style.background = `linear-gradient(to right, #00a3ff ${val}%, #d9d9d9 ${val}%)`;
      }
    }
  });
};

function setProgressbarValueToMusicMinutes() {
  const progressBar = document.getElementById("progressbar");
  const musicMaxValue = convertMinuteTextToSeconds(
    $("#musicMaxMinute")[0].textContent
  );
  progressBar.max = musicMaxValue;
}

const setCurrentMinuteText = (text) => {
  $("#currentMinute")[0].textContent = text;
};

const convertMinuteTextToSeconds = (time) =>
  time.split(":").reduce((s, t) => s * 60 + +t, 0);
