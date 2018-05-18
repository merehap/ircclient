extern crate bufstream;

use std::io::BufRead;
use std::io::Write;
use std::net::TcpStream;

use bufstream::BufStream;

fn main() {
    
    let mut stream = TcpStream::connect("chat.freenode.net:6667")
        .expect("Couldn't connect to the server.");

    let _ = stream.write(b"USER testtesttest 0 * TestTestTest\n");
    let _ = stream.write(b"NICK testtesttest\n");

    println!("Sent USER and NICK, now trying to receive.");
    let mut buf_stream = BufStream::new(stream);

    loop {
        let mut line = String::new();
        let _ = buf_stream.read_line(&mut line)
            .expect("Failed to read from server!");

        if line.starts_with("PING") {
            let message = &line[4..];
            println!("WE DONE GOT PINGED");
            let _ = buf_stream.write(("PONG".to_owned() + message).as_bytes());
            let _ = buf_stream.flush();
        } else if !line.is_empty() {
            print!("{}", line);
        }
    }
}
