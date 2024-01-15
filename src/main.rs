use autopilot::geometry::Point;
use std::thread;
use std::time::Duration;

fn main() {
    // Set the screen resolution (change these values according to your screen)
    let screen_width = 1920;
    let screen_height = 1080;

    // Set the speed of the mouse movement
    let speed = 50.0;

    loop {
        // Get the current mouse position
        let current_position = autopilot::mouse::location();

        // Calculate the new position (you can modify this according to your needs)
        let new_x = (current_position.x + speed)as i32 % screen_width;
        let new_y = (current_position.y + speed) as i32 % screen_height;

        // Move the mouse to the new position
        autopilot::mouse::move_to(Point::new(new_x as f64, new_y as f64));

        // Sleep for a short duration to control the speed of the loop
        thread::sleep(Duration::from_millis(500));
    }
}
