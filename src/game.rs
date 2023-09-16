use nom::{
    branch::alt, bytes::complete::{take_until, take_while1}, character::complete::{char}, combinator::{opt, eof}, Parser,
};
use nom::{
    bytes::complete::tag,
    combinator::not,
    multi::{many0, many1},
};

use crate::{kill::{chew, Kill}, time};

#[derive(PartialEq, Debug, Clone)]
enum GameLog {
    Kill(Kill),
    Ignored,
}

fn parse_game_log(i: &str) -> nom::IResult<&str, GameLog> {
    alt((Kill::parse_kill.map(GameLog::Kill), parse_ignored))(i)
}

fn parse_ignored(i: &str) -> nom::IResult<&str, GameLog> {
    let (i, _) = time::manytime1(i)?;
    let (i, _) = not(tag("ShutdownGame:\n").or(tag("-")))(i)?;
    let (i, _) = take_until("\n")(i)?;
    let (i, _) = char('\n')(i)?;
    Ok((i, GameLog::Ignored))
}

#[derive(PartialEq, Debug)]
struct Game {
    logs: Vec<GameLog>,
}

impl Game {
    fn parse_game(i: &str) -> nom::IResult<&str, Self> {
        let (i, _) = parse_game_start(i)?;
        let (i, logs) = many0(parse_game_log)(i)?;
        let (i, _) = parse_game_end(i)?;
        Ok((i, Game { logs }))
    }
}

fn parse_games(i: &str) -> nom::IResult<&str, Vec<Game>> {
    let (i, _) = opt(parse_shutdown_line)(i)?;
    many1(Game::parse_game)(i)
}

fn parse_game_start(i: &str) -> nom::IResult<&str, ()> {
    let (i, _) = time::manytime1(i)?;
    let (i, _) = tag("InitGame: ")(i)?;
    let (i, _) = chew("\n")(i)?;
    Ok((i, ()))
}

fn parse_shutdown(i: &str) -> nom::IResult<&str, ()> {
    let (i, _) = time::manytime1(i)?;
    let (i, _) = tag("ShutdownGame:\n")(i)?;
    let (i, _) = parse_shutdown_line1(i)?;
    Ok((i, ()))
}

fn parse_shutdown_line(i: &str) -> nom::IResult<&str, ()> {
    let (i, _) = time::manytime1(i)?;
    let (i, _) = take_while1(|c| c == '-')(i)?;
    let (i, _) = alt((tag("\n"), eof))(i)?;
    Ok((i, ()))
}

fn parse_shutdown_line1(i: &str) -> nom::IResult<&str, ()> {
    let (i, _) = parse_shutdown_line(i)?;
    let (i, _) = opt(parse_shutdown_line)(i)?;
    Ok((i, ()))
}

fn parse_game_end(i: &str) -> nom::IResult<&str, ()> {
    alt((parse_shutdown, parse_shutdown_line))(i)
}

#[cfg(test)]
mod test_game {
    use super::*;

    #[test]
    fn test_shutdown_lines1() {
        let i = "26  0:00 ------------------------------------------------------------\n";
        assert_eq!(parse_shutdown_line1(i), Ok(("", ())));

        let i = "20:37 ------------------------------------------------------------\n";
        assert_eq!(parse_shutdown_line1(i), Ok(("", ())));
    }

    #[test]
    fn test_shutdown() {
        let i = r"12:13 ShutdownGame:
 12:13 ------------------------------------------------------------
 12:13 ------------------------------------------------------------
        ";
        assert_eq!(parse_shutdown(i), Ok(("", ())));
    }

    #[test]
    fn test_parse_game() {
        let i = r" 0:00 ------------------------------------------------------------
 0:00 InitGame: \sv_floodProtect\1\sv_maxPing\0\sv_minPing\0\sv_maxRate\10000\sv_minRate\0\sv_hostname\Code Miner Server\g_gametype\0\sv_privateClients\2\sv_maxclients\16\sv_allowDownload\0\dmflags\0\fraglimit\20\timelimit\15\g_maxGameClients\0\capturelimit\8\version\ioq3 1.36 linux-x86_64 Apr 12 2009\protocol\68\mapname\q3dm17\gamename\baseq3\g_needpass\0
15:00 Exit: Timelimit hit.
20:34 ClientConnect: 2
20:34 ClientUserinfoChanged: 2 n\Isgalamido\t\0\model\xian/default\hmodel\xian/default\g_redteam\\g_blueteam\\c1\4\c2\5\hc\100\w\0\l\0\tt\0\tl\0
20:37 ClientUserinfoChanged: 2 n\Isgalamido\t\0\model\uriel/zael\hmodel\uriel/zael\g_redteam\\g_blueteam\\c1\5\c2\5\hc\100\w\0\l\0\tt\0\tl\0
20:37 ClientBegin: 2
20:37 ShutdownGame:
20:37 ------------------------------------------------------------
20:37 ------------------------------------------------------------
";
        assert_eq!(
            parse_games(i),
            Ok((
                "",
                vec![Game {
                    logs: vec![GameLog::Ignored; 5]
                }]
            ))
        );
    }

    #[test]
    fn test_full() {
        let i = include_str!("../extra/qgames.log");
        let (_i, games) = parse_games(i).unwrap();
        assert_eq!(games.len(), 21);
    }
}
