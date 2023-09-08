use std::{io::stdout, time::Duration, collections::VecDeque};
use crossterm::{terminal, cursor, execute, event, style::Stylize};
use anyhow::Result;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

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

#[derive(Component)]
struct Id(usize);

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

#[derive(Resource)]
struct Rotors([Rotor; 3]);

#[derive(Resource)]
struct CharLitUp(Option<char>);

#[derive(Component)]
struct Char(char);

fn main() {
    let rotors = Rotor::default_rotors();
    App::new()
        .insert_resource(Rotors(rotors))
        .insert_resource(CharLitUp(None))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (keyboard_input, light_up_char))
        .run()
}

fn spawn_letter(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    material: Handle<ColorMaterial>,
    asset_server: &Res<AssetServer>,
    x: f32,
    y: f32,
    c: char
) {
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(x, y, 0.),
            ..default()
        },
        Char(c)
    )).with_children(|parent| {
        parent.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(30.).into()).into(),
            material,
            ..default()
        });
        parent.spawn(Text2dBundle {
            text: Text::from_section(c,
                TextStyle { 
                    font: asset_server.load("fonts/Call of Ops Duty.otf"),
                    font_size: 40.,
                    color: Color::GRAY
                }),
            ..default()
        });
    });
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2dBundle::default());

    let material = materials.add(ColorMaterial::from(Color::BLACK));

    for (i, c) in "QWERTZUIO".chars().enumerate() {
        println!("({}, {})", ((i as f32)*70.)-280., 0.);
        spawn_letter(&mut commands, &mut meshes, material.clone(), &asset_server, ((i as f32)*100.)-400., 100., c);
    }
    for (i, c) in "ASDFGHJK".chars().enumerate() {
        println!("({}, {})", ((i as f32)*70.)-280., 0.);
        spawn_letter(&mut commands, &mut meshes, material.clone(), &asset_server, ((i as f32)*100.)-350., 0., c);
    }
    for (i, c) in "PYXCVBNML".chars().enumerate() {
        println!("({}, {})", ((i as f32)*70.)-280., 0.);
        spawn_letter(&mut commands, &mut meshes, material.clone(), &asset_server, ((i as f32)*100.)-400., -100., c);
    }
}

fn keyboard_input(keys: Res<Input<KeyCode>>, mut rotors: ResMut<Rotors>, mut lit_up: ResMut<CharLitUp>) {
    for key in keys.get_just_pressed() {
        let c = match key {
            KeyCode::A => 'a',
            KeyCode::B => 'b',
            KeyCode::C => 'c',
            KeyCode::D => 'd',
            KeyCode::E => 'e',
            KeyCode::F => 'f',
            KeyCode::G => 'g',
            KeyCode::H => 'h',
            KeyCode::I => 'i',
            KeyCode::J => 'j',
            KeyCode::K => 'k',
            KeyCode::L => 'l',
            KeyCode::M => 'm',
            KeyCode::N => 'n',
            KeyCode::O => 'o',
            KeyCode::P => 'p',
            KeyCode::Q => 'q',
            KeyCode::R => 'r',
            KeyCode::S => 's',
            KeyCode::T => 't',
            KeyCode::U => 'u',
            KeyCode::V => 'v',
            KeyCode::W => 'w',
            KeyCode::X => 'x',
            KeyCode::Y => 'y',
            KeyCode::Z => 'z',
            KeyCode::Return => {
                rotors.0 = Rotor::default_rotors();
                lit_up.0 = None;
                continue
            },
            _ => continue
        };
        if c.is_ascii_alphabetic() {
            let rotors = &mut rotors.0;
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
            lit_up.0 = Some(number_to_char(i));
        }
    }
}

fn light_up_char(lit_up: Res<CharLitUp>, q_chars: Query<(&Char, &Children)>, mut q_child: Query<&mut Handle<ColorMaterial>>, mut materials: ResMut<Assets<ColorMaterial>>,) {
    if let Some(c) = lit_up.0 {
        let c = c.to_uppercase().next().unwrap();
        for (parent, children) in q_chars.iter() {
            let material = materials.add(ColorMaterial {
                color: if parent.0 == c {
                    Color::YELLOW
                } else {
                    Color::BLACK
                },
                ..default()
            });
            for &child in children.iter() {
                let mut m = match q_child.get_mut(child) {
                    Ok(m) => m,
                    _ => continue
                };
                *m = material.to_owned();
            }

        }
    }
}

fn main_old() -> Result<()> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    execute!(stdout(), cursor::Hide)?;
    let mut lit_up: Option<char> = None;
    let mut rotors = Rotor::default_rotors();
    loop {
        let mut text = format!("
            A Z E R T Y U I O P\r
            Q S D F G H J K L M\r
                W X C V B N");
        
        if event::poll(Duration::from_millis(1000))? {
            if let event::Event::Key(k) = event::read()? {
                match k {
                    event::KeyEvent {
                        code: event::KeyCode::Char(c),
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
                        code: event::KeyCode::Enter,
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
