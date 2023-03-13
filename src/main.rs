#[allow(dead_code)]
use clap::Parser;
use std::io::stdin;
use std::thread;
use std::time::Duration;
use termion::terminal_size;

static ROMA: &str = "    ____  ____  __  ______ 
   / __ \\/ __ \\/  |/  /   |
  / /_/ / / / / /|_/ / /| |
 / _, _/ /_/ / /  / / ___ |
/_/ |_|\\____/_/  /_/_/  |_|
";

static FERRIS: &str = "                  ,.---.   
        ,,,,     /    _ `.
          \\\\\\\\   /      \\  )
         |||| /\\/``-.__\\/
  ::::/\\/_
{{`-.__.-'(`(^^(^^^(^ 9 `.========='
{{{{{{ { ( ( (  (   (-----:=
{{.-'~~'-.(,(,,(,,,(__6_.'=========.
  ::::\\/\\ 
        |||| \\/\\  ,-'/\\
        ////   \\ `` _/  )
       ''''     \\  `   /
                `---''";

#[derive(Parser)]
struct Input {
    /// Length of a work session in minutes
    #[arg(short, long, default_value_t = 25)]
    work_min: i32,

    /// Length of a short break in minutes
    #[arg(short, long, default_value_t = 5)]
    break_min: i32,

    /// Length of a long break in minutes
    #[arg(short, long, default_value_t = 15)]
    long_break: i32,

    /// If the timer's alarm is audible
    #[arg(short, long, default_value_t = false)]
    silent: bool,

    /// Number of work sessions until a long break
    #[arg(short, long, default_value_t = 4)]
    num_sessions: i8,
}

struct Sprite {
    x: i32,
    y: i32,
    ascii_string: String,
}

impl Sprite {
    fn render(&self) {
        let mut string = format!("{}", "\u{001b}[31m");
        for line in self.ascii_string.lines() {
            let padded_line = format!("{: >1$}", line, self.x as usize);
            string = format!("{}\n{}", string, padded_line);
        }
        println!("{}\u{001b}[0m", string);
    }
}

fn print_centered_text(text: &str) {
    let width = terminal_size().unwrap();
    let mut banner = format!("{}", "\u{001b}[31m");
    for line in text.lines() {
        let padded_line = format!("{: ^1$}", line, (width.0) as usize);
        banner = format!("{}{}", banner, padded_line);
    }
    println!("{}\u{001b}[0m", banner);
}
fn main() {
    let input = Input::parse();

    print_centered_text(ROMA);

    println!("Enter Current Task: \u{001b}[31m");
    let mut current_task = String::new();
    stdin()
        .read_line(&mut current_task)
        .ok()
        .expect("Failed to read line.");

    println!("{}", termion::clear::All);

    let handle = thread::spawn(move || {
        let sprite = Sprite {
            x: 0,
            y: 0,
            ascii_string: FERRIS.into(),
        };
        let total_frames = input.work_min * 3750;
        for i in 0..total_frames {
            let screen = terminal_size().unwrap();
            println!("{}", termion::clear::All);
            print_centered_text(ROMA);
            let percent = format!(
                "{}%",
                ((i as f32 / total_frames as f32) * 100 as f32) as i32
            );

            println!();
            let task_line = format!("Current Task: {}", current_task);
            print_centered_text(&task_line);
            print_centered_text(&percent);
            println!("{}", screen.1);

            // if screen.1 > 20 {
            //     sprite.render()
            // }

            println!(
                "{}",
                format!(
                    "{:#<1$}",
                    "#",
                    ((i as f32 / total_frames as f32) as f32 * screen.0 as f32) as usize
                )
            );

            thread::sleep(Duration::from_millis(16));
        }
    });

    handle.join().unwrap();
}
