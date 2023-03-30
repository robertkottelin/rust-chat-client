use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

#[tokio::main]
async fn main() {
    let server_addr = "SERVER_IP:8080"; // Replace SERVER_IP with your server's IP address
    let mut stream = TcpStream::connect(server_addr).await.unwrap();

    let (mut reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);

    // Spawn a task to read incoming messages from the server
    tokio::spawn(async move {
        let mut buffer = String::new();
        loop {
            match reader.read_line(&mut buffer).await {
                Ok(bytes) if bytes > 0 => {
                    println!("{}", buffer.trim());
                    buffer.clear();
                }
                Ok(_) => break, // EOF or the server closed the connection
                Err(e) => {
                    eprintln!("Error reading from server: {}", e);
                    break;
                }
            }
        }
    });

    // Read user input and send it to the server
    let mut input = String::new();
    while let Ok(_) = std::io::stdin().read_line(&mut input) {
        if let Err(e) = writer.write_all(input.as_bytes()).await {
            eprintln!("Error sending message: {}", e);
            break;
        }
        input.clear();
    }
}
