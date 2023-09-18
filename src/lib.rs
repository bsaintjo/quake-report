use nom::bytes::complete::{tag, take_until};

pub mod game;
pub mod kill;
mod time;

/// Take characters from a string until the tag t is found.
/// A convience function to skip over text.
pub(crate) fn chew<'a>(
    t: &'a str,
) -> impl FnMut(&'a str) -> nom::IResult<&'a str, &'a str> {
    move |i: &str| {
        let (i, r) = take_until(t)(i)?;
        let (i, _) = tag(t)(i)?;
        Ok((i, r))
    }
}
