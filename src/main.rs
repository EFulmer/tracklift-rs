extern crate chrono;
use chrono::prelude::*;

use std::io::{self, Read};

struct DailyWeight {
    time: DateTime<Local>,
    weight: u8,
}

#[derive(Show, Ord)]
struct Workout {
    date: Date<Local>,
    lifts: Vec<Lift>,
    user: String,
}

struct Lift {
    name: String,
    sets: Vec<Set>,
}

struct Set {
    warm_up: bool,
    weight: u32,
    reps: u8,
    notes: String,
}

fn make_workout(date: Date<Local>) -> Workout {
    Workout { date: date, lifts: vec![], user: "Eric" }
}

fn main() {
    // TODO use `clap` library.
    let mode = std::env::args().nth(1).expect("Need a mode such as `cli` or `server`.");
    println!("Your choice for mode was {}", mode);
    if mode == "cli" {
        cli_mode();
    }
}

fn cli_mode() {
    let mut user_input = String::new();
    let stdin = io::stdin();

    loop {
        println!("What do you want to do?");
        println!("");
        println!("[1] Add a workout");
        println!("[2] View a workout");
        println!("(\"b\" will take you back at any time)");
        print!("Your choice: ");

        stdin.read_line(&mut user_input);
        // TODO figure out why `str` needed for `.trim()`
        user_input = user_input.replace("\n", "");
        println!("");
        println!("\nYour choice was {}", user_input);

        if user_input == "1" {
            add_workout();
        } else if user_input == "2" {
            list_workouts();
        } else{
            println!("Sorry, I don't understand that. I was expecting \"1\" or \"2\".");
        }
    }
}

fn add_workout() {
    let mut user_input = String::new();
    let stdin = io::stdin();
    println!("When did you do this workout? (we're assuming today)");
    // TODO get some smart date parsing
    let day = stdin.read_line(&mut user_input);
}

fn list_workouts() {
    println!("Sorry, I'm not implemented yet!");
}

fn server_mode() {
    println!("Sorry, I'm not implemented yet!");
}
