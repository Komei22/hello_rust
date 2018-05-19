use std::thread;
use std::time::Duration;

struct Philosopher {
    name: String,
}

impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
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

    for p in &philosophers {
        p.eat();
    }
}
