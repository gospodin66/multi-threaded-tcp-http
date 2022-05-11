// stream.set_nonblocking(true).expect("set_nonblocking call failed");
// let peer_addr = stream.peer_addr().unwrap();
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