use std::collections::HashMap;

use quake_report::game::{parse_games, Game, GameLog};
use serde::{ser::SerializeStruct, Serialize, Serializer};

#[derive(Debug)]
struct GameReport {
    idx: usize,
    total_kills: usize,
    kills: HashMap<String, isize>,
    kills_by_means: HashMap<String, usize>,
}

impl GameReport {
    fn from_game(idx: usize, game: Game) -> Self {
        let mut total_kills = 0;
        let mut kills = HashMap::new();
        let mut kills_by_means = HashMap::new();
        for log in game.logs() {
            if let GameLog::Kill(kill) = log {
                total_kills += 1;

                // If player killed themselves or by environment, subtract 1
                if kill.killer_is_world() || kill.killer_is_victim() {
                    let victim_entry =
                        kills.entry(kill.victim.clone()).or_insert(0);
                    *victim_entry -= 1;
                } else {
                    let killer_entry =
                        kills.entry(kill.killer.clone()).or_insert(0);
                    *killer_entry += 1;
                }
                let kbm =
                    kills_by_means.entry(kill.weapon.clone()).or_insert(0);
                *kbm += 1;
            }
        }
        Self {
            idx,
            total_kills,
            kills,
            kills_by_means,
        }
    }
}

impl Serialize for GameReport {
    fn serialize<S: Serializer>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let mut report = serializer.serialize_struct("GameReport", 4)?;
        report.serialize_field("game", &self.idx)?;
        report.serialize_field("total_kills", &self.total_kills)?;
        report.serialize_field("kills", &self.kills)?;
        report.serialize_field("kills_by_means", &self.kills_by_means)?;
        report.end()
    }
}

fn main() {
    let filename = "extra/qgames.log";
    let file = std::fs::read_to_string(filename).unwrap();
    let games = parse_games(&file).unwrap().1;
    let reports: Vec<_> = games
        .into_iter()
        .enumerate()
        .map(|(idx, game)| GameReport::from_game(idx, game))
        .collect();
    println!("{}", serde_json::to_string_pretty(&reports).unwrap());
}
