
pub fn parse(text: &str) -> Option<Instruction> {
    let tokens: Vec<String> = text.split_whitespace()
        .map(|s| s.to_owned()).collect();

    if tokens.len() < 2 {
        return None;
    }

    let (targeter_, params_) = tokens.split_at(2);
    let targeter: Vec<String> = targeter_.iter()
        .map(|s| s.to_lowercase()).collect();
    let params: Vec<String> = params_.to_vec();

    if targeter == ["@grahamcofborg", "build"] {
        return Some(Instruction::Build(params));
    }

    return None;
}

#[derive(PartialEq, Debug)]
pub enum Instruction {
    Build(Vec<String>)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_empty() {
        assert_eq!(None, parse(""));
    }


    #[test]
    fn bogus_comment() {
        assert_eq!(None, parse(":) :) :)"));
    }

    #[test]
    fn build_comment() {
        assert_eq!(Some(Instruction::Build(vec![
            String::from("foo"),
            String::from("bar"),
            String::from("baz")
        ])),
                   parse("@GrahamCOfBorg build foo bar baz"));
    }

    #[test]
    fn build_comment_lower() {
        assert_eq!(Some(Instruction::Build(vec![
            String::from("foo"),
            String::from("bar"),
            String::from("baz")
        ])),
                   parse("@grahamcofborg build foo bar baz"));
    }


    #[test]
    fn build_whitespace_disregarded() {
        assert_eq!(Some(Instruction::Build(vec![
            String::from("foo"),
            String::from("bar"),
            String::from("baz")
        ])),
                   parse("


  @grahamcofborg
   build foo


        bar baz

"));
    }



    #[test]
    fn build_comment_lower_package_case_retained() {
        assert_eq!(Some(Instruction::Build(vec![
            String::from("foo"),
            String::from("bar"),
            String::from("baz.Baz")
        ])),
                   parse("@grahamcofborg build foo bar baz.Baz"));
    }

}
