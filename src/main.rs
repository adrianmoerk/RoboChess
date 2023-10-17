use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
mod robot;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut robot =robot::RobotArm::new("192.168.2.40:30002").await.unwrap(); 
    // robot.movej(0.0,0.5,0.0,0.0,0.0,0.0,Some(0.5),Some(0.2)).await.unwrap();
    robot.movep(135.0, -235.3, 984.5, 0.174, -3.446, 2.407, Some(1.2), Some(0.3)).await.unwrap();
    // let mut stream = TcpStream::connect("192.168.2.40:30002").await?; // Port 30002 ist für URScript-Befehle
    // let move_command = "movej([0, -2.57, 0, -1.57, 0, 0], a=1.2, v=0.3)\n"; // Beispielbefehl
    // stream.write_all(move_command.as_bytes()).await?;
    Ok(())
}
// HAT GEKLAPPT
