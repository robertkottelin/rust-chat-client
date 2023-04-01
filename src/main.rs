use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::TcpStream;
use tokio::task;
use tokio::sync::mpsc;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:8080").await?;

    // Create a buffered reader and writer for the stream
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);

    // Create an async channel for communication between tasks
    let (tx, mut rx) = mpsc::channel(1);

    // Spawn a new task to read incoming data from the server
    let read_task = task::spawn(async move {
        let mut server_data = String::new();
        loop {
            if reader.read_line(&mut server_data).await.is_err() {
                break;
            }
            println!("Server: {}", server_data);
            server_data.clear();
        }
    });

    let write_task = task::spawn(async move {
        let mut user_input = String::new();
        loop {
            if io::stdin().read_line(&mut user_input).is_err() {
                break;
            }
            let formatted_input = format!("{}\n", user_input.trim()); // Add a newline character
            if let Err(_) = tx.send(formatted_input.clone()).await {
                break;
            }
            user_input.clear();
        }
    });    

    let forward_task = task::spawn(async move {
        while let Some(user_input) = rx.recv().await {
            if user_input.trim().eq_ignore_ascii_case("exit") {
                break;
            }
            if writer.write_all(user_input.as_bytes()).await.is_err() {
                break;
            }
            writer.flush().await.unwrap(); // Flush the write buffer
        }
    });
    

    // Wait for all tasks to complete
    let _ = tokio::try_join!(read_task, write_task, forward_task);

    Ok(())
}
