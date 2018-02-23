#[derive(Debug, PartialEq, Eq)]
pub struct Args {
    pub command: String
}

impl Args {
    pub fn parse(raw_args: Vec<String>) -> Result<Args, String> {
        if raw_args.len() == 0 {
            return Err(String::from("Please pass remote command.")); // TODO more user friendly message, for now it's consistent with Bash version.
        } else if raw_args.len() > 1 {
            return Err(String::from("Mainframer supports only 1 argument which is a command that need to be executed."));
        } else {
            return Ok(Args {
                command: raw_args[0].clone()
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ok() {
        let raw_args = vec![String::from("test command")];
        assert_eq!(Args::parse(raw_args), Ok(Args { command: String::from("test command") }));
    }

    #[test]
    fn parse_empty() {
        let raw_args: Vec<String> = vec![];
        assert_eq!(Args::parse(raw_args), Err(String::from("Please pass remote command.")));
    }

    #[test]
    fn parse_more_than_1_arg() {
        let raw_args = vec![String::from("test"), String::from("command")];
        assert_eq!(Args::parse(raw_args), Err(String::from("Mainframer supports only 1 argument which is a command that need to be executed.")));
    }
}