pub struct Args {
    command: String
}

impl Args {

    pub fn parse(raw_args: Vec<String>) -> Result<Args, String> {
        if raw_args.len() == 0 {
            return Err(String::from("Please pass remote command.")); // TODO more user friendly message.
        } else if raw_args.len() > 1 {
            return Err(String::from("Mainframer supports only 1 argument which is a command that need to be executed."));
        } else {
            return Ok(Args {
                command: raw_args[0].clone()
            });
        }
    }
}

