mod robot;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut roboter_arm = robot::RobotArm::new("192.168.2.40:30002").await.unwrap();
    roboter_arm.movel(1.0, 1.0, 1.0, 0.0, 0.0, 0.0, None, None).await.unwrap();
    Ok(())
}
