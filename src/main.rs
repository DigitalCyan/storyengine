use std::{io::Error, env::args};

use storyengine::{StoryEngine};

mod storyengine;

fn main() -> Result<(), Error> {

    let mut args = args();

    StoryEngine::new().start()?;
    return Ok(());
}
