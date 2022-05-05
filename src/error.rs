use lapin::Error as LapinError;
use tokio::sync::mpsc::error::SendError;

#[derive(Debug)]
pub enum Error {
    Rabbit(LapinError),
    Mpsc,
}

impl From<LapinError> for Error {
    fn from(err: LapinError) -> Self {
        Self::Rabbit(err)
    }
}

impl<T> From<SendError<T>> for Error {
    fn from(_err: SendError<T>) -> Self {
        Self::Mpsc
    }
}
