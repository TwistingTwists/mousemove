// use rdev::{listen, Event, EventType::MouseMove};

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
enum Command {
    Move,
    Still,
    ResetTimer,
}

// park a thread ? using atomics, other shared state methods

fn main() {
    let (tx, rx) = mpsc::channel::<Command>();
    let handle = thread::spawn(move || {
        while let Ok(command) = rx.recv() {
            match command {
                Command::Move => println!("Move Please!"),
                // random_move_loop();
                Command::Still => println!("Stop Moving Please!"),
                Command::ResetTimer => println!("Reset the timer to 5 min!"),
            }
        }
    });
    // handle.join().unwrap();
    loop {
        tx.send(Command::Move).unwrap();
        thread::sleep(Duration::from_millis(300));
        tx.send(Command::ResetTimer).unwrap();
        thread::sleep(Duration::from_millis(300));

        tx.send(Command::Still).unwrap();
        thread::sleep(Duration::from_millis(300));
    }
}
