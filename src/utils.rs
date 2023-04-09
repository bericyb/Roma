use crate::Input;
use std::io::Write;
use std::thread;
use std::time::Duration;
use termion::terminal_size;

use crate::constants;

pub fn print_centered_text(text: &str, screen: (usize, usize)) {
    let mut banner = format!("{}", "\u{001b}[31m");
    for line in text.lines() {
        let padded_line = format!("{: ^1$}", line, (screen.0) as usize);
        banner = format!("{}{}", banner, padded_line);
    }
    println!("{}\u{001b}[0m", banner);
}

pub fn print_emoji(emoji: &str, emoji_position: &mut usize, screen: (usize, usize)) {
    *emoji_position += 1;
    if *emoji_position > screen.0 * 5 && *emoji_position < screen.0 * 6 {
        let padded_emoji = format!("{:>1$}", "🥫", *emoji_position % screen.0);
        println!("{}", padded_emoji);
        return;
    }

    if *emoji_position > screen.0 * 6 {
        *emoji_position = 0;
    }
    let padded_emoji = format!("{:>1$}", emoji, *emoji_position % screen.0);
    println!("{}\n", padded_emoji);
}

pub fn render_session(input: Input, session_num: i8, minutes: i32, message: String) {
    let handle = thread::spawn(move || {
        let total_frames = minutes * 3750;
        let mut emoji_position: usize = 0;
        for j in 0..total_frames {
            let (width_u16, height_u16) = terminal_size().unwrap();
            let screen: (usize, usize) = (width_u16 as usize, height_u16 as usize);
            println!("{}", termion::clear::All);

            print_emoji(&input.emoji, &mut emoji_position, screen);

            print_centered_text(
                &format!(
                    "Session {} of {} - {} minutes",
                    session_num + 1,
                    input.num_sessions,
                    minutes
                ),
                screen,
            );

            print_centered_text(constants::ROMA, screen);
            let percent = format!(
                "{}%\n",
                ((j as f32 / total_frames as f32) * 100 as f32) as i32
            );

            print_centered_text(&message, screen);
            print_centered_text(&percent, screen);

            println!(
                "\n{}\n",
                format!(
                    "{:#<1$}",
                    "#",
                    ((j as f32 / total_frames as f32) as f32 * screen.0 as f32) as usize
                )
            );

            thread::sleep(Duration::from_millis(constants::MILLISECONDS_PER_FRAME));
        }

        // Play bell sound
        print!("\x07");
        std::io::stdout().flush().expect("Failed to flush stdout");
        println!("{}", termion::clear::All);
    });
    handle.join().unwrap();
}
