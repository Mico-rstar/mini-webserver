use crate::router::Handler;
use crate::request::Request;
use crate::response::Response;
use matchit::Params;
use crate::structs::status::Status;
use crate::structs::content_type::ContentType;
use crate::structs::body::Body;

pub struct TestAPI;

impl Handler for TestAPI {
    fn handle(&self, request: &Request, params: Params) -> Result<Response, Box<dyn std::error::Error>> {
        let mut res = Response::new("mini-webserver/localhost", Status::Ok, ContentType::TEXT);
        let body_content = format!("Hello from TestAPI! Request URI: {}, Params: {:?}", request.request_line().uri, params);
        res.set_body(Body::Text(body_content));
        Ok(res)
    }
}
