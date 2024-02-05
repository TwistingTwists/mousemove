use autopilot::geometry::Point;
use rand::Rng;

pub fn random_mouse_move() {
    // let (screen_width, screen_height) = get_screen_size();
    let (screen_width, screen_height) = (1920, 1080);

    // Set the speed of the mouse movement
    let speed = 50.0;

    // loop {
    // Set the range for random values
    let random_range = 300;

    let mut rng = rand::thread_rng();

    // Get the current mouse position
    let current_position = autopilot::mouse::location();

    // Calculate the new position with added randomness
    let random_offset_x = rng.gen_range(-random_range..=random_range) as f64;
    let random_offset_y = rng.gen_range(-random_range..=random_range) as f64;

    // Calculate the new position (you can modify this according to your needs)
    let new_x = (current_position.x + speed + random_offset_x) as i32 % screen_width;
    let new_y = (current_position.y + speed + random_offset_y) as i32 % screen_height;

    // Move the mouse to the new position
    let _ = autopilot::mouse::move_to(Point::new(new_x as f64, new_y as f64));

    // // Sleep for a short duration to control the speed of the loop
    // thread::sleep(Duration::from_millis(500));
    // }
}
