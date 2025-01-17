use nom::types::CompleteStr;
use tracing::warn;

pub fn parse(text: &str) -> Option<Vec<Instruction>> {
    let instructions: Vec<Instruction> = text
        .lines()
        .flat_map(|s| match parse_line(s) {
            Some(instructions) => instructions.into_iter(),
            None => Vec::new().into_iter(),
        })
        .collect();

    if instructions.is_empty() {
        None
    } else {
        Some(instructions)
    }
}

named!(
    normal_token(CompleteStr) -> CompleteStr,
    verify!(
        take_while1!(|c: char| c.is_ascii_graphic()),
        |s: CompleteStr| !s.0.eq_ignore_ascii_case("@grahamcofborg")
    )
);
named!(
    parse_line_impl(CompleteStr) -> Option<Vec<Instruction>>,
    alt!(
        do_parse!(
            res: ws!(many1!(ws!(preceded!(
                alt!(tag_no_case!("@grahamcofborg") | tag_no_case!("@ofborg")),
                alt!(
                    ws!(do_parse!(
                    tag!("build") >>
                    pkgs: ws!(many1!(map!(normal_token, |s| s.0.to_owned()))) >>
                    (Some(Instruction::Build(Subset::Nixpkgs, pkgs)))
                )) |
                ws!(do_parse!(
                    tag!("test") >>
                    tests: ws!(many1!(map!(normal_token, |s| format!("nixosTests.{}", s.0)))) >>
                    (Some(Instruction::Build(Subset::Nixpkgs, tests)))
                )) |
                value!(Some(Instruction::Eval), tag!("eval")) |
                // TODO: Currently keeping previous behaviour of ignoring unknown commands. Maybe
                // it would be better to return an error so that the caller would know one of the
                // commands couldn't be handled?
                value!(None, many_till!(take!(1), tag_no_case!("@grahamcofborg")))
                )
            )))) >> eof!()
                >> (Some(res.into_iter().flatten().collect()))
        ) | value!(None)
    )
);

pub fn parse_line(text: &str) -> Option<Vec<Instruction>> {
    match parse_line_impl(CompleteStr(text)) {
        Ok((_, res)) => res,
        Err(e) => {
            // This should likely never happen thanks to the | value!(None), but well...
            warn!("Failed parsing string ‘{}’: result was {:?}", text, e);
            None
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Instruction {
    Build(Subset, Vec<String>),
    Eval,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub enum Subset {
    Nixpkgs,
    NixOS,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_empty() {
        assert_eq!(None, parse(""));
    }

    #[test]
    fn valid_trailing_instruction() {
        assert_eq!(
            Some(vec![Instruction::Eval]),
            parse(
                "/cc @grahamc for ^^
@GrahamcOfBorg eval",
            )
        );
    }

    #[test]
    fn bogus_comment() {
        assert_eq!(None, parse(":) :) :) @grahamcofborg build hi"));
    }

    #[test]
    fn bogus_build_comment_empty_list() {
        assert_eq!(None, parse("@grahamcofborg build"));
    }

    #[test]
    fn eval_comment() {
        assert_eq!(Some(vec![Instruction::Eval]), parse("@grahamcofborg eval"));
    }

    #[test]
    fn eval_and_build_comment() {
        assert_eq!(
            Some(vec![
                Instruction::Eval,
                Instruction::Build(Subset::Nixpkgs, vec![String::from("foo")]),
            ]),
            parse("@grahamcofborg eval @grahamcofborg build foo")
        );
    }

    #[test]
    fn build_and_eval_and_build_comment() {
        assert_eq!(
            Some(vec![
                Instruction::Build(Subset::Nixpkgs, vec![String::from("bar")]),
                Instruction::Eval,
                Instruction::Build(Subset::Nixpkgs, vec![String::from("foo")]),
            ]),
            parse(
                "
@grahamcofborg build bar
@ofborg eval
@grahamcofborg build foo",
            )
        );
    }

    #[test]
    fn complex_comment_with_paragraphs() {
        assert_eq!(
            Some(vec![
                Instruction::Build(Subset::Nixpkgs, vec![String::from("bar")]),
                Instruction::Eval,
                Instruction::Build(Subset::Nixpkgs, vec![String::from("foo")]),
            ]),
            parse(
                "
I like where you're going with this PR, so let's try it out!

@grahamcofborg build bar

I noticed though that the target branch was broken, which should be fixed. Let's eval again.

@grahamcofborg eval

Also, just in case, let's try foo
@grahamcofborg build foo",
            )
        );
    }

    #[test]
    fn build_and_eval_comment() {
        assert_eq!(
            Some(vec![
                Instruction::Build(Subset::Nixpkgs, vec![String::from("foo")]),
                Instruction::Eval,
            ]),
            parse("@grahamcofborg build foo @grahamcofborg eval")
        );
    }

    #[test]
    fn build_comment() {
        assert_eq!(
            Some(vec![Instruction::Build(
                Subset::Nixpkgs,
                vec![String::from("foo"), String::from("bar")]
            ),]),
            parse(
                "@OfBorg build foo bar

baz",
            )
        );
    }

    #[test]
    fn test_comment() {
        assert_eq!(
            Some(vec![Instruction::Build(
                Subset::Nixpkgs,
                vec![
                    String::from("nixosTests.foo"),
                    String::from("nixosTests.bar"),
                    String::from("nixosTests.baz"),
                ]
            ),]),
            parse("@GrahamCOfBorg test foo bar baz")
        );
    }

    #[test]
    fn build_comment_newlines() {
        assert_eq!(
            Some(vec![Instruction::Build(
                Subset::Nixpkgs,
                vec![
                    String::from("foo"),
                    String::from("bar"),
                    String::from("baz"),
                ]
            ),]),
            parse("@OfBorg build foo bar baz")
        );
    }

    #[test]
    fn build_comment_lower() {
        assert_eq!(
            Some(vec![Instruction::Build(
                Subset::Nixpkgs,
                vec![
                    String::from("foo"),
                    String::from("bar"),
                    String::from("baz"),
                ]
            ),]),
            parse("@grahamcofborg build foo bar baz")
        );
    }

    #[test]
    fn build_comment_lower_package_case_retained() {
        assert_eq!(
            Some(vec![Instruction::Build(
                Subset::Nixpkgs,
                vec![
                    String::from("foo"),
                    String::from("bar"),
                    String::from("baz.Baz"),
                ]
            ),]),
            parse("@ofborg build foo bar baz.Baz")
        );
    }
}
