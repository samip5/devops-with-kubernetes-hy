use rand::{distributions::Alphanumeric, Rng};
use std::{thread, time::Duration};
use chrono::Local;

fn main() {
    // Generate a random string on startup
    let random_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16) // Generate a string of 16 random alphanumeric characters
        .map(char::from)
        .collect();

    println!("Generated random string: {}", random_string);

    // Loop to output the string every 5 seconds with a timestamp
    loop {
        let current_time = Local::now(); // Get the current timestamp
        println!("[{}] {}", current_time.format("%Y-%m-%d %H:%M:%S"), random_string);
        thread::sleep(Duration::from_secs(5)); // Wait for 5 seconds
    }
}
