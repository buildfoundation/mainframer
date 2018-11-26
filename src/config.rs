#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    pub remote_machine: RemoteMachine,
    pub compression: Compression,
}

#[derive(Debug, Eq, PartialEq)]
pub struct RemoteMachine {
    pub host: String,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Compression {
    pub local: i64,
    pub remote: i64,
}
