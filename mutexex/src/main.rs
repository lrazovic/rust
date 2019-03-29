use std::sync::{Mutex, Arc};

fn main(){
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for i in 0..10 {
        let handle = thread::spawn(move || {
            
        });
    }

}