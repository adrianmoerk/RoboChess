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
        // test_field.convert_pos_to_coords();
        a8.convert_pos_to_coords();
        h8.convert_pos_to_coords();
        h1.convert_pos_to_coords();
        roboter_arm
            .move_to_field(&a1, None, None)
            .await
            .unwrap();
        // tokio::time::sleep(Duration::from_secs(5)).await;
        // roboter_arm.movel(-0.27125, -0.831, 0.199742457810, 1.211315540742, 2.895724392903, 0.011966278988, None,None).await.unwrap();
        tokio::time::sleep(Duration::from_secs(8)).await;
        roboter_arm
            .move_to_field(&a8, None, None)
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_secs(8)).await;
        roboter_arm
            .move_to_field(&h8, None, None)
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_secs(8)).await;
        roboter_arm
            .move_to_field(&h1, None, None)
            .await
            .unwrap();
        // let mut roboter_arm = robot::RobotArm::new("ur-2018300294:30002").await.unwrap();
        // let mut roboter_arm = robot::RobotArm::new("129.143.38.67:30002").await.unwrap();


        // let command = String::from("get_tcp_offset()");
        // roboter_arm
        //     .stream
        //     .write_all(command.as_bytes())
        //     .await
        //     .unwrap();
        // let mut buf = String::new();
        // roboter_arm.stream.read_to_string(&mut buf).await.unwrap();
        // println!("{}", buf);

        
        // roboter_arm
        //     .movel(-245.85, -857.72, 199.7, 1.211, 2.896, 0.012, None, None)
        //     .await
        //     .unwrap();
        
    });
    webserver::start_webserver().await.unwrap();
    Ok(())
}
// HAT GEKLAPPT
