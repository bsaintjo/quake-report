use nom::branch::alt;
use nom::bytes::streaming::tag;
use nom::character::streaming::char;
use nom::character::streaming::digit1;
use nom::combinator::recognize;
use nom::multi::many0;
use nom::multi::many1;

/// Each line starts with a timestamp. This timestamp may be preceded by a space.
fn time(i: &str) -> nom::IResult<&str, ()> {
    let (i, _) = many0(char(' '))(i)?;
    let (i, _) = many1(alt((digit1, tag(":"))))(i)?;
    Ok((i, ()))
}

/// Parses timestamp from line, can also deal with error line with
/// two or more timestamps.
pub fn manytime1(i: &str) -> nom::IResult<&str, &str> {
    let (i, r) = recognize(many1(time))(i)?;
    let (i, _) = many0(char(' '))(i)?;
    Ok((i, r))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_manytime() {
        let i = "26  0:00";
        assert_eq!(manytime1(i), Ok(("", "")));
    }
}
