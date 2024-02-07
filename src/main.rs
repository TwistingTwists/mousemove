#[allow(unused_mut)]
mod random_move;
use random_move::random_mouse_move;
use rdev::{listen, Event, EventType};
use tokio::sync::mpsc;
use tokio::task::spawn_blocking;
use tokio::time::Duration;

// runtime = exectuor + reactor
// futures = executor + _______  = 1/2 runtime
// https://github.com/richardanaya/executor
// exectuor = only executor

#[derive(Debug)]
enum Command {
    MoveMouse,
    ResetTimer,
}

#[tokio::main]
async fn main() {
    // say_hello(); // future => executor + ______ ?
    // say_hello().await;
    // await needs executor and reactor (the runtime).
    // you cannot run asynchronous code in sync code. => you need a runtime for it.
    // BUT if you need to run an async code in  a sync code in an async function ?
    //  => need nested runtimes. Nopes. not allowed.
    // async <- sync <- async

    let (tx, mut rx) = mpsc::channel::<Command>(10);
    let tx_outer = tx.clone();
    tokio::spawn(async move {
        // will take a thread from `hot threads` in thread pool that tokio manages
        let tx2 = tx.clone();
        let mut listened_key_mouse_event = spawn_blocking(move || {
            println!("Spawned keyboard / mouse listener");
            blocking_function(tx2)
        });
        dbg!(&listened_key_mouse_event);
    });

    // Timer task
    // let mut remaining_time = 5;
    let mut remaining_time = 5 * 60;
    let mut timer = tokio::time::interval(Duration::from_secs(1));
    println!("before tokio select! -> {} ", remaining_time);

    // tokio::spawn(my_timer(remaining_time));
    loop {
        tokio::select! {
        _ = timer.tick() => {
            remaining_time -= 1;
            if remaining_time <= 0 {
                tx_outer.send(Command::MoveMouse).await.unwrap();
                // break;
            }
            if remaining_time == 10 {
                println!(" {} ", remaining_time);
            }

        }
        Some(command) = rx.recv() => {
            // if let Ok(command_enum) = command {
            match command {
                Command::MoveMouse => {
                    // println!("What if mouse actually moved?");
                    random_mouse_move()
                }

                Command::ResetTimer => {

                    // remaining_time = 5; // Reset the timer
                    remaining_time = 5 * 60; // Reset the timer
                }
            }
        }
        }
    }
}

fn blocking_function(tx: mpsc::Sender<Command>) {
    let val = listen(move |event: Event| match event.event_type {
        EventType::MouseMove { x, y } => {
            // println!("MouseMove: {} {}", x, y);
            // tokio::time::sleep(Duration::from_millis(900));
            // tx.blocking_send(Command::MoveMouse).unwrap();
        }
        EventType::KeyPress(_) => {
            tx.blocking_send(Command::ResetTimer).unwrap();
        }
        other => {
            // println!("Got {:?} ", other);
        }
    });

    dbg!(&val);
}
