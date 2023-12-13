use std::time::Duration;

use robot::ChessTilePosition;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

mod robot;
mod webserver;
#[actix::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::spawn(async {
        let a1 = ChessTilePosition::new_position('a', 1);
        a1.convert_pos_to_coords();
        let mut roboter_arm = robot::RobotArm::new("192.168.2.40:30002").await.unwrap();
        let a8 = ChessTilePosition::new_position('a', 8);
        let h8 = ChessTilePosition::new_position('h', 8);
        let h1 = ChessTilePosition::new_position('h', 1);
        a8.convert_pos_to_coords();
        h8.convert_pos_to_coords();
        h1.convert_pos_to_coords();
        roboter_arm.move_to_field(&a1, None, None).await.unwrap();
        tokio::time::sleep(Duration::from_secs(8)).await;
        roboter_arm.move_to_field(&a8, None, None).await.unwrap();
        tokio::time::sleep(Duration::from_secs(8)).await;
        roboter_arm.move_to_field(&h8, None, None).await.unwrap();
        tokio::time::sleep(Duration::from_secs(8)).await;
        roboter_arm.move_to_field(&h1, None, None).await.unwrap();
    });
    webserver::start_webserver().await.unwrap();
    Ok(())
}
// HAT GEKLAPPT
