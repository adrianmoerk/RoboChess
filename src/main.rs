use robot::ChessTilePosition;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

mod robot;
mod webserver;
#[actix::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::spawn(async {
        let test_field = ChessTilePosition::new_position('c', 3);
        test_field.convert_pos_to_coords();
        // let mut roboter_arm = robot::RobotArm::new("192.168.2.40:30002").await.unwrap();
        // let mut roboter_arm = robot::RobotArm::new("ur-2018300294:30002").await.unwrap();
        let mut roboter_arm = robot::RobotArm::new("129.143.38.67:30002").await.unwrap();

        let command = String::from("get_tcp_offset()");
        roboter_arm
            .stream
            .write_all(command.as_bytes())
            .await
            .unwrap();
        let mut buf = String::new();
        roboter_arm.stream.read_to_string(&mut buf).await.unwrap();
        println!("{}", buf);

        roboter_arm
            .movel(-245.85, -857.72, 199.7, 1.211, 2.896, 0.012, None, None)
            .await
            .unwrap();
        roboter_arm
            .move_to_field(&test_field, None, None)
            .await
            .unwrap();
    });
    webserver::start_webserver().await.unwrap();
    Ok(())
}
