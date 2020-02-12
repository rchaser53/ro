#![feature(async_closure)]
use std::{thread, time};

use crossbeam_channel::{bounded, Sender};
fn main() {
    let start = time::Instant::now();
    println!("{:?}", time::Instant::now());

    let (s, r) = bounded(4);
    let length: u64 = 4;
    for num in 0..length {
        let cloned = s.clone();
        thread::spawn(move || wait_and_send(cloned, num + 1));
    }

    for num in r.iter().take(length as usize) {
        println!("{}", num);
    }

    println!("{:?}", time::Instant::now() - start);
}

fn wait_and_send(sender: Sender<u64>, i: u64) {
    let val = time::Duration::from_millis(i * 1000);
    thread::sleep(val);

    let _ = sender.send(i);
}
