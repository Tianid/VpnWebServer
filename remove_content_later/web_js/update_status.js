const socket = new WebSocket(`ws://${SERVER_HOST}:${SERVER_PORT}/ws`);
const circle = document.getElementById("statusCircle");
const button = document.getElementById("vpnButton");
const statusText = document.getElementById("statusText")


const VpnStatus = Object.freeze({
    DISCONNECTED: "disconnected",
    CONNECTING: "connecting",
    CONNECTED: "connected",
    DISCONNECTING: "disconnecting",
    RECONNECTING: "reconnecting"
});


const VpnStatusText = {
    0: "Отключен",
    1: "Подключение...",
    2: "Подключен",
    3: "Отключение..."
};

socket.onmessage = function (event) {
    // FIXME change status here for elements
    try {
        let data = JSON.parse(event.data);
        let status = data.status;
        updateUI(status);
    } catch (error) {
        console.error("WebSocket message error", error);
    }
};

socket.onopen = function () {
    console.log(`WebSocket connected to ws://${SERVER_HOST}:${SERVER_PORT}/ws`)
};

/*socket.onerror = function (error) {
    console.error("WebSocket error:", error)
};
*/
/*socket.addEventListener("error", (event) => {
    console.log("WebSocket error: ", event);
});*/

socket.onclose = function () {
    console.log("WebSocket connection closed")
};



function updateUI(status) {
    if (status === VpnStatus.DISCONNECTED) {
        circle.style.background = "red";
        button.innerText = "Connect";
        button.disabled = false;
        button.onclick = () => sendRequest("/start")
    } else if (status === VpnStatus.CONNECTED) {
        circle.style.background = "green";
        button.innerText = "Disconnect";
        button.disabled = false;
        button.onclick = () => sendRequest("/stop")
    } else {
        circle.style.background = "orange";
        button.innerText = "Disconnect";
        button.disabled = true;
        button.onclick = () => sendRequest("/stop")
    }

    statusText.innerText = VpnStatusText[status]; т
}

function sendRequest(endpoint) {
    fetch(endpoint, { method: "POST" })
        .then(response => response.text())
        .then(data => console.log("Ответ сервера:", data))
        .catch(error => console.error("Ошибка:", error));
}
