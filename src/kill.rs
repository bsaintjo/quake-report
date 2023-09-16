use nom::bytes::complete::take_until;
use nom::{bytes::complete::tag, character::complete::newline};

use crate::time::manytime1;

// means of death
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
enum MeansOfDeath {
    MOD_UNKNOWN,
    MOD_SHOTGUN,
    MOD_GAUNTLET,
    MOD_MACHINEGUN,
    MOD_GRENADE,
    MOD_GRENADE_SPLASH,
    MOD_ROCKET,
    MOD_ROCKET_SPLASH,
    MOD_PLASMA,
    MOD_PLASMA_SPLASH,
    MOD_RAILGUN,
    MOD_LIGHTNING,
    MOD_BFG,
    MOD_BFG_SPLASH,
    MOD_WATER,
    MOD_SLIME,
    MOD_LAVA,
    MOD_CRUSH,
    MOD_TELEFRAG,
    MOD_FALLING,
    MOD_SUICIDE,
    MOD_TARGET_LASER,
    MOD_TRIGGER_HURT,
    MISSIONPACK,
    MOD_NAIL,
    MOD_CHAINGUN,
    MOD_PROXIMITY_MINE,
    MOD_KAMIKAZE,
    MOD_JUICED,
    MOD_GRAPPLE,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Kill {
    killer: String,
    victim: String,
    weapon: String,
}

impl Kill {
    fn new(killer: String, victim: String, weapon: String) -> Kill {
        Kill {
            killer,
            victim,
            weapon,
        }
    }

    pub fn parse_kill(i: &str) -> nom::IResult<&str, Kill> {
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
            Kill {
                killer: killer.to_string(),
                victim: victim.to_string(),
                weapon: weapon.to_string(),
            },
        ))
    }
}

pub fn chew<'a>(t: &'a str) -> impl FnMut(&'a str) -> nom::IResult<&'a str, &'a str> {
    move |i: &str| {
        let (i, r) = take_until(t)(i)?;
        let (i, _) = tag(t)(i)?;
        Ok((i, r))
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
