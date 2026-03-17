use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::multi::many1;
use nom::sequence::preceded;
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

fn is_not_whitespace(c: char) -> bool {
    !c.is_ascii_whitespace()
}

fn normal_token(input: &str) -> IResult<&str, String> {
    let (input, tokens) =
        many1(nom::character::complete::satisfy(is_not_whitespace)).parse(input)?;

    let s: String = tokens.into_iter().collect();
    if s.eq_ignore_ascii_case("@grahamcofborg") {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )))
    } else {
        Ok((input, s))
    }
}

fn parse_command(input: &str) -> IResult<&str, Instruction> {
    alt((
        preceded(
            preceded(multispace0, tag("build")),
            preceded(
                multispace1,
                map(many1(preceded(multispace0, normal_token)), |pkgs| {
                    Instruction::Build(Subset::Nixpkgs, pkgs)
                }),
            ),
        ),
        preceded(
            preceded(multispace0, tag("test")),
            preceded(
                multispace1,
                map(many1(preceded(multispace0, normal_token)), |tokens| {
                    let tests: Vec<String> = tokens
                        .into_iter()
                        .map(|s| format!("nixosTests.{}", s))
                        .collect();
                    Instruction::Build(Subset::Nixpkgs, tests)
                }),
            ),
        ),
        preceded(multispace0, map(tag("eval"), |_| Instruction::Eval)),
    ))
    .parse(input)
}

fn parse_line_impl(input: &str) -> IResult<&str, Option<Vec<Instruction>>> {
    let (input, _) = multispace0.parse(input)?;

    let result = map(
        many1(preceded(
            multispace0,
            preceded(
                alt((tag_no_case("@grahamcofborg"), tag_no_case("@ofborg"))),
                preceded(multispace1, parse_command),
            ),
        )),
        |instructions| Some(instructions),
    )
    .parse(input);

    match result {
        Ok((rest, instructions)) => Ok((rest, instructions)),
        Err(_e) => Ok((input, None)),
    }
}

pub fn parse_line(text: &str) -> Option<Vec<Instruction>> {
    match parse_line_impl(text) {
        Ok((_, res)) => res,
        Err(e) => {
            warn!("Failed parsing string '{}': result was {:?}", text, e);
            None
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Instruction {
    Build(Subset, Vec<String>),
    Eval,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Clone)]
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
