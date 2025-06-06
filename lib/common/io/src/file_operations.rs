use std::fs::File;
use std::io::{self, BufReader, BufWriter, Write};
use std::path::Path;
use std::result;

use atomicwrites::{AtomicFile, OverwriteBehavior};
use serde::Serialize;
use serde::de::DeserializeOwned;

pub fn atomic_save_bin<T: Serialize>(path: &Path, object: &T) -> Result<()> {
    let af = AtomicFile::new(path, OverwriteBehavior::AllowOverwrite);
    af.write(|f| {
        let mut writer = BufWriter::new(f);
        bincode::serialize_into(&mut writer, object)?;
        writer.flush().map_err(bincode::Error::from)
    })?;
    Ok(())
}

pub fn atomic_save_json<T: Serialize>(path: &Path, object: &T) -> Result<()> {
    let af = AtomicFile::new(path, OverwriteBehavior::AllowOverwrite);
    af.write(|f| {
        let mut writer = BufWriter::new(f);
        serde_json::to_writer(&mut writer, object)?;
        writer.flush()
    })?;
    Ok(())
}

pub fn read_bin<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let file = File::open(path)?;
    let value = bincode::deserialize_from(BufReader::new(file))?;
    Ok(value)
}

pub fn read_json<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let file = File::open(path)?;
    let value = serde_json::from_reader(BufReader::new(file))?;
    Ok(value)
}

pub type FileOperationResult<T> = Result<T>;
pub type FileStorageError = Error;

pub type Result<T, E = Error> = result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Io(#[from] io::Error),

    #[error("{0}")]
    Bincode(#[from] bincode::ErrorKind),

    #[error("{0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("{0}")]
    Generic(String),
}

impl Error {
    pub fn generic(msg: impl Into<String>) -> Self {
        Self::Generic(msg.into())
    }
}

impl<E> From<atomicwrites::Error<E>> for Error
where
    Self: From<E>,
{
    fn from(err: atomicwrites::Error<E>) -> Self {
        match err {
            atomicwrites::Error::Internal(err) => err.into(),
            atomicwrites::Error::User(err) => err.into(),
        }
    }
}

impl From<bincode::Error> for Error {
    fn from(err: bincode::Error) -> Self {
        Self::Bincode(*err)
    }
}

impl From<Error> for io::Error {
    fn from(err: Error) -> Self {
        match err {
            Error::Io(err) => err,
            Error::Bincode(err) => Self::other(err),
            Error::SerdeJson(err) => Self::other(err),
            Error::Generic(msg) => Self::other(msg),
        }
    }
}
