// #![allow(dead_code)]
// #![allow(unused_variables)]

use rdev::{listen, Event, EventType::MouseMove};

use std::sync::{Arc, Mutex};
use std::time::Instant;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio::time::{self, Duration};

#[derive(Debug)]
enum Command {
    MouseMoved(Event), // TimerTicked,
}

type OneShotArc = Arc<Mutex<Option<oneshot::Sender<()>>>>;

#[tokio::main]
async fn main() {
    let (cmd_tx, mut cmd_rx) = mpsc::channel::<(Command, oneshot::Sender<Event>)>(1);

    // Spawn a task to handle mouse move events
    // tokio::task::spawn_blocking(move || {
    tokio::spawn(async move {
        let mut last_move = Instant::now();
        let tx2 = cmd_tx.clone();
        // Spawning a blocking task
        let (oneshot_tx, oneshot_rx) = oneshot::channel();

        let val = listen(move |event: Event| {
            match event.event_type {
                MouseMove { x, y } =>
                // how to send tx.send from here and reset the timer and exit this closure.
                {
                    println!("MouseMove: {} {}", x, y);
                    // tx2.send((Command::MouseMoved(event), oneshot_tx));
                    // oneshot_tx.send(());
                }
                other => {
                    println!("Got {:?} ", other);
                }
            }
        });

        // let mut listened_key_mouse_event =
        //     tokio::task::spawn_blocking(move || blocking_function(tx2)).await;
        // println!("spawn_blocking is running: \t ",);

        // Simulate mouse move events every 2 seconds
        loop {
            // Wait for 2 seconds
            time::sleep(Duration::from_secs(1)).await;

            let elapsed = last_move.elapsed().as_secs();
            println!("elapsed time - {}\n", elapsed);

            // match on event.name in closure
            // this will block the loop. which is what we want.

            //  pin box and suffering
            // https://github.com/unibg-seclab/Cage4Deno/blob/ddd61865d5c31782fe88ff5b00f86dceab2d3511/cli/tools/repl.rs#L716
            // https://github.com/fairhopeweb/tauri/blob/a50f24b2bd93864cacd9047b6fcd152f71a38c45/tooling/cli/src/helpers/web_dev_server.rs#L199
            // https://github.com/troy351/vscode/blob/f3966c3f624906464cedf3c644f9a652b3d5c446/cli/src/commands/serve_web.rs#L572
            // manual yield in tick timers -- https://github.com/itzhang89/vaultwarden/blob/0a4997a3b3a07d2a3750c201a81a100590219a19/src/api/notifications.rs#L140
            // tokio::select! {
            //     result_val =  listened_key_mouse_event => {
            //         println!("result_val: {:?}",result_val);
            //     }
            // }

            // println!("result tokio:task, {:?}", result.unwrap().await);
            // // if true {
            // // Send a message to the main task to reset the timer
            // if let Err(_) = tx.send(()).await {
            //     println!("Failed to send reset_timer event.");
            //     break;
            // }
            // }
            // last_move = Instant::now();
        }
    });

    // Spawn a task to handle mouse move events
    // tokio::spawn(async move {
    //     async_mouse_move(tx).await;
    // });

    // Timer task
    let mut remaining_time = 5 * 60; // 5 minutes
    let mut timer = time::interval(Duration::from_secs(1));
    println!("before tokio select! -> {} ", remaining_time);

    loop {
        tokio::select! {
                    _ = timer.tick() => {
                        remaining_time -= 1;
                        println!("Remaining time: {} seconds", remaining_time);
                        if remaining_time <= 0 {
                            break;
                        }
                    }
                     Some((Command::MouseMoved(event), oneshot_sender) ) = cmd_rx.recv() => {
                        println!("command: {:?} \t oneshot_sender: {:?}",event, oneshot_sender);
                        // oneshot_sender.send(event)
                        remaining_time = 5 * 60; // Reset the timer
                    }
        // what happens if cmd_rx gives some other event?
                }
    }

    println!("Timer expired!");
}

// async fn blocking_function(tx: mpsc::Sender<()>) {
//     // async fn blocking_function(oneshot_tx: oneshot::Sender<Event>) {
//     println!("\t in Blocking_fn");

//     let val = listen(move |event: Event| {
//         match event.event_type {
//             MouseMove { x, y } =>
//             // how to send tx.send from here and reset the timer and exit this closure.
//             {
//                 println!("MouseMove: {} {}", x, y);
//                 tx.send(());
//             }
//             other => {
//                 println!("Got {:?} ", other);
//             }
//         }
//     });
//     println!("listen callback {:?}", val);
// }
