use std::fs;
use std::net::{TcpStream};
use std::io::Write;

use crate::lib::headers;
use crate::lib::cstmconfig;

fn validate_request_method(meth: &str) -> Result<(), String> {
    let server_config = cstmconfig::ServerConfig::new_cfg();
    for method in server_config.request_methods {
        if meth == method {
            return Ok(());
        }
    }
    Err(String::from("Invalid request method."))
}
fn validate_route<'a>(route: &'a str, routes: [&str; 3]) -> Result<(), String> {
    for r in routes {
        if r == route {
            return Ok(());
        }
    }
    Err(String::from("Invalid route path."))
}

fn fetch_get_routes() -> [&'static str; 3] {
    [
        "/",
        "/users",
        "/tokens"
    ]
}
fn fetch_post_routes() -> [&'static str; 3] {
    [
        "/",
        "/users",
        "/tokens"
    ]
}

fn build_http_response(buffer: &str) -> Result<(&str,&str,[&str;3],String,String), String> {
    let res_ok : String = format!("{} 200 OK", cstmconfig::BaseConfig::new_cfg().http_protocol);
    let assets_cfg = cstmconfig::AssetsConfig::new_cfg();
    let http_req : Vec<&str>;
    /*
     * use 1st tuple val of buffer, drop the rest as
     * req_method|route|http_proto are always first
     * in HTTP request
     */
    match crate::lib::request::validate_http_request(&buffer) {
        Ok(http_request_parsed) => {
            http_req = http_request_parsed;
        },
        Err(e) => {
            let errmsg = format!("Error validating HTTP request: {}", e);
            println!("{}", errmsg);
            return Err(errmsg);
        }
    }
    //*just for better reading
    let req_method = http_req[0];
    let req_route = http_req[1];

    match validate_request_method(&req_method) {
        Ok(()) => {},
        Err(e) => {
            let errmsg = format!("Error validating request method: {}", e);
            println!("{}", errmsg);
            return Err(errmsg);
        }
    }

    let routes = if req_method == "POST" {
        fetch_post_routes()
    } else {
        fetch_get_routes()
    };

    match validate_route(&req_route, routes) {
        Ok(()) => {},
        Err(e) => {
            let errmsg = format!("Error validating route: {}", e);
            println!("{}", errmsg);
            return Err(errmsg);
        }
    }

    let (status_line, view_file) = if req_route == routes[0] {
        (res_ok, format!("{}page.html", assets_cfg.html_base_path))
    } else if req_route == routes[1] {
        (res_ok, format!("{}users.html", assets_cfg.html_base_path))
    } else if req_route == routes[2] {
        (res_ok, format!("{}tokens.html", assets_cfg.html_base_path))
    } else {
        (res_ok, format!("{}404.html", assets_cfg.html_base_path))
    };

    Ok((req_method, req_route, routes, status_line, view_file))
}

pub fn respond_html(mut stream: &TcpStream, buffer: &str) -> Result<(), String> {
    let mut response_data : String = String::new();
    let mut response : String = String::new();
    let contents_all : String;
    let (req_method, route, routes, status_line, view_file) : (&str,&str,[&str;3],String,String);
    
    match build_http_response(&buffer) {
        Ok((rm, rt, rts, sl, vf)) => {
            req_method = rm;
            route = rt;
            routes = rts;
            status_line = sl;
            view_file = vf;
        },
        Err(e) => {
            let errmsg = format!("Error building response: {}", e);
            println!("{}", errmsg);
            return Err(errmsg);
        }
    }

    match crate::lib::request::process_request(&req_method, &route, &routes) {
        Ok(res_data) => {
            response_data = res_data;
        },
        Err(e) => {
            println!("Error processing request: {}", e);
        }
    }

    match fs::read_to_string(view_file) {
        Ok(contents) => {
            contents_all = format!("{}{}",contents,response_data);
        },
        Err(e) => {
            // file read error
            contents_all = String::from("500 Custom Server Error");
            println!("Error opening content file: {}", e);
        }
    }

    let headers = headers::fetch_headers(contents_all.len());
    /*
     * HTTP text-based protocol basic response format:
     * 
     * {HTTP/1.1 200 OK}\r\n
     * {HEADERS}\r\n
     * {CONTENT}
     */
    response.push_str(format!(
        "{}\r\n{}\r\n{}\r\n{}\r\n{}\r\n{}\r\n{}\r\n{}\r\n\r\n{}",
        status_line,
        headers[0],
        headers[1],
        headers[2],
        headers[3],
        headers[4],
        headers[5],
        headers[6],
        contents_all,
    ).as_str());

    match stream.write(response.as_bytes()) {
        Ok(bytes) => {
            println!("Successfuly written {} bytes to stream.", bytes);
        }, 
        Err(e) => {
            println!("Error writing to stream: {}", e);
        }
    }
    // flush() ensures all data is written on the stream
    match stream.flush() {
        Ok(()) => {}, 
        Err(e) => {
            println!("Error writing to stream: {}", e);
        }
    }
    Ok(())
}
