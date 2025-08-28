use crate::Entry;
#[doc = r"Parsing edited manual pages"]
use winnow::Result;
use winnow::{
    Parser,
    ascii::{line_ending, multispace0, space0, space1, till_line_ending},
    combinator::{cond, delimited, preceded},
    error::ContextError,
    token::{take, take_until},
};

pub fn parse_entry(entry: &mut &str, application_name: &str) -> Option<Entry> {
    _parse_file(entry, application_name).ok()
}

// TODO: Refine this for better error handling we need to give back better responses for bad inputs
fn _parse_file(input: &mut &str, name: &str) -> Result<Entry> {
    let tldr_header = preceded(multispace0, _parse_heading).parse_next(input);
    let tldr_text: Result<Option<&str>> = cond(
        tldr_header.is_ok_and(|v| v.trim_end().eq("TLDR")),
        take_until(0.., "# "),
    )
    .parse_next(input);
    let info_header = _parse_heading.parse_next(input);
    let info_text: Result<Option<&str>> = cond(
        info_header.is_ok_and(|v| v.trim_end().eq("Info")),
        take_until(0.., "# "),
    )
    .parse_next(input);
    let common_uses_header = _parse_heading.parse_next(input);
    let common_uses_text: Result<Option<&str>> = cond(
        common_uses_header.is_ok_and(|v| v.trim_end().eq("Common Uses")),
        take_until(0.., "# "),
    )
    .parse_next(input);
    let resources_header = _parse_heading.parse_next(input);
    let resources_text: Result<Option<&str>> = cond(
        resources_header.is_ok_and(|v| v.trim_end().eq("Resources")),
        take_until(0.., "# "),
    )
    .parse_next(input);
    let _: Result<&str> = take(2u8).parse_next(input); // cleans up the final header which 
    if tldr_text.clone().is_ok_and(|f| f.is_none())
        || info_text.clone().is_ok_and(|f| f.is_none())
        || common_uses_text.clone().is_ok_and(|f| f.is_none())
        || resources_text.clone().is_ok_and(|f| f.is_none())
    {
        return Err(ContextError::new());
    }
    Ok(Entry {
        name: name.to_string(),
        tldr: tldr_text.unwrap().unwrap().trim_end().to_string(),
        info: info_text.unwrap().unwrap().trim_end().to_string(),
        common_uses: common_uses_text.unwrap().unwrap().trim_end().to_string(),
        resources: resources_text.unwrap().unwrap().trim_end().to_string(),
        extra: input.trim_end().to_string(),
    })
}

fn _parse_heading<'s>(input: &mut &'s str) -> Result<&'s str> {
    delimited(
        '#',
        delimited(space1, till_line_ending, space0),
        line_ending,
    )
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    #[doc = "The tests"]
    use super::*;

    #[test]
    fn heading_text() {
        let mut t1 = "# simply \n";
        let p = _parse_heading(&mut t1);
        assert!(p.is_ok());
        assert_eq!(p.unwrap(), "simply ");
        assert_eq!(t1, "");
    }
}
