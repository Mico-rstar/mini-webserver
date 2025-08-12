use httparse::{EMPTY_HEADER, Request};
use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                handle_connection(&mut stream);
            }
            Err(e) => {
                println!("{e}");
            }
        }
    }
}

fn handle_connection(stream: &mut TcpStream) {
    let buf_reader = BufReader::new(stream);
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     //.take_while(|line| !line.is_empty())
    //     .collect();

    let mut lines = buf_reader.lines();

    // 收集 header（直到第一个空行）
    let header: HashMap<String, String> = lines
        .by_ref()
        .map_while(Result::ok)
        .take_while(|line| !line.is_empty())
        .filter_map(|line| {
            let mut parts = line.splitn(2, ':');
            match (parts.next(), parts.next()) {
                // 移除键的前后空白和值的前导空白
                (Some(key), Some(value)) => {
                    Some((key.trim().to_string(), value.trim_start().to_string()))
                }
                _ => None, // 忽略无效行
            }
        })
        .collect();

    // 剩余部分作为 body
    let body = lines.map_while(Result::ok).collect::<Vec<_>>().join("\n");

    println!("Header: {:#?}", header);
    println!("Body: {body}");
}
