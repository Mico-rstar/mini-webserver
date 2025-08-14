use crate::router::Handler;
use crate::request::Request;
use crate::response::{Response, ResponseBuilder};
use matchit::Params;
use crate::structs::content_type::ContentType;
use crate::structs::body::Body;

use std::fs;

pub struct RootResourcesHandler;

impl Handler for RootResourcesHandler {
    fn handle(&self, request: &Request, _params: Params) -> Result<Response, Box<dyn std::error::Error>> {
        let host = "mini-webserver/localhost";

        // 1. Define the base directory for resources and get its absolute path.
        let base_dir = match fs::canonicalize("./resources") {
            Ok(path) => path,
            Err(_) => {
                eprintln!("Security warning: 'resources' directory not found or inaccessible.");
                return Ok(Response::internal_server_error(host));
            }
        };

        // 2. Construct the full path from the request URI.
        let uri_path = request.request_line().uri.trim_start_matches('/');
        let mut requested_path = base_dir.clone();
        requested_path.push(uri_path);

        // If the path is a directory, default to serving 'index.html'.
        if requested_path.is_dir() {
            requested_path.push("index.html");
        }

        // 3. Canonicalize the path to resolve symlinks, '..', etc. This also checks for existence.
        let final_path = match fs::canonicalize(&requested_path) {
            Ok(path) => path,
            Err(_) => {
                // The standard not_found builder doesn't include the path, so we build it manually.
                return Ok(Response::not_found(host));
            }
        };

        // 4. SECURITY CHECK: Prevent directory traversal attacks.
        // Ensure the resolved path is still within the intended 'resources' directory.
        if !final_path.starts_with(&base_dir) {
            eprintln!("Security alert: Directory traversal attempt blocked for path: {}", uri_path);
            return Ok(Response::forbidden(host));
        }

        // 5. Identify the resource's MIME type based on its file extension.
        let content_type = ContentType::from_mime(final_path.extension().and_then(|s| s.to_str()).unwrap_or_default());

        // 6. Read the resource content from the filesystem.
        let file_content = match fs::read(&final_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading file {:?}: {}", final_path, e);
                return Ok(Response::internal_server_error(host));
            }
        };

        // 7. Build and return the successful response using the builder trait.
        Ok(Response::success(host, content_type, Body::Binary(file_content)))
    }
}
