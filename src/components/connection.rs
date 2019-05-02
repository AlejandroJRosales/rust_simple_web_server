use crate::components::routes;
use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 9192];
    stream.read(&mut buffer).unwrap();
    let mut _request: Vec<u8> = Vec::new();
    for c in buffer.iter() {
        if c != &0 {
            _request.push(*c);
        }
    }
    let request = String::from_utf8(_request).unwrap();
    let response = verify(request);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn verify(request: String) -> String {
    let _request = &request.as_bytes();
    if count_string_occurrence(request.clone(), String::from("\r\n\r\n")) != 1
        || String::from("\r\n\r\n") != &request[request.len() - 4..]
    {
        return get_response_status_code(404);
    }

    let lines: Vec<&str> = request.split("\r\n").collect();
    println!("{}", lines[0]);
    let first_line: Vec<&str> = lines[0].split(" ").collect();
    let method = first_line[0];
    let mut file_path = first_line[1];
    let http_version = first_line[2];
    if !is_supported_method(method) {
        return get_response_status_code(405);
    }
    if !is_supported_http_version(http_version) {
        return get_response_status_code(505);
    }
    let does_exist = does_file_exist(file_path);
    if !does_exist.0 && !does_exist.1 {
        return get_response_status_code(404);
    }
    let mut temp_file_path = file_path.to_owned();
    if does_exist.1 {
        for route in &convert_routes(routes::get_routes()) {
            if file_path.to_owned() == route.target {
                temp_file_path = route.source.clone();
                break;
            }
        }
    }
    file_path = temp_file_path.as_str();
    let file = file_path[1..].to_owned();
    let contents = fs::read_to_string(file).unwrap();
    return format!("{}{}\r\n", get_file_header(file_path.to_owned()), contents);
}

fn count_string_occurrence(mut request: String, target: String) -> u32 {
    let mut count: u32 = 0;
    while request.find(&target) != None {
        count = count + 1;
        request = request[request.find(&target).unwrap() + 1..].to_owned();
    }
    return count;
}

fn get_response_status_code(status_code: u32) -> String {
    let header = format_header(status_code);
    // Consider making an actual not found page
    return format!("{}\r\nContent-Type: text/html\r\n\r\n<html><head></head><body><h1>{} {} I need to make a better error page handler</h1></body></html>\r\n", header, status_code, get_status_code_value(status_code));
}

fn format_header(status_code: u32) -> String {
    return format!(
        "HTTP/1.1 {} {}",
        status_code,
        get_status_code_value(status_code)
    );
}

fn get_status_code_value(status_code: u32) -> String {
    let end = match status_code {
        200 => "Ok",
        400 => "Bad Request",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        500 => "Internal Server Error",
        505 => "HTTP Version Not Supported",
        _ => "",
    };
    return end.to_owned();
}

fn get_content_types(content_type: String) -> String {
    let end = match &*content_type {
        "html" | "htm" => "text/html",
        "css" => "text/css",
        "gif" => "image/gif",
        "png" => "image/png",
        "jpg" => "image/jpg",
        "xml" => "application/xml",
        "svg" => "image/svg+xml",
        "txt" => "text/plain",
        _ => "", // Change this so that an error is thrown if an unsupported file format is requested
    };
    return end.to_owned();
}

fn contains_method(method: &str) -> bool {
    let methods = ["GET"];
    if methods.contains(&method) {
        return true;
    } else {
        return false;
    }
}

fn contains_version(version: &str) -> bool {
    let versions = ["HTTP/1.0", "HTTP/1.1"];
    if versions.contains(&version) {
        return true;
    } else {
        return false;
    }
}

fn is_supported_method(method: &str) -> bool {
    return contains_method(method);
}

fn is_supported_http_version(version: &str) -> bool {
    return contains_version(version);
}

fn does_file_exist(file_path: &str) -> (bool, bool) {
    let mut second = false;
    for route in &convert_routes(routes::get_routes()) {
        if file_path.to_owned() == route.target {
            second = true;
        }
    }
    let path = format!(
        "{}{}",
        std::env::current_dir().unwrap().as_path().to_str().unwrap(),
        file_path
    );
    return (Path::new(&path).exists(), second);
}

fn get_file_header(file_path: String) -> String {
    let _extension: Vec<&str> = file_path.split(".").collect();
    let extension = _extension.last().unwrap();
    let content_type = get_content_types(format!("{}", extension));
    return format!(
        "{}\r\nContent-Type: {}\r\n\r\n",
        format_header(200),
        content_type
    );
}

fn convert_routes(
    routes: Vec<(String, String, Vec<(String, String, String)>)>,
) -> Vec<RoutingPath> {
    let mut final_routes = Vec::<RoutingPath>::new();
    for route in &routes {
        for sub_route in &route.2 {
            final_routes.push(RoutingPath::new(
                format!("{}{}", route.0, sub_route.0),
                format!("/src/{}/html/{}", route.1, sub_route.1),
                sub_route.2.to_owned(),
            ));
        }
    }
    return final_routes;
}

pub struct RoutingPath {
    pub target: String,
    pub source: String,
    pub name: String,
    // At some point I need to add the contents to be pushed, this requires database implementation and other stuff tho.
}

impl RoutingPath {
    pub fn new(target: String, source: String, name: String) -> RoutingPath {
        return RoutingPath {
            target: target,
            source: source,
            name: name,
        };
    }
}
