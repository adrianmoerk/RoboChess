use robot::ChessTilePosition;

mod robot;
mod webserver;
#[actix::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::spawn(async {
        let test_field = ChessTilePosition::new_position('c', 3);
        let mut roboter_arm = robot::RobotArm::new("192.168.2.40:30002").await.unwrap();
        roboter_arm.movel(-245.85, -857.72, 199.7, 1.211, 2.896, 0.012, None, None).await.unwrap();

        roboter_arm.move_to_field(&test_field, None, None).await.unwrap();
    });
    webserver::start_webserver().await.unwrap();
    Ok(())
}
