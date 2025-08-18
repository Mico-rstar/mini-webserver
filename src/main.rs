use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::Arc;

use tracing::{Level, error, info};
use tracing_subscriber::FmtSubscriber;

mod handler;
mod request;
mod response;
mod router;
mod structs;
mod threads;

use crate::handler::api_handler::test_api::TestAPI;
use crate::handler::static_handler::root_resources::RootResourcesHandler;
use crate::router::Router;
use crate::threads::thread_pool::ThreadPool;

fn main() {
    // 设置日志订阅者
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("设置默认日志订阅者失败");

    // 创建线程池
    let pool = ThreadPool::new(8);

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // 初始化router
    let router = Arc::new(router_init());

    // 等待客户端连接
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let rp = router.clone();
                pool.execute(move || {
                    if let Err(e) = handle_connection(stream, rp) {
                        error!("{e}");
                    }
                })
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

fn handle_connection(
    mut stream: TcpStream,
    router: Arc<Router>,
) -> Result<(), Box<dyn std::error::Error>> {
    let req = request::Request::from_stream(&mut stream)?;

    info!(
        "Connection from {}: {}",
        req.header()
            .get("Host")
            .unwrap_or(&"unknown host".to_string()),
        req.request_line()
    );

    let res = router.handle_request(&req)?;
    stream.write_all(res.as_bytes().as_slice())?;

    Ok(())
}
