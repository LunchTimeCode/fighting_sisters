use rand::seq::SliceRandom;

#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct Character {
    name: String,
    image: String,
    health: i32,
    attack: i32,
    level: i32,
    experience: i32,
    class: CharacterClass,
}

impl Character {
    pub fn new(
        name: String,
        image: String,
        health: i32,
        attack: i32,
        level: i32,
        experience: i32,
        class: CharacterClass,
    ) -> Character {
        Character {
            name,
            image,
            health,
            attack,
            level,
            experience,
            class,
        }
    }

    pub fn image(&self) -> &str {
        &self.image
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn health(&self) -> i32 {
        self.health
    }

    pub fn attack(&self) -> i32 {
        self.attack
    }

    pub fn level(&self) -> i32 {
        self.level
    }

    pub fn experience(&self) -> i32 {
        self.experience
    }

    pub fn level_up(&mut self) {
        self.level += 1;
    }

    pub fn gain_experience(&mut self, experience: i32) {
        self.experience += experience;
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
    }

    pub fn is_dead(&self) -> bool {
        self.health <= 0
    }

    pub fn random() -> Self {
        available_characters()
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone()
    }
}

pub fn available_characters() -> Vec<Character> {
    vec![
        Character::new(
            String::from("Aria Nightshade"),
            String::from("bg-rogue"),
            85,
            15,
            5,
            2300,
            CharacterClass::Rogue,
        ),
        Character::new(
            String::from("Thorgar Ironbeard"),
            String::from("bg-warrior"),
            120,
            20,
            6,
            3100,
            CharacterClass::Warrior,
        ),
        Character::new(
            String::from("Elara Brightweave"),
            String::from("bg-mage"),
            70,
            25,
            5,
            2800,
            CharacterClass::Mage,
        ),
    ]
}
#[derive(Debug, Clone, Default)]
pub enum CharacterClass {
    #[default]
    Warrior,
    Mage,
    Rogue,
}
