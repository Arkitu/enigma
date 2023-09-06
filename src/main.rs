use std::{io::stdout, time::Duration};

use crossterm::{terminal, cursor, execute, event::{self, KeyCode}, style::Stylize};
use anyhow::Result;

struct CleanUp;
impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().unwrap();
        execute!(stdout(), cursor::Show).unwrap();
    }
}

struct Rotor {
    input: [char; 26],
    output: [char; 26]
}
impl Rotor {
    pub fn new(input: [char; 26])
}

fn main() -> Result<()> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    execute!(stdout(), cursor::Hide)?;
    let mut last_in: Option<char> = None;
    loop {
        let mut text = format!("
            A Z E R T Y U I O P\r
            Q S D F G H J K L M\r
                W X C V B N");
        
        if event::poll(Duration::from_millis(1000))? {
            if let event::Event::Key(k) = event::read()? {
                match k {
                    event::KeyEvent {
                        code: KeyCode::Char(c),
                        ..
                    } => if c.is_alphabetic() {
                        last_in = Some(c.to_uppercase().next().unwrap())
                    },
                    _ => {}
                }
            }
        }



        if let Some(c) = last_in {
            if let Some(splited) = text.split_once(c) {
                text = splited.0.to_string() + &c.black().on_dark_yellow().to_string() + splited.1;
            }
        }

        execute!(stdout(), cursor::MoveTo(0, 0))?;
        execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
        println!("{}", text);
    }
}
