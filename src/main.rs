use std::io;

use sodiumoxide;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use interface::app::AppResult;
use interface::event::{Event, EventHandler};
use interface::handler::handle_key_events;
use otp::otp_helper;
use interface::ui::Tui;
use zeroize::Zeroize;

mod utils;
mod argument_functions;
mod cryptography;
mod importers;
mod otp;
mod interface;
mod database_management;
mod args;

fn init() -> Result<bool, String> {
    match sodiumoxide::init() {
        Err(()) => {
            return Err(String::from("Error during sodiumoxide initialization"));
        }
        _ => {}
    };
    match utils::create_db_if_needed() {
        Ok(value) => {
            if value {
                let mut pw = cryptography::prompt_for_database_password("Choose a password: ", 0, true);
                let result = match database_management::overwrite_database_json("[]", &pw) {
                    Ok(()) => Ok(true),
                    Err(_e) => Err(String::from("An error occurred during database overwriting")),
                };
                pw.zeroize();
                return result;
            }
            Ok(false)
        }
        Err(()) => {
            return Err(String::from("An error occurred during database creation"));
        }
    }
}

fn main() -> AppResult<()> {
    match init() {
        Ok(true) => {
            println!("Database correctly initialized");
            return Ok(());
        }
        Ok(false) => {}
        Err(e) => {
            println!("{}", e);
            std::process::exit(-1);
        }
    }
    match args::args_parser() {
        // no args, show dashboard
        true => match dashboard(){
            Ok(()) =>std::process::exit(0),
            Err(_) => std::process::exit(-2), 
        },
        // args parsed, can exit
        false => std::process::exit(0),
    }
}

fn dashboard() -> AppResult<()> {
    match otp_helper::read_codes() {
        Ok(elements) => {
            if elements.len() == 0 {
                println!("No codes, type \"cotp -h\" to get help");
            } else {
                // Create an application.
                let mut app = interface::app::App::new(elements);

                // Initialize the terminal user interface.
                let backend = CrosstermBackend::new(io::stderr());
                let terminal = Terminal::new(backend)?;
                let events = EventHandler::new(250);
                let mut tui = Tui::new(terminal, events);
                tui.init()?;

                // Start the main loop.
                while app.running {
                    // Render the user interface.
                    tui.draw(&mut app)?;
                    // Handle events.
                    match tui.events.next()? {
                        Event::Tick => app.tick(),
                        Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
                        Event::Mouse(_) => {}
                        Event::Resize(_, _) => {}
                    }
                }

                // Exit the user interface.
                tui.exit()?;
            }
        }
        Err(e) => {
            eprintln!("An error occurred: {}", e);
            return Err(Box::new(io::Error::new(io::ErrorKind::InvalidInput, e)));
        }
    }
    Ok(())
}