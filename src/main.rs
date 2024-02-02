use rdev::{listen, Event, EventType::MouseMove};

use std::time::Instant;
use tokio::sync::{mpsc, mpsc::Sender};
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<()>(1);
    // let mut got_event = false;
    // let val = &mut got_event;

    // Spawn a task to handle mouse move events
    // tokio::task::spawn_blocking(move || {
    tokio::spawn(async move {
        let mut last_move = Instant::now();
        let tx2 = tx.clone();

        // Spawning a blocking task
        let mut listened_key_mouse_event =
            tokio::task::spawn_blocking(move || blocking_function(tx2)).await;

        // Simulate mouse move events every 2 seconds
        loop {
            // Wait for 2 seconds
            time::sleep(Duration::from_secs(1)).await;

            let elapsed = last_move.elapsed().as_secs();
            println!("entered loop \n - {}", elapsed);

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
            _ = rx.recv() => {
                remaining_time = 5 * 60; // Reset the timer
            }
        }
    }

    println!("Timer expired!");
}

async fn blocking_function(tx: Sender<()>) {
    // if let Err(error) = listen(callback) {
    //     println!("Error: {:?}", error);
    //     return;
    // }
    let val = listen(callback);
    println!("listen callback {:?}", val);

    // // if above doesn't give error, then reset the timer by sending event.
    // if let Err(_) = tx.send(()).await {
    //     println!("Failed to send reset_timer event.");
    // }
}

fn callback(event: Event) {
    // println!("Event: {:?}", event);

    match event.event_type {
        MouseMove { x, y } =>
        // how to send tx.send from here and reset the timer and exit this closure.
        {
            println!("MouseMove: {} {}", x, y);
        }
        other => {
            println!("Got {:?} ", other);
        }
    }
}
// async fn async_mouse_move(mut tx: tokio::sync::mpsc::Sender<()>) {
//     let mut last_move = std::time::Instant::now();

//     bind_any_button(Action::handle_mouse(|button| {
//         println!("Mouse button pressed {:?}", button);

//         // Send a message to reset the timer for any other key press
//         if let Err(_) = tx.send(()).now_or_never().unwrap() {
//             println!("Failed to send reset_timer event.");
//         }
//     }));

//     bind_any_key(Action::handle_kb(|key| {
//         match key {
//             Keyboard::Space | Keyboard::S => {
//                 // Send a message to reset the timer for any other key press
//                 // todo: check now_or_never
//                 if let Err(_) = tx.send(()).now_or_never().unwrap() {
//                     println!("Failed to send reset_timer event.");
//                 }
//                 last_move = std::time::Instant::now();
//             }
//             _ => {
//                 // ignore any other keys.
//             }
//         }
//     }));

//     loop {
//         if last_move.elapsed().as_secs() >= 2 {
//             // Simulate mouse move events every 2 seconds
//             println!("Mouse moved!");
//             last_move = Instant::now();
//         }

//         // Add some delay to avoid high CPU usage
//         tokio::time::sleep(Duration::from_millis(200)).await;
//     }
// }

// async fn async_mouse_move(mut tx: mpsc::Sender<()>) {
//     use crossterm::event::{self, Event, KeyCode};

//     let mut last_move = Instant::now();

//     loop {
//         if event::poll(Duration::from_secs(2)).await.unwrap_or(false) {
//             if let Event::Mouse(_) = event::read().unwrap() {
//                 // Mouse event detected, send message to reset the timer
//                 if let Err(_) = tx.send(()).await {
//                     println!("Failed to send reset_timer event.");
//                     break;
//                 }
//                 last_move = Instant::now();
//             } else if event::read().unwrap() == Event::Key(KeyCode::Char(' ')) {
//                 // Space bar pressed, send message to reset the timer
//                 if let Err(_) = tx.send(()).await {
//                     println!("Failed to send reset_timer event.");
//                     break;
//                 }
//                 last_move = Instant::now();
//             }
//         }

//         if last_move.elapsed().as_secs() >= 2 {
//             // Simulate mouse move events every 2 seconds
//             println!("Mouse moved!");
//             last_move = Instant::now();
//         }

//         // Add some delay to avoid high CPU usage
//         time::sleep(Duration::from_millis(100)).await;
//     }
// }
