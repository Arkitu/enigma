use std::{io::stdout, time::Duration, collections::VecDeque};

use crossterm::{terminal, cursor, execute, event::{self, KeyCode}, style::Stylize};
use anyhow::Result;

struct CleanUp;
impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().unwrap();
        execute!(stdout(), cursor::Show).unwrap();
    }
}

fn char_to_number(c: char) -> u8 {
    if !c.is_ascii_alphabetic() {
        panic!("pas une lettre");
    }
    (c as u8) - 97
}
fn chars_to_numbers(string: &str) -> Vec<u8> {
    string.chars().map(|c| char_to_number(c)).collect()
}

fn number_to_char(i: u8) -> char {
    if i > 25 {
        panic!("trop grand");
    }
    (i+97) as char
}

struct Rotor {
    matching: VecDeque<u8>,
    get_count: usize,
    max_count: usize
}
impl Rotor {
    pub fn new(matching: &[u8], max_count: usize) -> Self {
        Self {
            matching: matching.to_vec().into(),
            get_count: 0,
            max_count
        }
    }
    pub fn default_rotors() -> [Self; 3] {
        [
            Rotor::new(&chars_to_numbers("ekmflgdqvzntowyhxuspaibrcj"), 26*26),
            Rotor::new(&chars_to_numbers("ajdksiruxblhwtmcqgznpyfvoe"), 26),
            Rotor::new(&chars_to_numbers("bdfhjlcprtxvznyeiwgakmusqo"), 1)
        ]
    }
    pub fn rotate(&mut self) {
        let n = self.matching.pop_back().unwrap();
        self.matching.push_front(n);
    }
    pub fn get(&mut self, i: u8, reverse: bool) -> u8 {
        let result = if !reverse {
            *self.matching.get(i as usize).unwrap()
        } else {
            self.matching.iter().position(|&x| x == i).unwrap() as u8
        };
        self.get_count+=1;
        if self.get_count <= self.max_count {
            self.get_count = 0;
            self.rotate();
        }
        result
    }
}

fn main() -> Result<()> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    execute!(stdout(), cursor::Hide)?;
    let mut lit_up: Option<char> = None;
    let mut rotors = Rotor::default_rotors()
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
                    } => if c.is_ascii_alphabetic() {
                        let c = c.to_lowercase().next().unwrap();
                        let mut i = char_to_number(c);
                        for rotor in rotors.iter_mut() {
                            println!("{}", i);
                            i = rotor.get(i, false);
                        }
                        for rotor in rotors.iter_mut() {
                            println!("{}", i);
                            i = rotor.get(i, true);
                        }
                        lit_up = Some(number_to_char(i));
                    },
                    event::KeyEvent {
                        code: KeyCode::Enter,
                        ..
                    } => {
                        rotors = Rotor::default_rotors();
                    },
                    _ => {}
                }
            }
        }


        if let Some(c) = lit_up {
            let c = c.to_uppercase().next().unwrap();
            if let Some(splited) = text.split_once(c) {
                text = splited.0.to_string() + &c.black().on_dark_yellow().to_string() + splited.1;
            }
        }

        execute!(stdout(), cursor::MoveTo(0, 0))?;
        execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
        println!("{}", text);
    }
}
