use sdl2::mixer::Music;

use std::collections::HashMap;

pub struct Sound<'a> {
    // sound fx in order to enhance gaming experience
    pub sounds: HashMap<String, Music<'a>>,
}

impl Sound<'_> {
    pub fn new<'a>() -> Sound<'a> {
        let sounds = Sound::generate_sound();

        Sound { sounds }
    }

    pub fn play(&self, key: &str) {
        match self.sounds.get(key).unwrap().play(1) {
            Ok(_) => {}
            Err(str) => {
                println!("Error: {}", str);
            }
        }
    }

    fn generate_sound<'a>() -> HashMap<String, Music<'a>> {
        let mut sounds: HashMap<String, Music> = HashMap::new();
        sounds.insert(
            String::from("castle"),
            Music::from_file("sound/castling.mp3").unwrap(),
        );
        sounds.insert(
            String::from("check"),
            Music::from_file("sound/check.mp3").unwrap(),
        );
        sounds.insert(
            String::from("move"),
            Music::from_file("sound/placement.mp3").unwrap(),
        );
        sounds.insert(
            String::from("starting_game"),
            Music::from_file("sound/starting_game.mp3").unwrap(),
        );
        sounds.insert(
            String::from("take"),
            Music::from_file("sound/taking.mp3").unwrap(),
        );
        sounds.insert(
            String::from("game_over"),
            Music::from_file("sound/game_over.mp3").unwrap(),
        );
        sounds
    }
}
