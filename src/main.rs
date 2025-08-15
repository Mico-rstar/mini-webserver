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
mod api;

use crate::router::Router;
use crate::api::test_api::TestAPI;


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
                if let Err(e) = test_router(&mut stream) {
                    eprintln!("{e}");
                } 
            }
            Err(e) => {
                println!("{e}");
            }
        }
    }
}

fn test_router(stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let req = request::Request::from_stream(stream)?;

    info!("Connection from {}: {}", req.header().get("Host").unwrap_or(&"unknown host".to_string()), req.request_line());

    let mut router = Router::new();
    router.add_route("/api/test", TestAPI);

    let res = router.handle_request(&req)?;
    stream.write_all(res.as_bytes().as_slice())?;

    Ok(())
}
