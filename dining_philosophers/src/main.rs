use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

struct Table {
    forks: Vec<Mutex<()>>,
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self) {
        println!("{} is eating.", self.name);

        thread::sleep(Duration::from_millis(1000));

        println!("{} is done eating!", self.name);
    }
}


fn main() {
    let philosophers = vec![
        Philosopher::new("John"),
        Philosopher::new("Andy"),
        Philosopher::new("Karl"),
        Philosopher::new("Emma"),
        Philosopher::new("Michel"),
    ];

    // シングルスレッド処理　哲学者は一人ずつ食事をする
    // for p in &philosophers {
    //     p.eat();
    // }

    // マルチスレッド処理 哲学者は複数人で食事をする
    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        thread::spawn(move||{
            p.eat();
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
