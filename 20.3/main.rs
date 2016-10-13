use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let data_source = Arc::new(Mutex::new(vec![0]));

    let data = data_source.clone();
    thread::spawn(move || {
        loop {
            let mut data = data.lock().unwrap();
            if data[0] >= 100 {
                break;
            }
            data[0] += 1;
            print!("{}, ", data[0]);
        }
    });


    let data = data_source.clone();
    thread::spawn(move || {
        loop {
            let mut data = data.lock().unwrap();
            if data[0] >= 100 {
                break;
            }
            data[0] += 1;
            println!("{}", data[0]);
        }
    });

    thread::sleep(Duration::from_millis(50));
}
