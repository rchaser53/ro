#![feature(async_closure)]
use std::{thread, time};

use crossbeam_channel::{bounded, Sender};
use tokio;

fn main() {
    let start = time::Instant::now();
    println!("{:?}", time::Instant::now());

    let (s, r) = bounded(4);
    let length: u64 = 4;
    for num in 0..length {
        let cloned = s.clone();
        thread::spawn(move || {
            let mut rt = tokio::runtime::Runtime::new().unwrap();
            let f = wait_and_send(cloned, num + 1);
            rt.block_on(f);
        });
    }

    for num in r.iter().take(length as usize) {
        println!("{}", num);
    }    

    println!("{:?}", time::Instant::now() - start);
}

async fn wait_and_send(sender: Sender<u64>, i: u64) {
    let val = time::Duration::from_millis(i * 1000);
    thread::sleep(val);

    if let Err(err) = sender.send(i) {
        panic!("{}", err);
    }
}
