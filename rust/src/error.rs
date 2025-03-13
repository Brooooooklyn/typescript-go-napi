use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("cannot get effective type roots without a config file path or current directory")]
    NoConfigFileOrCurrentDirectory,
}
