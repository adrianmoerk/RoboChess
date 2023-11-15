// create WebSocket connection
const socket = new WebSocket("ws://127.0.0.1:8080/ws");

// called when the connection is established
socket.onopen = function () {
	console.log("WebSocket connection established");
	// used to join desired room, just socket.send("/join <anything>")
	socket.send("/join test");
	socket.send("HELLO WORLD");
};

// called when a message is received from the server
socket.onmessage = function (event) {
	console.log("Message received:", event.data);
};

// called when the connection is closed
socket.onclose = function () {
	console.log("WebSocket connection closed");
};
