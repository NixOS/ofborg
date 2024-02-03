use crate::systems::System;
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
    system(CompleteStr) -> System,
    alt!(
        value!(System::X8664Linux, tag!("x86_64-linux")) |
        value!(System::Aarch64Linux, tag!("aarch64-linux")) |
        value!(System::X8664Darwin, tag!("x86_64-darwin")) |
        value!(System::Aarch64Darwin, tag!("aarch64-darwin"))
    )
);

named!(
    invocation_prefix(CompleteStr) -> CompleteStr,
    alt!(tag_no_case!("@ofborg") | tag_no_case!("@grahamcofborg"))
);

enum Command {
    Eval,
    Build,
    BuildSystem,
    Test,
}

named!(
    command_str(CompleteStr) -> Option<Command>,
    alt!(
        value!(Some(Command::Eval), tag!("eval")) |
        value!(Some(Command::BuildSystem), tag!("build_system")) |
        value!(Some(Command::Build), tag!("build")) |
        value!(Some(Command::Test), tag!("test")) |

        // TODO: Currently keeping previous behaviour of ignoring unknown commands. Maybe
        // it would be better to return an error so that the caller would know one of the
        // commands couldn't be handled?
        value!(None, many_till!(take!(1), invocation_prefix))
    )
);

named!(
    command(CompleteStr) -> Option<Instruction>,
    preceded!(
        ws!(invocation_prefix),
        switch!( ws!(command_str),
            Some(Command::Build) =>
                ws!(do_parse!(
                    pkgs: ws!(many1!(map!(normal_token, |s| s.0.to_owned()))) >>
                    (Some(Instruction::Build(Subset::Nixpkgs, pkgs)))
                )) |
            Some(Command::BuildSystem) =>
                ws!(do_parse!(
                    system: ws!(system) >>
                    pkgs: ws!(many1!(map!(normal_token, |s| s.0.to_owned()))) >>
                    (Some(Instruction::BuildOnSystem(system, Subset::Nixpkgs, pkgs)))
                )) |
            Some(Command::Test) =>
                ws!(do_parse!(
                    tests: ws!(many1!(map!(normal_token, |s| format!("nixosTests.{}", s.0)))) >>
                    (Some(Instruction::Build(Subset::Nixpkgs, tests)))
                )) |
            Some(Command::Eval) => ws!(do_parse!( (Some(Instruction::Eval)) )) |
            None => do_parse!( (None) )
        )
    )
);

named!(
    parse_line_impl(CompleteStr) -> Option<Vec<Instruction>>,
    opt!(
        do_parse!(
            res: ws!(many1!(ws!(command)))
            >> eof!()
            >> (res.into_iter().flatten().collect())
        )
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
    BuildOnSystem(System, Subset, Vec<String>),
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
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
    fn build_system_comment() {
        assert_eq!(
            Some(vec![Instruction::BuildOnSystem(
                System::X8664Linux,
                Subset::Nixpkgs,
                vec![String::from("foo")]
            ),]),
            parse("@ofborg build_system x86_64-linux foo")
        );
    }

    #[test]
    fn unknown_system_comment() {
        assert_eq!(None, parse("@ofborg build_system x86_64-foolinux foo"));
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
