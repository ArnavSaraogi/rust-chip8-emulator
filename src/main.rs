mod chip8;
mod display;
mod timers;
mod stack;
use chip8::Chip8;
use std::io;
use std::process;
use std::io::Write;

fn main() {
    loop {
        println!("\nWelcome to Arnav and Mikey's CHIP-8 Emulator! The games you can play are listed below.");
        println!("    1. Astro Dodge");
        println!("    2. Blitz");
        println!("    3. Brix");
        println!("    4. Nim");
        println!("    5. Pong");
        println!("    6. Space Invaders");

        let game_choice = loop {
            print!("Enter a number from 1 to 6 to play a game, or 0 to quit: ");
            io::stdout().flush().expect("Failed to flush stdout");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");

            match input.trim().parse::<u32>() {
                Ok(num) if (0..=6).contains(&num) => break num,
                _ => println!("\nInvalid input. Enter a number between 1 and 6."),
            }
        };

        println!("");

        let mut path = "";
        match game_choice {
            0 => process::exit(0),
            1 => {
                path = "roms/astro_dodge.ch8";
                println!("Your goal is to make your way through the asteroids field and dodge the asteroids,\n\
                        scoring points for each asteroid you are able to dodge.\n\
                        2,Q,E,S will move your ship, W will start the game.");
            }
            2 => {
                path = "roms/blitz.ch8";
                println!("You are in a plane, and you must destroy the towers of a town.\n\
                        Your plane is flying left to right, and goes down. Use W to drop a bomb.\n\
                        The game ends when you crash yourself on a tower...");
            }
            3 => {
                path = "roms/brix.ch8";
                println!("Use your slider to break all the bricks! Use Q and E to move left and right.");
            }
            4 => {
                path = "roms/nim.ch8";
                println!("Each turn, you subtract 1, 2 or 3 from the score, playing against the computer.\n\
                        Whoever ends up with the last number loses!\n\
                        Press V to go first, and any other for the computer to go first.\n\
                        Press 1, 2, or 3 to subtract respective amount of points.");
            }
            5 => {
                path = "roms/pong.ch8";
                println!("Don't let the ball past you! Use 1 and Q to move.");
            }
            6 => {
                path = "roms/space_invaders.ch8";
                println!("Destroy the invaders with your ship. Shoot with W, move with Q and E.");
            }
            _ => {}
        }

        let start = loop {
            print!("\nStart game? 1 to start, 0 to quit: ");
            io::stdout().flush().expect("Failed to flush stdout");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");

            match input.trim().parse::<u32>() {
                Ok(num) if (0..=1).contains(&num) => break num,
                _ => println!("\nInvalid input. Enter 0 or 1."),
            }
        };

        match start {
            0 => process::exit(0),
            1 => {
                let mut chip8: Chip8 = Chip8::default();
                let result = chip8.load_rom(path);
                if result.is_err() {
                    println!("Error in reading file");
                }
                chip8.run();
            }, 
            _ => {}
        }
    }
}