use std::io;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::TcpStream;
use tokio::task;

#[tokio::main]
async fn main() -> io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:8080").await?;

    // Create a buffered reader and writer for the stream
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);

    let read = task::spawn(async move {
        // Read from server
        loop {
            let mut server_data = String::new();
            reader.read_line(&mut server_data).await.unwrap();
            println!("{}", server_data);
        }
    });

    let write = task::spawn(async move {
        // Write to server
        loop {
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).unwrap();
            writer.write_all(user_input.as_bytes()).await.unwrap();
            writer.flush().await.unwrap();
        }
    });

    // Wait for all tasks to complete
    let _ = tokio::try_join!(read, write);

    Ok(())
}
