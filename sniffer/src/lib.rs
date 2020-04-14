use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::sync::mpsc::{channel, Sender};
use std::thread;

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16, lower: u16, upper: u16) {
    let mut port: u16 = start_port + lower;

    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                println!("Port {} is open", port);
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        };

        if (upper - port) <= num_threads {
            break;
        }
        port += num_threads;
    }
}

pub fn sniff(ip: IpAddr, threads: u16, lower: u16, upper: u16) {
    let (tx, rx) = channel();

    println!("Starting port enumeration on {}", ip);
    println!("Start port: {}", lower);
    println!("End port: {}", upper);
    println!("");

    for i in 0..threads {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, ip, threads, lower, upper);
        });
    }

    let mut out = vec![];

    drop(tx);
    for p in rx {
        out.push(p);
    }

    println!("");
    println!("Port enumeration successful!");
    println!("Found {} ports", out.len());
}
