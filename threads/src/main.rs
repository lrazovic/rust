use std::sync::mpsc; //multiple producer, single consumer
use std::thread;
use std::time::Duration;

struct Audio {
    len: i32,
    name: String,
}

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        //the spawned thread needs to own the transmitting channel
        let vals = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("spawned"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap(); //send method returns a Result<T, E>, so let's unwrap it
            thread::sleep(Duration::from_secs(2));
        }
    });

    for msg in rx {
        println!("Received {}", msg);
    }
    let track = Audio {
        len: 32,
        name: String::from("Name"),
    };
    println!("id {} name {}", track.len, track.name);
}
