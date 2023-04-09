use clap::Parser;
use std::io::stdin;
use termion::terminal_size;

pub mod constants;
mod utils;

#[derive(Parser, Clone)]
pub struct Input {
    /// Length of a work session in minutes
    #[clap(short, long, default_value_t = 25)]
    work_min: i32,

    /// Length of a short break in minutes
    #[clap(short, long, default_value_t = 5)]
    break_min: i32,

    /// Length of a long break in minutes
    #[clap(short, long, default_value_t = 20)]
    long_break: i32,

    /// If the timer's alarm is audible
    #[clap(short, long, default_value_t = false)]
    silent: bool,

    /// Number of work sessions until a long break
    #[clap(short, long, default_value_t = 4)]
    num_sessions: i8,

    /// Custom emoji
    #[clap(short, long, default_value = "🍅")]
    emoji: String,
}

fn main() {
    let input = Input::parse();

    let (width_u16, height_u16) = terminal_size().unwrap();
    let screen: (usize, usize) = (width_u16 as usize, height_u16 as usize);

    // Loop infinitely
    loop {
        // Count the number of sessions
        for i in 0..input.num_sessions {
            utils::print_centered_text(
                &format!("Work Session {} of {}", i + 1, input.num_sessions),
                screen,
            );

            utils::print_centered_text(constants::ROMA, screen);

            // Grab user input for task
            println!("Enter Current Task: \u{001b}[31m");
            let mut current_task = String::new();
            stdin()
                .read_line(&mut current_task)
                .ok()
                .expect("Failed to read line.");

            println!("{}", termion::clear::All);

            let task_string = format!("Current Task: {}", current_task);

            // Start work session
            utils::render_session(input.clone(), i, input.work_min, task_string);

            // Start break session
            if i < input.num_sessions - 1 {
                utils::render_session(
                    input.clone(),
                    i,
                    input.break_min,
                    format!("Break {} of {}", i + 1, input.num_sessions),
                );
            } else {
                utils::render_session(
                    input.clone(),
                    i,
                    input.long_break,
                    format!("Long break of {} minutes", input.long_break),
                );
            }
        }
    }
}
