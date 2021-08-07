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

use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;

// Initialixe all game constants
const MAP: [&str; 12] = [
"############",
"#..........#",
"#.....##...#",
"#..........#",
"#..........#",
"#..........#",
"#..........#",
"#..........#",
"######.....#",
"#..........#",
"#..........#",
"############",];
const SHADING: [char; 12] = ['@', '%', '#', '*', '+', '=', '~', '-', ';', ':', '.', ' '];
const PLAYER_FOV: u8 = 1;    // FOV of the player
const PLAYER_ROTATION_SPEED: f32 = 0.25;    // Speed of the player rotation
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
        // Get terminal size
        let ter_size = termion::terminal_size().unwrap();
        // Loop through every "Pixel" of the screen
        for column in 1..=ter_size.0 {
            let ray_angle: f32 = player_rotation + ((PLAYER_FOV as f32) * ((((column as f32) / (ter_size.0 as f32)) * 2.0) - 1.0));
            let mut i = 1;
            let mut ray_x = 5.0;
            let mut ray_y = 5.0;
            while MAP[ray_y as usize].as_bytes()[ray_x as usize] as char != '#' {
                ray_x = player_position_x + (f32::sin(ray_angle) * (i as f32));
                ray_y = player_position_y + (f32::cos(ray_angle) * (i as f32));
                i += 1;
            }
            let offset = i;
            for line in 2..=ter_size.1 {
                let char_to_print = if (line <= (2 + offset)) | (line >= (ter_size.1 - offset)) {' '} else {if offset < 12 {SHADING[(offset - 1) as usize]} else {' '}};
                // Move cursor to targeted pixel and display the needed char
                if (column > 12) | (line > 13) {
                    write!(stdout, "{set_cursor}{}\r",
                            char_to_print,
                            set_cursor = termion::cursor::Goto(column, line)).unwrap();
                }
            }
        }

        // Display all access information in the top left corner of the screen
        write!(stdout, "{set_cursor}{clear}{invert}rotation: {}{reset}",
                player_rotation,
                set_cursor = termion::cursor::Goto(1, 1),
                clear = termion::clear::CurrentLine,
                invert = termion::style::Invert,
                reset = termion::style::Reset).unwrap();
        // Print a live minimap to the screen
        for i in 0..12 {
            for ii in 0..12 {
                // Check if the current part of the map is empty or if the player is on it
                let field_is_player = if (player_position_x as u16 == ii) & (player_position_y as u16 == i) {true} else {false};
                write!(stdout, "{set_cursor}{style}{}{reset}",
                       if field_is_player {'P'} else {MAP[i as usize].as_bytes()[ii as usize] as char},
                       set_cursor = termion::cursor::Goto(ii as u16 + 1, i as u16 + 2),
                       style = termion::color::Fg(termion::color::Green),
                       reset = termion::style::Reset).unwrap();
            }
        }
        stdout.flush().unwrap();

        // Checking if a key was pressed
        let input = stdin.next();
        if let Some(Ok(key)) = input {
            match key {
                // Exit if esc was pressed
                Key::Esc => break,
                Key::Char('a') => player_rotation -= PLAYER_ROTATION_SPEED,
                Key::Char('d') => player_rotation += PLAYER_ROTATION_SPEED,
                Key::Char('w') => {
                    player_position_x += (f32::sin(player_rotation) * PLAYER_MOVEMENT_SPEED);
                    player_position_y += (f32::cos(player_rotation) * PLAYER_MOVEMENT_SPEED);
                }
                Key::Char('s') => {
                    player_position_x -= (f32::sin(player_rotation) * PLAYER_MOVEMENT_SPEED);
                    player_position_y -= (f32::cos(player_rotation) * PLAYER_MOVEMENT_SPEED);
                }
                _ => (),
            }
        }
    }
    write!(stdout, "{set_cursor}{clear}{}",
            termion::cursor::Show,
            set_cursor = termion::cursor::Goto(1, 1),
            clear = termion::clear::AfterCursor).unwrap();
}
