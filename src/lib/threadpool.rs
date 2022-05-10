use std::io::prelude::*;
use std::net::{TcpStream,TcpListener};
use std::sync::mpsc::{Sender};
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::mpsc;
// use std::time::Duration;
use std::thread;
use crate::lib::thrstdin;
use crate::lib::request;
use crate::lib::response;
use crate::lib::cstmfiles;
use crate::lib::cstmconfig;
use chrono::{Local};

/*
 * 1. The ThreadPool will create a channel and hold on to the sending side of the channel.
 * 2. Each Worker will hold on to the receiving side of the channel.
 * 3. We’ll create a new Job struct that will hold the closures we want to send down the channel.
 * 4. The execute method will send the job it wants to execute down the sending side of the channel.
 * 5. In its thread, the Worker will loop over its receiving side of the channel and execute the closures of any jobs it receives.
 */
static THREAD_LIMIT : usize = 10;
type Job = Box<dyn FnOnce() + Send + 'static>;

#[allow(dead_code)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    tx: Sender<Job>
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (tx,rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&rx)));
        }
        ThreadPool { workers, tx }
    }

    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        self.tx.send(job).unwrap();
    }

}

/*
 * Worker is responsible for taking jobs and exec them 
 */
#[allow(dead_code)]
pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thr = thread::spawn(move || loop {
            // retrieve job from channel
            let job = rx.lock().unwrap().recv().unwrap();
            println!("----------------------");
            println!("Worker {} got a new job!", id);
            job();
        });
        Worker {
            id: id,
            thread: thr,
        }
    }
}

pub fn handle_in_threadpool(listener: &TcpListener) -> Result<(), String> {
    let pool = ThreadPool::new(THREAD_LIMIT);
    // let mut cfg = cstmconfig::ServerConfig::new_cfg();

    for s in listener.incoming() {
        match s {
            Ok(stream) => {
                // stream.set_nonblocking(true).expect("set_nonblocking call failed");
                // let peer_addr = stream.peer_addr().unwrap();
                let stream_clone = stream.try_clone().expect("clone-stream failed...");
                // let (tx,rx) : (Sender<TcpStream>, Receiver<TcpStream>) = mpsc::channel();
                // let mut recievers = Vec::with_capacity(10_usize);
                // let mutex_tx = Arc::new(Mutex::new(tx));
                // let mutex_rx = Arc::new(Mutex::new(rx));

                // for _i in 0..10 {
                //     recievers.push(Arc::clone(&mutex_rx));
                // }
                // let mutex_rx = Arc::new(Mutex::new(rx));
                // let mut cfg_rec = cstmconfig::ServerConfig::new_cfg();
                //cfg.connections.push(stream);
    

                // crossbeamthread::scope(|scope| {
                //     let thr = scope.spawn(|_| {
                //         let mtx = mutex_tx.lock().unwrap();    
                //         println!("\r\n\r\nConnections sent to main thread: ");
                //         for conn in cfg.connections.iter() {
                //             println!("{:?}", conn);
                //         }
                //         mtx.send(cfg.connections).unwrap();
                //         std::thread::sleep(Duration::from_secs(1));
                //     });
                //     thr.join().unwrap();
                // }).unwrap();
                
                // let mrx = mutex_rx.lock().unwrap();
                // let recv = mrx.recv().unwrap();
                // cfg_rec.connections = recv;
                // for conn in cfg.connections.iter() {
                //     println!("{:?}", conn);
                // }


                pool.execute(|| {
                    match handle_connection(stream_clone) {
                        Ok(()) => {},
                        Err(e) => {
                            println!("Error on connection handler: {}", e);
                        }
                    }
                });
            },
            Err(e) => {
                let errmsg = format!("Error on creating stream: {}", e);
                println!("{}", errmsg);
                return Err(errmsg);
            }
        }
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<(), String> {
    let mut buffer = [0; 4096];
    let fpath = cstmconfig::AssetsConfig::new_cfg().log_path;
    let stream_clone = stream.try_clone().unwrap();

    let channel = thrstdin::Thr::new_thr();

    println!("spawning send stream thread..");
    thrstdin::Thr::send_stream(&channel, stream_clone);

    println!("spawning listen channel thread..");
    let new_stream = thrstdin::Thr::listen_channel(channel.rx);

    println!("New stream: {:?}", new_stream);

    match stream.read(&mut buffer) {
        Ok(bytes) => {
            /*
             * String::from_utf8_lossy() converts the bytes in the buffer to a string.
             * Function takes a &[u8] and produces a String from it. The “lossy” part
             * of the name indicates the behavior of this function when it sees an
             * invalid UTF-8 sequence: it will replace the invalid sequence
             * with �, the U+FFFD REPLACEMENT CHARACTER
             */
            let recv = String::from_utf8_lossy(&buffer[..]);
            let data = recv.trim_matches(char::from(0));
            let msg = format!("\n[{}]: received [{} bytes]:\r\n{}", Local::now().to_rfc3339(), bytes, &data);
            println!("{}", msg);
            match cstmfiles::write(&fpath, msg) {
                Ok(()) => {}
                Err(err) => {
                    println!("Oops! Error writing to log! {:?}", err);
                }
            }
            /*******Response*******/
            match request::validate_http_request(&data) {
                Ok(_http_request) => {
                    match response::respond_html(&stream, &data) {
                        Ok(()) => {}, 
                        Err(e) => {
                            println!("Error sending html response: {}", e);
                        }
                    }
                }, 
                _ => {
                    // default tcp connection
                }
            }
        },
        Err(e) => {
            let errmsg = format!("Error recieving data: {}", e);
            println!("{}", &errmsg);
            return Err(errmsg);
        }
    }
    println!("----------------------");
    Ok(())
}