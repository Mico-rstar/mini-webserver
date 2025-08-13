use std::net::TcpListener;
use std::net::TcpStream;

mod structs;
mod request;



fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                if let Err(e) = handle_connection(&mut stream) {
                    eprintln!("{e}");
                } 
            }
            Err(e) => {
                println!("{e}");
            }
        }
    }
}

fn handle_connection(stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>>{
    let req = request::Request::from_stream(stream)?;
    println!("{:#?}", req.header());
    println!("{:#?}", req.body());
    Ok(())
}
