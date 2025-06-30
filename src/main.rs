use spellbinder::{memorize_decks, Cmd, CmdConveyer, CmdCrafter, CmdRecorder, CmdConveyor, Decks};

mod spellbinder;
mod danse_macabre;
mod tao_te_ching;
mod console;
mod maya;
mod anger;
mod apathy;
mod emotions;
mod fear;


fn main() {
    let decks = memorize_decks();

    let record_words: CmdRecorder = console::record_words;
    let formulate_cmd: CmdCrafter = spellbinder::try_craft_cmd;
    let convey_words: CmdConveyor<Decks> = console::convey_words;

    let cmd_conveyor = CmdConveyer::<Decks>(decks, record_words, formulate_cmd, convey_words);
    CmdConveyer::exec(&cmd_conveyor);
}
