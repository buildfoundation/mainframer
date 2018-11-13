#[derive(Debug, PartialEq, Eq)]
pub struct Args {
    pub command: String
}

impl Args {
    pub fn parse(raw_args: &[String]) -> Result<Args, String> {
        match raw_args.len() {
            0 => Err(String::from("Please pass remote command.")), // TODO more user friendly message, for now it's consistent with Bash version.
            _ => Ok(Args {
                command: raw_args.join(" ").trim().into()
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_command_passed_as_single_parameter() {
        let raw_args = vec![String::from("test command")];
        assert_eq!(Args::parse(raw_args.as_ref()), Ok(Args { command: String::from("test command") }));
    }

    #[test]
    fn parse_empty() {
        let raw_args: Vec<String> = vec![];
        assert_eq!(Args::parse(raw_args.as_ref()), Err(String::from("Please pass remote command.")));
    }

    #[test]
    fn parse_command_passed_as_multiple_parameters() {
        let raw_args = vec![String::from("test"), String::from("command")];
        assert_eq!(Args::parse(raw_args.as_ref()), Ok(Args { command: String::from("test command") }));
    }
}
