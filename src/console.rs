use std::io::{self, Write};

use crate::{spellbinder::formulate_cmd_conveyance, Cmd, Decks};

pub fn record_words() -> String {
    let mut buffer = String::with_capacity(52);
    let stdin = io::stdin();
    match stdin.read_line(&mut buffer) {
        Ok(_) => buffer.trim().to_string(),
        Err(_) => panic!("console read line error"),
    }
}

pub fn convey_words(decks: &Decks, cmd: Cmd) {
    let conveyance = formulate_cmd_conveyance(decks, cmd);
    writeln!(io::stdout(), "{}", conveyance).unwrap();
}
