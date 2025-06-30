use chrono::{Datelike, Local, NaiveDate};
use rand::Rng;

use crate::{
    anger, apathy, danse_macabre::{memorize_death_danse, DanseMacabreCard, DanseMacabreCardKey}, emotions, fear, maya::{LongDate, MayaEpoch, RoundDate}, tao_te_ching::{memorize_tao_te_ching, TaoTeChingKey}
};

pub struct Decks {
    pub tao: Vec<String>,
    pub danse: Vec<DanseMacabreCard>,
    pub anger: Vec<String>,
    pub apathy: Vec<String>,
    pub emotions: Vec<String>,
    pub fear: Vec<String>,
}

pub enum Cmd {
    Tao(TaoTeChingKey),
    DanseMacabre(DanseMacabreCardKey),
    Maya(NaiveDate),
    Anger,
    Apathy,
    Emotions,
    Fear,
}

pub type CmdRecorder = fn() -> String;
pub type CmdCrafter = fn(user_input: String) -> Option<Cmd>;
pub type CmdConveyor<T> = fn(user_data: &T, cmd: Cmd) -> ();

pub struct CmdConveyer<T>(
    pub T,
    pub CmdRecorder,
    pub CmdCrafter,
    pub CmdConveyor<T>,
);

impl<T> CmdConveyer<T> {
    pub fn exec(&self) {
        let (user_data, record_event, craft_cmd, convey_cmd) =
            (&self.0, &self.1, &self.2, &self.3);

        loop {
            let user_input = record_event();
            match craft_cmd(user_input) {
                Some(cmd) => convey_cmd(user_data, cmd),
                None => continue,
            };
        }
    }
}

fn select_tao_chapter(user_input: &str) -> TaoTeChingKey {
    match TaoTeChingKey::try_from(user_input) {
        Ok(tao_key) => tao_key,
        Err(_) => TaoTeChingKey::select_random(),
    }
}

fn select_danse_card(user_input: &str) -> DanseMacabreCardKey {
    match user_input.parse::<usize>() {
        Ok(key @ 1..=49) => DanseMacabreCardKey::try_from(key - 1).unwrap(),
        _ => match DanseMacabreCardKey::try_from(user_input) {
            Ok(card) => card,
            Err(_) => DanseMacabreCardKey::select_random(),
        },
    }
}

fn select_random(emotion: &[String]) -> String {
    let mut rng = rand::rng();
    let len = emotion.len();
    let random = rng.random_range(0..len);
    emotion[*&random].clone()
}

impl Cmd {
    fn craft_tao(user_input: &[&str]) -> Cmd {
        let chapter = match user_input {
            [head, _tail @ ..] => select_tao_chapter(head),
            _ => TaoTeChingKey::select_random(),
        };
        Cmd::Tao(chapter)
    }

    fn craft_danse(user_input: &[&str]) -> Cmd {
        let user_input = vec![user_input.join(" ")];
        let card = match &user_input[..] {
            [head, _tail @ ..] => select_danse_card(head),
            _ => DanseMacabreCardKey::select_random(),
        };
        Cmd::DanseMacabre(card)
    }

    fn craft_maya(user_input: &[&str]) -> Cmd {
        let user_input = user_input.join(" ");
        match try_parse_date(&user_input) {
            Some(date) => Cmd::Maya(date),
            None => Cmd::Maya(Local::now().date_naive()),
        }
    }
}

fn craft_maya_conveyance(date: NaiveDate) -> String {
    let long = LongDate::new(
        MayaEpoch::BC3114,
        date.year(),
        date.month0() as i32 + 1,
        date.day0() as i32 + 1,
    );
    let round = RoundDate::from(&long);

    let msg = vec![
        format!("{:#?}", long),
        format!("{:#?}", round),
        format!("{}", round.0 .1.detailed_meaning()),
        format!("{}", round.0 .1.meaning()),
        format!("{}", round.1 .1.meaning()),
    ]
    .join("\n");
    msg
}

