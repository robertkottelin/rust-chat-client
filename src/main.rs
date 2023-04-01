use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::TcpStream;
use tokio::task;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:8080").await?;

    // Create a buffered reader and writer for the stream
    let (reader, writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut writer = BufWriter::new(writer);

    // Spawn a new task to read incoming data from the server
    let read_task = task::spawn(async move {
        let mut server_data = String::new();
        loop {
            if reader.read_line(&mut server_data).await.is_err() {
                break;
            }
            if !server_data.is_empty() {
                println!("Server: {}", server_data);
                server_data.clear();
            }
        }
    });

    // Spawn a new task to read user input from the console and send it to the server
    let write_task = task::spawn(async move {
        let mut user_input = String::new();
        loop {
            if io::stdin().read_line(&mut user_input).is_err() {
                break;
            }
            if user_input.trim().eq_ignore_ascii_case("exit") {
                break;
            }
            if writer.write_all(user_input.as_bytes()).await.is_err() {
                break;
            }
            user_input.clear();
        }
    });

    // Wait for both tasks to complete
    let _ = tokio::try_join!(read_task, write_task);

    Ok(())
}
