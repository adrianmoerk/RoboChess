use std::time::Duration;

use robot::ChessTilePosition;
mod chess_tiles;
mod robot;
mod webserver;
#[actix::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::spawn(async {
        let mut roboter_arm = robot::RobotArm::new("192.168.2.40:30002").await.unwrap();
        let a1 = ChessTilePosition::new_position('a', 1);
        let a8 = ChessTilePosition::new_position('a', 8);
        let h8 = ChessTilePosition::new_position('h', 8);
        let h1 = ChessTilePosition::new_position('h', 1);
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
