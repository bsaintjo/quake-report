use nom::{
    bytes::complete::{tag, take_until},
    character::complete::newline,
};

use crate::{chew, time::manytime1};

#[derive(PartialEq, Debug, Clone)]
pub struct Kill {
    pub killer: String,
    pub victim: String,
    pub weapon: String,
}

impl Kill {
    fn new(killer: String, victim: String, weapon: String) -> Kill {
        Kill {
            killer,
            victim,
            weapon,
        }
    }

    /// Killed via environment, such as falling into lava.
    pub fn killer_is_world(&self) -> bool {
        self.killer == "<world>"
    }

    /// A player can kill themselves with rocket launcher splash, for example.
    /// This is recorded differently than a player being killed by <world>
    pub fn killer_is_victim(&self) -> bool {
        self.killer == self.victim
    }

    /// Parses a kill line from a game log.
    ///
    /// Format is roughly:
    /// {timestamp} Kill: {coordinates?}: {killer} killed {victim_id} by
    /// {weapon}\n
    pub(crate) fn parse_kill(i: &str) -> nom::IResult<&str, Kill> {
        let (i, _) = manytime1(i)?;
        let (i, _) = tag("Kill: ")(i)?;
        let (i, _) = chew(": ")(i)?;
        let (i, killer) = take_until(" ")(i)?;
        let (i, _) = tag(" killed ")(i)?;
        let (i, victim) = take_until(" ")(i)?;
        let (i, _) = tag(" by ")(i)?;
        let (i, weapon) = take_until("\n")(i)?;
        let (i, _) = newline(i)?;
        Ok((
            i,
            Kill::new(
                killer.to_string(),
                victim.to_string(),
                weapon.to_string(),
            ),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_kill_parser() {
        let i = " 21:42 Kill: 1022 2 22: <world> killed Isgalamido by MOD_TRIGGER_HURT\n";
        let answer = Kill::new(
            "<world>".to_string(),
            "Isgalamido".to_string(),
            "MOD_TRIGGER_HURT".to_string(),
        );
        assert_eq!(Kill::parse_kill(i), Ok(("", answer)));
    }
}
