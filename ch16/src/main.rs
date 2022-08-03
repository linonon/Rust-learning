use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("spawned i: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("main i: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    match handle.join() {
        Ok(_) => println!("finished"),
        Err(_) => println!("interrupted"),
    };
}

#[test]
fn move_closuer() {
    let v = vec![1, 2, 3];
    thread::spawn(move || {
        println!("vector: {:?}", v);
    })
    .join()
    .unwrap();
}

#[test]

