use super::character::Character;

#[derive(Debug, Clone, Default)]
pub struct Team {
    characters: Vec<Character>,
}

impl Team {
    pub fn characters(&self) -> &Vec<Character> {
        &self.characters
    }

    pub fn add_character(&mut self, character: Character) {
        self.characters.push(character);
    }

    pub fn add_random_char(&mut self) {
        self.add_character(Character::random());
    }
}
