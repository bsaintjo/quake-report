use nom::{
    bytes::complete::tag,
    character::complete::{char, newline},
};
use nom::{bytes::complete::take_until, character::complete::alphanumeric1, sequence::delimited};

#[derive(PartialEq, Debug)]
struct Kill {
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

    fn kill_parser(i: &str) -> nom::IResult<&str, Kill> {
        let (i, _) = chew("Kill: ")(i)?;
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

fn chew<'a>(t: &'a str) -> impl FnMut(&'a str) -> nom::IResult<&'a str, &'a str> {
    move |i: &str| {
        let (i, _) = take_until(t)(i)?;
        let (i, _) = tag(t)(i)?;
        Ok((i, i))
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
        assert_eq!(Kill::kill_parser(i), Ok(("", answer)));
    }
}
