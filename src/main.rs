use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

use crate::structs::body::Body;
use crate::structs::content_type::ContentType;
use crate::structs::status::Status;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod structs;
mod request;
mod response;
mod router;



fn main() {

     // 设置日志订阅者
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)
        .expect("设置默认日志订阅者失败");

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

    info!("Connection from {}: {}", req.header().get("Host").unwrap_or(&"unknown host".to_string()), req.request_line());

    // 构造响应报文
    let mut res = response::Response::new("mini-webserver/localhost", Status::Ok, ContentType::TEXT);
    res.set_body(Body::Text("hello world".to_string()));
    stream.write_all(res.as_bytes().as_slice())?;

    Ok(())
}
