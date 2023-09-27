use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("192.168.2.40:30002").await?; // Port 30002 ist für URScript-Befehle
    let move_command = "movej([0, -1.57, 0, -1.57, 0, 0], a=1.2, v=0.3)\n"; // Beispielbefehl
    stream.write_all(move_command.as_bytes()).await?;
    Ok(())
}
// HAT GEKLAPPT
