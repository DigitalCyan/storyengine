# StoryEngine
A simple text choice based story engine written in Rust. It allows for creating small simple multiple choice story games using json files.

# How to get started
1. Install `rustup` and `cargo`.
2. Get the latest stable rust toolchain.
3. Clone this repo and `cd` into it.
4. Run `cargo build --release`.
5. Find the exe under `/build/release` and copy it into a separate folder.
6. Copy everything from the `example` directory into that directory too.

# Rules
* `config.json` must exist within the program's directory.
* `config.json` has to have a field `game_name`. This is where you define which game will be loaded from `games` directory.
* The game is a folder in `games` directory and consists of a `start.json` which is the entrypoint of the story and any number of other story nodes defined by other json files.

# Architecture of a story node
This is how a story node should look like.

```json
{
    "title": "MYTH OF ADLEZ",
    "text": "Once upon a time in a land known as Dragdim lived a powerful wizard.",
    "choices": [
        {
            "text": "Vidit Dragdim",
            "to": "visit_dragdim"
        },
        {
            "text": "Jump off a cliff",
            "to": "game_over"
        }
    ]
}
```

`title` - The title of the story node. Is isplayed first.

`text` - The display text of the node. This is where you explain the 
consenquences fo the user's choice.

`choices` - An array of objects that represent different choices available to the player consisting of 2 fields

* `text` - The display text of the choice.

* `to` - Name of the next story file (without the ".json" extension).
