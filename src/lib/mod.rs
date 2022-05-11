/*
 * We bring std::io::prelude into scope to get access to certain
 * traits that let us read from and write to the stream
 */
use std::net::{TcpListener,SocketAddr};

mod threadpool;
mod thrstdin;
mod helpers;
mod database;
mod cstmconfig;
mod cstmfiles;
mod headers;
mod request;
mod response;

pub fn server() -> Result<(), String>{
    let cfg: cstmconfig::ServerConfig = cstmconfig::ServerConfig::new_cfg();
    let port1 : u16 = cfg.port1;
    let port2 : u16 = cfg.port2;
    let fpath = cstmconfig::AssetsConfig::new_cfg().log_path;
    /*
     *  convert ip address from .env file: String => Vec<&str> => Vec<u8> => [u8; 4]
     */
    let ip_str : Vec<&str> = cfg.host.as_str().split('.').collect();
    let ip_vec : Vec<u8> = ip_str.into_iter().map(|val| val.parse::<u8>().unwrap()).collect();
    let ip : [u8; 4] = helpers::vec_to_arr(ip_vec);
    let addrs = [
        SocketAddr::from((ip, port1)),
        SocketAddr::from((ip, port2)),
    ];
    match cstmfiles::create(&fpath) {
        Ok(()) => { 
            println!("Successfuly created log file!");
        }
        Err(_err) => {}
    }
    match init_server(&addrs) {
        Ok(listener) => {
            println!("Server listening for connections..");
            match listen_for_connections(&listener) {
                Ok(()) => {
                    Ok(())
                },
                Err(e) => {
                    let errmsg = format!("Error on listener: {}", e);
                    println!("{}", &errmsg);
                    return Err(errmsg);
                }
            }
        }, 
        Err(e) => {
            let errmsg = format!("Error initializing server: {}", e);
            println!("{}", &errmsg);
            return Err(errmsg);
        }
    }
}

fn init_server(ip_port: &[SocketAddr; 2]) -> Result<TcpListener, String>{
    match TcpListener::bind(format!("{}", ip_port[0])) {
        Ok(listener) => {
            Ok(listener)
        },
        _ => {
            println!("Error on bind().. trying another ip:port pair..");
            match TcpListener::bind(format!("{}", ip_port[1])) {
                Ok(listener) => {
                    Ok(listener)
                },
                _ => {
                    let msg = "Error on bind() on fallback ip:port pair.";
                    println!("{}", msg);
                    return Err(String::from(msg));
                }
            }
        }
    }
}

fn listen_for_connections(listener: &TcpListener) -> Result<(), String> {
    match threadpool::handle_in_threadpool(&listener) {
        Ok(()) => {
            println!("Worker finsihed the job successfuly.");
        },
        Err(e) => {
            let errmsg = format!("Error on threadpool handler: {}", e);
            println!("{}", errmsg);
        }
    }
    Ok(())
}