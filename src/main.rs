// Author:
//  _____ ___
// |  ___/ _ \__/\__
// | |_ | | | \    /
// |  _|| |_| /_  _\
// |_|   \___/  \/

// Initialize termion crate
extern crate termion;

// Import all needed crates
use std::io::{Write, stdout};

use std::time;
use std::thread;
use std::sync::mpsc;
use std::process;

use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;

// Initialixe all game constants
const MAP: [str; 12] = [
"############",
"#..........#",
"#..........#",
"#..........#",
"#..........#",
"#..........#",
"#..........#",
"#..........#",
"#..........#",
"#..........#",
"#..........#",
"############",];
const PLAYER_FOV: u8 = 90;    // FOV of the player
const PLAYER_ROTATION_SPEED: f32 = 1.5;    // Speed of the player rotation
const PLAYER_MOVEMENT_SPEED: f32 = 0.1;    // Speed of the players movement


fn main() {
    // Get input stream
    let mut stdin = termion::async_stdin().keys();
    // Enter raw mode
    let mut stdout = stdout().into_raw_mode().unwrap();
    // Hide cursor
    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Hide).unwrap();
    stdout.flush().unwrap();
    
    // Initializing all game variables
    let mut player_rotation: f32 = 0.0;    // Current rotation of the player
    let mut player_position_x: f32 = 5.0;    // Current x position of the player
    let mut player_position_y: f32 = 5.0;    // Current y position of the player


    // MAIN LOOP
    loop {
        let ter_size = termion::terminal_size().unwrap();
        for column in 1..=ter_size.0 {
            for line in 2..=ter_size.1 {
                write!(stdout, "{set_cursor}#",
                        set_cursor = termion::cursor::Goto(column, line)).unwrap();
            }
        }
        write!(stdout, "{set_cursor}{invert}rotation: {}{reset}",
                player_rotation,
                set_cursor = termion::cursor::Goto(1, 1),
                invert = termion::style::Invert,
                reset = termion::style::Reset).unwrap();
        stdout.flush().unwrap();
        thread::sleep(time::Duration::from_millis(50));

        // Checking if a key was pressed
        let input = stdin.next();
        if let Some(Ok(key)) = input {
            match key {
                // Exit if esc was pressed
                Key::Esc => break,
                Key::Char('a') => player_rotation -= PLAYER_ROTATION_SPEED,
                Key::Char('d') => player_rotation += PLAYER_ROTATION_SPEED,
                _ => (),
            }
        }
    }
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
