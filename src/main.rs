use std::thread;
use std::time::Duration;
use chrono::{Local, Timelike};

#[derive(Debug)]
struct Reminder {
    activity_name: String,
    hour: u32,
    minute: u32,
}

fn main() {
    println!("My Assistant");

    let mut reminders = Vec::new();

    loop {
        // Input: activity name and time
        let mut activity_name = String::new();
        println!("What do you want to do:");
        std::io::stdin().read_line(&mut activity_name).expect("Failed to read input");
        let activity_name = activity_name.trim().to_string(); // Trimming input

        // Input hour and minute with validation
        let mut when_hour: u32;
        let mut when_minute: u32;

        loop {
            // Reading the hour
            let mut hour_input = String::new();
            println!("At what time? Write Hour (24-hour format):");
            std::io::stdin().read_line(&mut hour_input).expect("Failed to read input");
            when_hour = hour_input.trim().parse().expect("Please enter a valid number, Only hour eg 20 for 8pm");

            // Validate hour range
            if when_hour > 23 {
                println!("Invalid hour. Please enter an hour between 0 and 23.");
                continue;
            }

            // Reading the minute
            let mut minute_input = String::new();
            println!("How many minutes passed the hour:");
            std::io::stdin().read_line(&mut minute_input).expect("Failed to read input");
            when_minute = minute_input.trim().parse().expect("Please enter a valid number");

            // Validate minute range
            if when_minute > 59 {
                println!("Invalid minute. Please enter a minute between 0 and 59.");
                continue;
            }

            // Confirming reminder
            println!("I want to {} at {:02}:{:02}.", activity_name, when_hour, when_minute);

            let now = Local::now();

            // Check if the time set is valid (not in the past)
            if when_hour < now.hour() || (when_hour == now.hour() && when_minute <= now.minute()) {
                println!("The time you entered has already passed. Please set a future time.");
            } else {
                break; // Exit the loop if the time is valid
            }
        }

        // Storing reminder with a clone of activity_name to avoid ownership issues
        reminders.push(Reminder {
            activity_name: activity_name.clone(),
            hour: when_hour,
            minute: when_minute,
        });

        // Ask user if they want to add more reminders or view them.
        println!("Do you have something else you would like to do? (y/n) or view reminders (v)?");
        let mut add_more = String::new();
        std::io::stdin().read_line(&mut add_more).expect("Failed to read input");

        let add_more = add_more.trim().to_lowercase();
        if add_more == "y" {
            continue; // Continue adding more reminders
        } else if add_more == "v" {
            // View all added reminders.
            println!("\nYour reminders:");
            for (index, reminder) in reminders.iter().enumerate() {
                println!("{}: {} at {:02}:{:02}", index + 1, reminder.activity_name, reminder.hour, reminder.minute);
            }
        } else {
            break; // Exit loop if the user doesn't want to add more or view reminders
        }
    }

    // Reminder loop
    loop {
        let now = Local::now();

        // Check each reminder
        for reminder in &reminders {
            if now.hour() == reminder.hour && now.minute() == reminder.minute {
                println!("Time to {} at {}!", reminder.activity_name, now.format("%H:%M"));
            }
        }

        // Sleep for 60 seconds to prevent constant CPU usage and check once per minute
        thread::sleep(Duration::from_secs(60));
    }
}
