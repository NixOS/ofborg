
pub fn parse(text: &str) -> Option<Vec<Instruction>> {
    let instructions: Vec<Instruction> = text.lines()
        .map(|s| match parse_line(s) {
            Some(instructions) => instructions,
            None => vec![]
        })
        .fold(vec![], |mut collector, mut inst| {
            collector.append(&mut inst);
            collector
        });

    if instructions.len() == 0 {
        return None;
    } else {
        return Some(instructions)
    }
}

pub fn parse_line(text: &str) -> Option<Vec<Instruction>> {
    let tokens: Vec<String> = text.split_whitespace()
        .map(|s| s.to_owned()).collect();

    if tokens.len() < 2 {
        return None;
    }

    if tokens[0].to_lowercase() != "@grahamcofborg" {
        return None;
    }

    let commands: Vec<&[String]> = tokens
        .split(|token| token.to_lowercase() == "@grahamcofborg")
        .filter(|token| token.len() > 0)
        .collect();

    let mut instructions: Vec<Instruction> = vec![];
    for command in commands {
        let (left, right) = command.split_at(1);
        match left[0].as_ref() {
            "build" => {
                instructions.push(Instruction::Build(Subset::Nixpkgs, right.to_vec()))
            }
            "test" => {
                instructions.push(
                    Instruction::Build(Subset::NixOS,
                                       right
                                       .into_iter()
                                       .map(|attr| format!("tests.{}.x86_64-linux", attr))
                                       .collect()
                    )
                );

            }
            "eval" => {
                instructions.push(Instruction::Eval)
            }
            _ => {}
        }
    }

    return Some(instructions);
}

#[derive(PartialEq, Debug)]
pub enum Instruction {
    Build(Subset, Vec<String>),
    Eval

}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
            parse("/cc @grahamc for ^^
@GrahamcOfBorg eval")
        );
    }

    #[test]
    fn bogus_comment() {
        assert_eq!(None, parse(":) :) :) @grahamcofborg build hi"));
    }

    #[test]
    fn eval_comment() {
        assert_eq!(Some(vec![Instruction::Eval]),
                   parse("@grahamcofborg eval"));
    }

    #[test]
    fn eval_and_build_comment() {
        assert_eq!(Some(vec![
            Instruction::Eval,
            Instruction::Build(Subset::Nixpkgs, vec![
                String::from("foo"),
            ])
        ]),
                   parse("@grahamcofborg eval @grahamcofborg build foo"));
    }

    #[test]
    fn build_and_eval_and_build_comment() {
        assert_eq!(Some(vec![
            Instruction::Build(Subset::Nixpkgs, vec![
                String::from("bar"),
            ]),
            Instruction::Eval,
            Instruction::Build(Subset::Nixpkgs, vec![
                String::from("foo"),
            ])
        ]),
                   parse("
@grahamcofborg build bar
@grahamcofborg eval
@grahamcofborg build foo"));
    }

    #[test]
    fn complex_comment_with_paragraphs() {
        assert_eq!(Some(vec![
            Instruction::Build(Subset::Nixpkgs, vec![
                String::from("bar"),
            ]),
            Instruction::Eval,
            Instruction::Build(Subset::Nixpkgs, vec![
                String::from("foo"),
            ])
        ]),
                   parse("
I like where you're going with this PR, so let's try it out!

@grahamcofborg build bar

I noticed though that the target branch was broken, which should be fixed. Let's eval again.

@grahamcofborg eval

Also, just in case, let's try foo
@grahamcofborg build foo"));
    }


    #[test]
    fn build_and_eval_comment() {
        assert_eq!(Some(vec![
            Instruction::Build(Subset::Nixpkgs, vec![
                String::from("foo"),
            ]),
            Instruction::Eval,
        ]),
                   parse("@grahamcofborg build foo @grahamcofborg eval"));
    }

    #[test]
    fn build_comment() {
        assert_eq!(Some(vec![Instruction::Build(Subset::Nixpkgs, vec![
            String::from("foo"),
            String::from("bar")
        ])]),
                   parse("@GrahamCOfBorg build foo bar

baz"));
    }

    #[test]
    fn test_comment() {
        assert_eq!(Some(vec![Instruction::Build(Subset::NixOS, vec![
            String::from("tests.foo.x86_64-linux"),
            String::from("tests.bar.x86_64-linux"),
            String::from("tests.baz.x86_64-linux")
        ])]),
                   parse("@GrahamCOfBorg test foo bar baz"));
    }

    #[test]
    fn build_comment_newlines() {
        assert_eq!(Some(vec![Instruction::Build(Subset::Nixpkgs, vec![
            String::from("foo"),
            String::from("bar"),
            String::from("baz")
        ])]),
                   parse("@GrahamCOfBorg build foo bar baz"));
    }

    #[test]
    fn build_comment_lower() {
        assert_eq!(Some(vec![Instruction::Build(Subset::Nixpkgs, vec![
            String::from("foo"),
            String::from("bar"),
            String::from("baz")
        ])]),
                   parse("@grahamcofborg build foo bar baz"));
    }

    #[test]
    fn build_comment_lower_package_case_retained() {
        assert_eq!(Some(vec![Instruction::Build(Subset::Nixpkgs, vec![
            String::from("foo"),
            String::from("bar"),
            String::from("baz.Baz")
        ])]),
                   parse("@grahamcofborg build foo bar baz.Baz"));
    }

}