fn craft_death_danse_conveyance(decks: &Decks, key: DanseMacabreCardKey) -> String {
    let card = &decks.danse[key as usize];
    let msg = vec![
        card.key.to_string(),
        card.german_name.clone(),
        card.description.clone(),
        String::from(""),
        card.bible_verse.clone(),
        String::from(""),
        card.quatrain.clone(),
        String::from(""),
        card.bible_verse_eng.clone(),
        String::from(""),
        card.quatrain_eng.clone(),
    ]
    .join("\n");
    msg
}

fn try_parse_date(user_input: &str) -> Option<NaiveDate> {
    let formats = [
        "%Y-%m-%d",  // 2023-04-05
        "%Y%m%d",    // 20230405
        "%m/%d/%Y",  // 04/05/2023 (US)
        "%Y/%m/%d",  // 2023/04/05
        "%b %d, %Y", // Apr 05, 2023
        "%B %d, %Y", // April 05, 2023
        "%d %b %Y",  // 05 Apr 2023
        "%d %B %Y",  // 05 April 2023
        "%Y %m %d",  // 2025 04 24
        "%m %d %Y",  // 04 24 2025
    ];
    for fmt in formats.iter() {
        if let Ok(date) = NaiveDate::parse_from_str(user_input, fmt) {
            return Some(date);
        }
    }
    None
}

pub fn formulate_cmd_conveyance(decks: &Decks, cmd: Cmd) -> String {
    match cmd {
        Cmd::Tao(key) => decks.tao[key as usize].clone(),
        Cmd::DanseMacabre(key) => craft_death_danse_conveyance(decks, key),
        Cmd::Maya(date) => craft_maya_conveyance(date),
        Cmd::Anger => select_random(&decks.anger),
        Cmd::Apathy => select_random(&decks.apathy),
        Cmd::Emotions => select_random(&decks.emotions),
        Cmd::Fear => select_random(&decks.fear),
    }
}

pub fn try_craft_cmd(user_input: String) -> Option<Cmd> {
    let cmd: Vec<&str> = user_input.split(' ').collect();
    let cmd = &cmd[..];

    match cmd {
        | ["./tao", user_input @ ..] => Some(Cmd::craft_tao(user_input)),
        | ["./dansemacabre", user_input @ ..]
        | ["./danse_macabre", user_input @ ..]
        | ["./danse", user_input @ ..]
        | ["./dance", user_input @ ..]
        | ["./death", user_input @ ..]
        | ["./die", user_input @ ..] => Some(Cmd::craft_danse(user_input)),
        | ["./maya", user_input @ ..] => Some(Cmd::craft_maya(user_input)),
        | ["./anger", _user_input @ ..] => Some(Cmd::Anger),
        | ["./apathy", _user_input @ ..] => Some(Cmd::Apathy),
        | ["./emotions", _user_input @ ..] => Some(Cmd::Emotions),
        | ["./fear", _user_input @ ..] => Some(Cmd::Fear),
        _ => None,
    }
}

fn memorize_emotion(data: &str) -> Vec<String> {
    let data = data.as_bytes();
    let mut csv = csv::Reader::from_reader(data);
    let mut emotion = Vec::with_capacity(52);

    for record in csv.records() {
        let record = record.unwrap();
        let quote = record[0].to_owned();
        let author = record[1].to_owned();
        let source = record[2].to_owned();
        let unit = vec![
            quote,
            format!("- {} | {}", author, source)
        ].join("\n");
        emotion.push(unit);

    }

    emotion
}

pub fn memorize_decks() -> Decks {
    let tao = memorize_tao_te_ching();
    let danse = memorize_death_danse();
    let anger = memorize_emotion(&anger::record());
    let apathy = memorize_emotion(&apathy::record());
    let emotions = memorize_emotion(&emotions::record());
    let fear = memorize_emotion(&fear::record());
    let decks = Decks { tao, danse, anger, apathy , emotions, fear };
    decks
}