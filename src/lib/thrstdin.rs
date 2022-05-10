use std::io;
use std::io::prelude::*;
use std::net::{TcpStream};
use std::thread;
use std::sync::mpsc;
use chrono::{Local};
use crate::lib::cstmfiles;
use crate::lib::cstmconfig;
// use std::time::Duration;
use std::sync::Mutex;
use std::sync::Arc;


pub struct Thr {
    pub tx: Arc<Mutex<mpsc::Sender<TcpStream>>>,
    pub rx: Arc<Mutex<mpsc::Receiver<TcpStream>>>
}

impl Thr {
    pub fn new_thr() -> Thr {
        let (tx, rx) = mpsc::channel();
        Thr {
            tx:  Arc::new(Mutex::new(tx)),
            rx: Arc::new(Mutex::new(rx)),
        }
    }

    pub fn listen_channel(rx: Arc<Mutex<mpsc::Receiver<TcpStream>>>) -> Result<TcpStream, String> {
        // let mut d : TcpStream;

        let (gtx, grx) : (mpsc::Sender<TcpStream>, mpsc::Receiver<TcpStream>) = mpsc::channel();

        let tx_clone = gtx.clone();
        
        thread::spawn(move || {
            let data = rx.lock().unwrap().recv().unwrap();
            println!("RECIEVED DATA: {:?}", &data);
            // d = data.try_clone().unwrap();
            tx_clone.send(data).unwrap();
            // std::thread::sleep(Duration::from_secs(1));
        });

        let data = grx.recv().unwrap();
        Ok(data)

    }


    pub fn send_stream(&self, stream: TcpStream) {
        use crossbeam::thread as crossbeamthread;
        crossbeamthread::scope(|scope| {
            let thr = scope.spawn(|_| {
                let mutex_tx = Arc::new(Mutex::new(&self.tx));
                let mtx = mutex_tx.lock().unwrap();
                println!("Connections sent to other thread!");
                mtx.lock().unwrap().send(stream).unwrap();
                // std::thread::sleep(Duration::from_secs(1));
            });
            thr.join().unwrap();
        }).unwrap();
    }


}




#[allow(dead_code)]
pub fn init_thread() -> Result<(), String> {
    let fpath = cstmconfig::AssetsConfig::new_cfg().log_path;
    match cstmfiles::create(&fpath) {
        Ok(()) => { 
            println!("File created successfuly.");
        }
        Err(_e) => {}
    }
    let streams = cstmconfig::ServerConfig::new_cfg();
    let loopwrite = loop_user_stdin(streams.connections, fpath);
    match loopwrite {
        Ok(()) => {
            println!("Write thread finished successfuly.");
            Ok(())
        }
        Err(e) => {
            let errmsg = format!("Write thread error: {}", e);
            println!("{}", errmsg);
            return Err(errmsg);
        }
    }
}

pub fn loop_user_stdin(streams: Vec<TcpStream>, fpath: String) -> Result<(), String> {
    // let mut response = String::new();
    let mut contents = String::new();
    /*
     * Using scopes guarantees to terminate before the scope exits,
     * allowing it to reference variables outside the scope.
     */
    thread::spawn(move || {
        loop {
            io::stdin()
                .read_line(&mut contents)
                .expect("Error sending msg!");

            let response = format!("{}", contents.trim());



            // STVAR JE U TOME KAJ JE STREAMS PRAZAN DOK SE PROSLJEDI
            // TREBALO BI KORISTITI DODATNI THREAD KOJI BUDE SLUSAL NOVE KONEKCIJE
            // DODATNI THREAD PROSLJEĐUJE (TX,RX) UPDATEANU LISTU KONEKCIJA KOJA SE VRTI U LOOPU



            for mut s in streams.iter() {

                println!("Writing to stream {:?}", s);

                match s.write(response.as_bytes()) {
                    Ok(bytes) => {
                        let msg = format!("[{}]: successfuly written {} bytes to stream.", Local::now().to_rfc3339(), bytes);
                        match cstmfiles::write(&fpath, msg) {
                            Ok(()) => {}
                            Err(err) => {
                                println!("Oops! Error writing to log! {:?}", err);
                            }
                        }
                        println!("Written data to {:?}", s);
                    }, 
                    Err(e) => {
                        let errmsg = format!("Error writing to stream: {}", e);
                        println!("{}", errmsg);
                    }
                }
                match s.flush() {
                    Ok(()) => {}, 
                    Err(e) => {
                        let errmsg = format!("Flush error on writing to stream: {}", e);
                        println!("{}", errmsg);
                    }
                }
            }
        }
    });
    // thr.join().unwrap();
    Ok(())
}