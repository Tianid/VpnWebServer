const host = window.location.hostname;
const port = window.location.port;
const socket = new WebSocket(`ws://${host}:${port}/ws`);

let isConnected = false;

// DOM elements
const indicator = document.getElementById("vpnStatus");
const statusText = document.getElementById("statusText");
const vpnToggleButton = document.getElementById("vpnToggleButton");
const wifiReconnectButton = document.getElementById("wifiReconnectButton");
const refreshStatusButton = document.getElementById("refreshStatusButton");

// === Translations for RU and EN ===
const translations = {
  en: {
    connected: "Connected",
    disconnected: "Disconnected",
    inProcess: "In Process",
    connectBtn: "Connect",
    disconnectBtn: "Disconnect",
    wifiBtn: "Reconnect WiFi",
    refreshBtn: "Refresh"
  },
  ru: {
    connected: "Подключено",
    disconnected: "Отключено",
    inProcess: "В процессе",
    connectBtn: "Подключиться",
    disconnectBtn: "Отключиться",
    wifiBtn: "Переподключить WiFi",
    refreshBtn: "Обновить"
  }
};

// === Language detection ===
const userLang = navigator.language.toLowerCase();
const isRussian = userLang.startsWith("ru");
const locale = isRussian ? "ru" : "en";
const t = translations[locale];

// === Apply initial UI text ===
document.addEventListener("DOMContentLoaded", () => {
  if (vpnToggleButton) vpnToggleButton.textContent = t.connectBtn;
  if (wifiReconnectButton) wifiReconnectButton.textContent = t.wifiBtn;
  if (refreshStatusButton) refreshStatusButton.textContent = t.refreshBtn;
});

// === Update visual state ===
function updateStatusUI(state) {
  switch (state) {
    case "Connected":
      indicator.style.backgroundColor = "green";
      statusText.textContent = t.connected;
      vpnToggleButton.textContent = t.disconnectBtn;
      setButtonsEnabled(true);
      isConnected = true;
      break;

    case "Disconnected":
      indicator.style.backgroundColor = "red";
      statusText.textContent = t.disconnected;
      vpnToggleButton.textContent = t.connectBtn;
      setButtonsEnabled(true);
      isConnected = false;
      break;

    case "InProcess":
      indicator.style.backgroundColor = "orange";
      statusText.textContent = t.inProcess;
      vpnToggleButton.textContent = t.disconnectBtn;
      setButtonsEnabled(false);
      break;
  }
}

// === Enable or disable all action buttons ===
function setButtonsEnabled(enabled) {
  vpnToggleButton.disabled = !enabled;
  wifiReconnectButton.disabled = !enabled;
  refreshStatusButton.disabled = !enabled;
}

// === WebSocket open: request initial status ===
socket.addEventListener("open", function () {
  const message = JSON.stringify({ request_type: "Status" });
  socket.send(message);
});

// === Handle messages from server ===
socket.addEventListener("message", function (event) {
  console.log(`Received message from server: ${event.data}`);

  try {
    const data = JSON.parse(event.data);
    if (data.status === "Connected") {
      updateStatusUI("Connected");
    } else if (data.status === "Disconnected") {
      updateStatusUI("Disconnected");
    }
  } catch (e) {
    console.warn("Failed to parse server message as JSON:", event.data);
  }
});

socket.onerror = (error) => {
  console.error("WebSocket error:", error);
};

// === Button click handlers ===
document.addEventListener("DOMContentLoaded", () => {
  if (vpnToggleButton) {
    vpnToggleButton.addEventListener("click", () => {
      const message = {
        request_type: isConnected ? "Disconnect" : "Connect"
      };
      updateStatusUI("InProcess");
      socket.send(JSON.stringify(message));
    });
  }

  if (wifiReconnectButton) {
    wifiReconnectButton.addEventListener("click", () => {
      updateStatusUI("InProcess");
      socket.send(JSON.stringify({ request_type: "ReconnectToWiFi" }));
    });
  }

  if (refreshStatusButton) {
    refreshStatusButton.addEventListener("click", () => {
      updateStatusUI("InProcess");
      const message = JSON.stringify({ request_type: "Status" });
      socket.send(message);
    });
  }
});

