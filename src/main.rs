use std::time::Duration;

use robot::ChessTilePosition;
use tokio::io::AsyncWriteExt;
mod chess_tiles;
mod robot;
mod webserver;
use websockets::WebSocket;

#[actix::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::spawn(async {
        let mut roboter_arm = robot::RobotArm::new("192.168.2.40:30002").await.unwrap();
        tokio::time::sleep(Duration::from_secs(5)).await;
        let mut ws = WebSocket::connect("ws://127.0.0.1:8080/ws")
            .await
            .expect("Failed to Connect to Websocket");
        ws.send_text("/join backend".to_string()).await.unwrap();

        loop {
            let gui_command = ws.receive().await.unwrap().as_text().unwrap().0.to_string();
            println!("received command: {:?}", gui_command);
            let gui_command_vec = gui_command.split(" ").collect::<Vec<&str>>();
            let from_field_str = gui_command_vec.get(0).unwrap().to_string();
            let move_operation_str = gui_command_vec.get(1).unwrap().to_string();
            let to_field_str = gui_command_vec.get(2).unwrap().to_string();
            let from_field = ChessTilePosition::position_from_str(from_field_str);
            let to_field = ChessTilePosition::position_from_str(to_field_str);

            match move_operation_str.as_str() {
                "to" => {
                    roboter_arm
                        .move_chesspiece_to_empty_field(&from_field, &to_field)
                        .await
                        .unwrap();
                }
                "hit" => roboter_arm
                    .move_chesspiece_to_occupied_field(&from_field, &to_field)
                    .await
                    .unwrap(),
                _ => {}
            }
        }

        // let a1 = ChessTilePosition::new_position('a', 1);
        // let a8 = ChessTilePosition::new_position('a', 8);
        // let h8 = ChessTilePosition::new_position('h', 8);
        // let h1 = ChessTilePosition::new_position('h', 1);

        // roboter_arm.open_gripper().await.unwrap();

        // tokio::time::sleep(Duration::from_secs(8)).await;
        // roboter_arm.move_to_field(&a1, None, None).await.unwrap();
        // tokio::time::sleep(Duration::from_secs(8)).await;
        // roboter_arm.move_to_field(&a8, None, None).await.unwrap();
        // tokio::time::sleep(Duration::from_secs(8)).await;
        // roboter_arm.move_to_field(&h8, None, None).await.unwrap();
        // tokio::time::sleep(Duration::from_secs(8)).await;
        // roboter_arm.move_to_field(&h1, None, None).await.unwrap();
    });
    webserver::start_webserver().await.unwrap();
    Ok(())
}
