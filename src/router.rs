use crate::request::Request;
use matchit::Params;

// 定义处理程序特征
pub trait Handler {
    fn handle(&self, request: &Request, params: Params) -> String;
}

// 为闭包实现Handler（支持状态捕获）
impl<F> Handler for F
where
    F: Fn(&Request, Params) -> String,
{
    fn handle(&self, request: &Request, params: Params) -> String {
        self(request, params)
    }
}

// 使用Box包装处理程序以实现类型擦除
type BoxedHandler = Box<dyn Handler + Send + Sync>;

// 路由器结构体
pub struct Router {
    // 使用matchit进行路径匹配
    routes: matchit::Router<BoxedHandler>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: matchit::Router::new(),
        }
    }

    // 添加路由路径和处理程序
    pub fn add_route(&mut self, path: &str, handler: impl Handler + Send + Sync + 'static) {
        self.routes
            .insert(path, Box::new(handler))
            .expect("Failed to add route");
    }

    // 处理请求并返回响应
    pub fn handle_request(&self, request: &Request) -> Result<String, String> {
        match self.routes.at(&request.request_line().uri) {
            Ok(matched) => Ok(matched.value.handle(request, matched.params)),
            Err(e) => Err(format!("Route not found: {e}")),
        }
    }
}