use std::time::Instant;
use tokio::sync::mpsc;
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<()>(1);

    // Spawn a task to handle mouse move events
    tokio::spawn(async move {
        let mut last_move = Instant::now();
        // Simulate mouse move events every 2 seconds
        loop {
            // Wait for 2 seconds
            time::sleep(Duration::from_secs(2)).await;
            let elapsed = last_move.elapsed().as_secs();
            if elapsed >= 2 {
                // Send a message to the main task to reset the timer
                if let Err(_) = tx.send(()).await {
                    println!("Failed to send reset_timer event.");
                    break;
                }
            }
            last_move = Instant::now();
        }
    });

    // Timer task
    let mut remaining_time = 5 * 60; // 5 minutes
    let mut timer = time::interval(Duration::from_secs(1));

    loop {
        tokio::select! {
            _ = timer.tick() => {
                remaining_time -= 100;
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
