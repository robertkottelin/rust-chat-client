use std::io;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::task;

#[tokio::main]
async fn main() -> io::Result<()> {
    let stream = TcpStream::connect("localhost:8080").await?;

    // Create a buffered reader and writer for the stream
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);

    let read = task::spawn(async move {
        // Read from server
        loop {
            let mut server_data = String::new();
            match reader.read_line(&mut server_data).await {
                Ok(n) if n == 0 => continue, // empty line, continue loop
                Ok(_) => println!("{}", server_data), // line read successfully
                Err(e) => panic!("Error reading from server: {}", e), // error reading from server
            }
        }
    });

    let write = task::spawn(async move {
        // Write to server
        loop {
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).unwrap();
    
            match writer.write_all(user_input.as_bytes()).await {
                Ok(_) => {
                    if let Err(e) = writer.flush().await {
                        println!("Error flushing writer: {}", e);
                        break; // exit loop if there is an error flushing the writer
                    }
                },
                Err(e) => {
                    println!("Error writing to server: {}", e);
                    break; // exit loop if there is an error writing to the server
                }
            }
        }
    });
    
    // Wait for all tasks to complete
    let _ = tokio::try_join!(read, write);

    Ok(())
}
