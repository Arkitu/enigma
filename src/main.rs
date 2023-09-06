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
    match c {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        'i' => 8,
        'j' => 9,
        'k' => 10,
        'l' => 11,
        'm' => 12,
        'n' => 13,
        'o' => 14,
        'p' => 15,
        'q' => 16,
        'r' => 17,
        's' => 18,
        't' => 19,
        'u' => 20,
        'v' => 21,
        'w' => 22,
        'x' => 23,
        'y' => 24,
        'z' => 25,
        _ => panic!("pas une lettre")
    }
}
fn chars_to_numbers(string: &str) -> Vec<u8> {
    string.chars().map(|c| char_to_number(c)).collect()
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
    let mut last_in: Option<char> = None;
    let rotors = [
        Rotor::new(&chars_to_numbers("ekmflgdqvzntowyhxuspaibrcj"), 26*26),
        Rotor::new(&chars_to_numbers("ajdksiruxblhwtmcqgznpyfvoe"), 26),
        Rotor::new(&chars_to_numbers("bdfhjlcprtxvznyeiwgakmusqo"), 1)
    ];
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
