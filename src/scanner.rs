
use std::fmt::format;
use std::net::{TcpStream, SocketAddr};
use std::time::Duration;
use std::thread;
use std::sync::mpsc;
use rayon::prelude::*;

// not needed as im calling mpsc above
// nope i actually need this
use std::sync::mpsc::Sender;


pub fn run_scan(target: String, tx: Sender<String>) {

    //let target = "127.0.0.1";
    let start_port = 1;
    let end_port = 1024;    
    // increased timeout to 100
    let timeout = Duration::from_millis(100);

    // print this to the console?
    //println!("Scanning {} on ports {}-{}", target, start_port, end_port);

    // NOT using this as we are sending our tx from my_window.rs
    // channel to communicate between the threads we open
    //let (tx, rx) = mpsc::channel::<String>(); // changed from mpsc::channel() to the String equivalent 

    //replacing "for loop" with a thread pool
    (1..=1024).into_par_iter().for_each(|port| {

        let tx = tx.clone();

        let target = target.to_string();

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