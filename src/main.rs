use std::io::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    let mut serverinput = stream.read(&mut [0; 128])?;

    println!("{:?}", serverinput);

    let stdin = stdin();

    let mut userinput = String::new();
    stdin.lock().read_line(&mut userinput)?;

    stream.write(userinput.as_bytes())?;
    
    Ok(())
}