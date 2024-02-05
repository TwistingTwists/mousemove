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



----
modify this code to do the following
1. Run a function called async mouse_move
2. keep listening to mouse events
3. Reset the timer when mouse event is received or space bar is pressed.





---
use mki::{bind_key, Action, InhibitEvent, Keyboard, Sequence};
use std::thread;
use std::time::Duration;

fn main() {
    Keyboard::A.bind(|_| {
        println!("A pressed, sending B");
        Keyboard::B.click();
    });
    mki::bind_any_key(Action::handle_kb(|key| {
        use Keyboard::*;
        if matches!(key, S | L | O | W | LeftShift | LeftControl | B) {
            // Ignore outputs from other commands for nicer output
        } else {
            println!("Some key pressed pressed: {:?}", key);
        }
    }));
    mki::bind_any_button(Action::handle_mouse(|button| {
        println!("Mouse button pressed {:?}", button);
    }));
    mki::register_hotkey(&[Keyboard::LeftControl, Keyboard::B], || {
        println!("Ctrl+B Pressed")
    });
    mki::bind_key(
        Keyboard::S,
        Action::sequencing_kb(|_| {
            Sequence::text("LLLLLow").unwrap().send();
            thread::sleep(Duration::from_secs(1));
        }),
    );

    // This binds action to a W key,
    // that W press will not be sent to the following services ( only on windows )
    // whenever Caps Lock is toggled
    // Action will be executed on separate thread.
    bind_key(
        Keyboard::W,
        Action {
            callback: Box::new(|event, state| {
                println!("key: {:?} changed state now is: {:?}", event, state);
            }),
            inhibit: InhibitEvent::maybe(|| {
                if Keyboard::CapsLock.is_toggled() {
                    InhibitEvent::Yes
                } else {
                    InhibitEvent::No
                }
            }),
            sequencer: false,
            defer: true,
        },
    );

    thread::sleep(Duration::from_secs(100));
}

---
this is documentation of mki library.
rewrite the async_mouse_move function using mki library. Use the above as reference to remove the dependency on crossterm crate.
