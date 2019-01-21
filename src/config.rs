#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    pub remote: Remote,
    pub push: Push,
    pub pull: Pull,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Remote {
    pub host: String,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Push {
    pub compression: u8,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Pull {
    pub compression: u8,
}
