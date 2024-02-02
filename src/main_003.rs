use autopilot::geometry::Point;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tokio::sync::mpsc;
use tokio::task;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<()>(100);
    let mouse_move_event_tx = tx.clone();

    let timer = Arc::new(Mutex::new(Instant::now()));

    let timer_clone = timer.clone();
    task::spawn(async move {
        loop {
            // Check if it's been more than 2 seconds since the last mouse move event
            if timer_clone.lock().unwrap().elapsed() >= Duration::from_secs(2) {
                tx.send(()).await.expect("Failed to send signal");
            }
            sleep(Duration::from_millis(500)).await;
        }
    });

    let screen_width = 1920;
    let screen_height = 1080;
    let speed = 50.0;

    loop {
        let random_range = 300;
        let mut rng = rand::thread_rng();
        let current_position = autopilot::mouse::location();

        let random_offset_x = rng.gen_range(-random_range..=random_range) as f64;
        let random_offset_y = rng.gen_range(-random_range..=random_range) as f64;

        let new_x = (current_position.x + speed + random_offset_x) as i32 % screen_width;
        let new_y = (current_position.y + speed + random_offset_y) as i32 % screen_height;

        let timer = timer.clone();
        let tx = mouse_move_event_tx.clone();

        task::spawn(async move {
            let timer = timer.clone();
            let tx = tx.clone();

            let mut receiver = rx.recv().await;
            // If we receive a signal, reset the timer
            while receiver.is_some() {
                timer.lock().unwrap().replace(Instant::now());
                receiver = rx.recv().await;
            }

            // Move the mouse cursor to the new position
            let _ = autopilot::mouse::move_to(Point::new(new_x as f64, new_y as f64));
        });

        sleep(Duration::from_millis(500)).await;
    }
}

// old code by chatgpt
// use autopilot::geometry::Point;
// use rand::Rng;
// use std::sync::atomic::{AtomicBool, Ordering};
// use std::sync::Arc;
// use std::time::Duration;
// use tokio::sync::mpsc;
// use tokio::time::{sleep, Duration as TokioDuration};

// #[tokio::main]
// async fn main() {
//     // Set the screen resolution (change these values according to your screen)
//     let (screen_width, screen_height) = (1920, 1080);

//     // Set the speed of the mouse movement
//     let speed = 30;

//     // Set the range for random values
//     let random_range = 300;

//     // Flag to indicate if mouse moved
//     let mouse_moved = Arc::new(AtomicBool::new(false));
//     let mouse_moved_clone = mouse_moved.clone();

//     // Timer duration and flag
//     let timer_duration = Duration::from_secs(300); // 5 minutes
//     let (timer_tx, mut timer_rx) = mpsc::channel(1);

//     // Spawn task to handle mouse move events
//     let (mouse_tx, mut mouse_rx) = mpsc::channel(1);
//     let mouse_moved_clone_task = mouse_moved_clone.clone();
//     tokio::spawn(async move {
//         while let Some(val) = mouse_rx.recv().await {
//             dbg!(&val);
//             mouse_moved_clone_task.store(true, Ordering::Relaxed);
//         }
//     });

//     // Timer task
//     tokio::spawn(async move {
//         sleep(TokioDuration::from_secs(300)).await;
//         let _ = timer_tx.send(()).await;
//     });

//     loop {
//         // Get the current mouse position
//         let current_position = autopilot::mouse::location();

//         // Calculate the new position with added randomness
//         let random_offset_x = rand::thread_rng().gen_range(-random_range..=random_range);
//         let random_offset_y = rand::thread_rng().gen_range(-random_range..=random_range);

//         let new_x = ((current_position.x + speed + random_offset_x) % screen_width + screen_width)
//             % screen_width;
//         let new_y = ((current_position.y + speed + random_offset_y) % screen_height
//             + screen_height)
//             % screen_height;

//         // Move the mouse to the new position
//         autopilot::mouse::move_to(
//             Point::new(new_x as f64, new_y as f64),
//             Duration::from_millis(100),
//         );

//         tokio::select! {
//             _ = mouse_rx.recv() => {
//                 mouse_moved.store(true, Ordering::Relaxed);
//             }
//             _ = timer_rx.recv() => {
//                 if !mouse_moved.load(Ordering::Relaxed) {
//                     loop {
//                         let new_x = rand::thread_rng().gen_range(0..screen_width) as f64;
//                         let new_y = rand::thread_rng().gen_range(0..screen_height) as f64;
//                         autopilot::mouse::move_to(Point::new(new_x, new_y), Duration::from_millis(100));
//                         if mouse_rx.recv().await.is_ok() {
//                             break;
//                         }
//                     }
//                 }
//             }
//         }

//         if mouse_moved.load(Ordering::Relaxed) {
//             mouse_moved.store(false, Ordering::Relaxed);
//         }

//         // Sleep for a short duration to control the speed of the loop
//         sleep(Duration::from_millis(100)).await;
//     }
// }
