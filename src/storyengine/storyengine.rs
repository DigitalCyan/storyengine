use serde::{Serialize, Deserialize};
use serde_json::from_str;
use std::{collections::HashMap, fs::{self, File}, io::{Error, Read}, path::Path, ffi::{OsString} };

use super::get_usize_from_console;

const GAMES_DIR: &'static str = "games";

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Story {
    pub title: String,
    pub text: String,
    pub choices: Vec<Choice>
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Choice {
    pub text: String,
    pub to: String,
}

pub struct StoryEngine {
    story_map: HashMap<String, Story>,
    config: Config
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Config {
    game_name: String
}

impl StoryEngine {
    pub fn new() -> Self {
        return Self {
            story_map: HashMap::new(),
            config: Config { game_name: String::new() }
        };
    }

    pub fn start(&mut self) -> Result<(), Error> {
        self.load_config()?;

        let path = Path::new(GAMES_DIR);
        let path = path.join(&self.config.game_name);

        let entries = fs::read_dir(path)?;

        for entry in entries {
            let entry = entry?;

            let metadata = entry.metadata()?;
            
            if metadata.is_dir() {
                continue;
            }

            self.load(entry.file_name())?;
        }

        self.mainloop()?;

        return Ok(());
    }

    pub fn load(&mut self, file_name: OsString) -> Result<(), Error> {

        let path = Path::new(GAMES_DIR);
        let path = path.join(self.config.game_name.clone());
        let path = path.join(file_name.clone());


        let mut buf: String = String::new();
        let mut file = File::open(path)?;
        file.read_to_string(&mut buf)?;

        let story: Story = from_str(buf.as_str())?;

        let (name, _) = file_name.to_str().unwrap().split_at(file_name.len() - 5);

        self.story_map.insert(name.to_string(), story);

        return Ok(());
    }

    pub fn mainloop(&mut self) -> Result<(), Error> {
        let start = self.story_map.get(&"start".to_string()).unwrap().clone();

        self.execute_story(start)?;

        return Ok(());
    }

    pub fn execute_story(&mut self, story: Story) -> Result<(), Error> {
        println!("{}", story.title);
        println!("{}", story.text);

        let choice = self.choice_selection(&story.choices);

        let choice = story.choices[choice - 1].clone();

        let Some(story) = self.story_map.get(&choice.to) else {
            panic!("Could not find {}.", choice.to);
        };

        self.execute_story(story.clone())?;

        return Ok(());
    }

    pub fn choice_selection(&mut self, choices: &Vec<Choice>) -> usize {
        let mut id = 1;

        for choice in choices.iter() {
            println!("{}) {}", id, choice.text);
            id += 1;
        }

        loop {
            let n = get_usize_from_console();
            if n > 0 && n < choices.len() + 1 {
                return n;
            }
        }
    }

    pub fn load_config(&mut self) -> Result<(), Error> {
        let mut file = File::open("config.json")?;

        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        self.config = from_str(buf.as_str())?;

        return Ok(());
    }
}