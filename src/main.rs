use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

use tracing::{info, error, Level};
use tracing_subscriber::FmtSubscriber;

mod structs;
mod request;
mod response;
mod router;
mod handler;

use crate::router::Router;
use crate::handler::api_handler::test_api::TestAPI;
use crate::handler::static_handler::root_resources::RootResourcesHandler;


fn main() {

     // 设置日志订阅者
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)
        .expect("设置默认日志订阅者失败");

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // 初始化router
    let router = router_init();
    
    // 等待客户端连接
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                if let Err(e) = handle_connection(&mut stream, &router) {
                    eprintln!("{e}");
                } 
            }
            Err(e) => {
                error!("{e}");
            }
        }
    }
}

fn router_init() -> Router {
    let mut router = Router::new();
    // API routes should be registered before the catch-all static handler.
    router.add_route("/api/test", TestAPI);

    // Add the static file handler as a catch-all route.
    router.add_route("/*path", RootResourcesHandler);
    router
}



fn handle_connection(stream: &mut TcpStream, router: &Router) -> Result<(), Box<dyn std::error::Error>> {
    let req = request::Request::from_stream(stream)?;

    info!("Connection from {}: {}", req.header().get("Host").unwrap_or(&"unknown host".to_string()), req.request_line());

    let res = router.handle_request(&req)?;
    stream.write_all(res.as_bytes().as_slice())?;

    Ok(())
}
