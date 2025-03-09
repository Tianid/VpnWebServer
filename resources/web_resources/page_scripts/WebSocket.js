const socket = new WebSocket(`ws://127.0.0.1:9000/ws`, `http`);

socket.addEventListener("open", function (event) {
  socket.send("Connection Established");
});

socket.addEventListener("message", function (event) {
  console.log(`Receive from server message: ${event.data}`);
  if (new String(event.data) == "Connection Established") {
    console.log("Mesage sended to back");
    socket.send("Send again from Client");
  }
});

/*addEventListener("error", function (error) {
    console.log(error);
}); */

socket.onerror = (error) => console.error("ERROR:", error);

const contactServer = () => {
  socket.send("Initialize");
};


// document.getElementById("sendButton").addEventListener("click", () => {
//     if (socket.readyState === WebSocket.OPEN) {
//         socket.send("Привет, сервер! Это сообщение от клиента.");
//         console.log("Сообщение отправлено");
//     } else {
//         console.error("WebSocket не подключен");
//     }
// });



document.addEventListener("DOMContentLoaded", () => {
    const button = document.getElementById("sendButton");

    if (button) {
        button.addEventListener("click", () => {
          socket.send("Привет, сервер! Это сообщение от клиента.");
          console.log("Кнопка нажата!");
        });
    } else {
        console.error("Ошибка: Кнопка не найдена в DOM!");
    }
});
