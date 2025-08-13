use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

use crate::structs::content_type::ContentType;
use crate::structs::status::Status;

mod structs;
mod request;
mod response;



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

    // 构造响应报文
    let res = response::Response::new("mini-webserver/localhost", Status::Ok, ContentType::TEXT);
    stream.write_all(res.as_bytes().as_slice())?;

    println!("{:?}", res);


    Ok(())
}
