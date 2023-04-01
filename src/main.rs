use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::net::TcpStream;
use std::io::stdin;

fn main() -> std::io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:8080")?;

    // Create a buffered reader and writer for the stream
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    loop {
        // Read incoming data from the server
        let mut server_data = String::new();
        reader.read_line(&mut server_data)?;
        println!("Server: {}", server_data);

        // Read user input from the console
        let mut user_input = String::new();
        stdin().read_line(&mut user_input)?;

        // If the user types "exit", close the connection and exit the program
        if user_input.trim().eq_ignore_ascii_case("exit") {
            break;
        }

        // Send user input back to the server
        writer.write_all(user_input.as_bytes())?;
        writer.flush()?;
    }

    Ok(())
}
