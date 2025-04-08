
use std::fmt::format;
use std::net::{TcpStream, SocketAddr};
use std::time::Duration;
use std::thread;
use std::sync::mpsc;

// not needed as im calling mpsc above
// nope i actually need this
use std::sync::mpsc::Sender;


pub fn run_scan(target: String, tx: Sender<String>) {

    //let target = "127.0.0.1";
    let start_port = 1;
    let end_port = 1024;    
    let timeout = Duration::from_millis(10);

    // print this to the console?
    //println!("Scanning {} on ports {}-{}", target, start_port, end_port);

    // NOT using this as we are sending our tx from my_window.rs
    // channel to communicate between the threads we open
    //let (tx, rx) = mpsc::channel::<String>(); // changed from mpsc::channel() to the String equivalent 

    for port in start_port..=end_port {

        let tx = tx.clone();

        let target = target.to_string();

        thread::spawn(move || {
           let address = format!("{}:{}", target, port);
           if let Ok(socket) = address.parse::<SocketAddr>() {

               if TcpStream::connect_timeout(&socket, timeout).is_ok() {
                    let msg = format!("{}: open\r\n", port);
                    // DEBUG
                    //println!("{}",msg);
                    let _ = tx.send(msg);
               }

           }

        });
    
    }

}