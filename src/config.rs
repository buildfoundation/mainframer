#[derive(Debug, PartialEq)]
pub struct Config {
    pub remote: Remote,
    pub push: Push,
    pub pull: Pull,
}

#[derive(Debug, PartialEq)]
pub struct Remote {
    pub host: String,
}

#[derive(Debug, PartialEq)]
pub struct Push {
    pub compression: u8,
}

#[derive(Debug, PartialEq)]
pub struct Pull {
    pub compression: u8,
}
