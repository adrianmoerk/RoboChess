//! Example on how to easily connect to and use the webserver
use tokio::time::Duration;
use websockets::WebSocket;
pub async fn initialize_websocket() -> WebSocket {
    tokio::time::sleep(Duration::from_secs(5)).await;
    let mut ws = WebSocket::connect("ws://127.0.0.1:8080/ws")
        .await
        .expect("Failed to Connect to Websocket");
    ws.send_text("/join test".to_string()).await.unwrap();
    loop {
        println!("received{:?}", ws.receive().await.unwrap());
    }
}
