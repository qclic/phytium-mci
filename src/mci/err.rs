use super::RegError;

#[derive(Debug)]
pub enum FsdifError {
    Timeout,
    NotInit,
    ShortBuf,
    NotSupport,
    InvalidState,
    TransTimeout,
    CmdTimeout,
    NoCard,
    Busy,
    DmaBufUnalign,
    InvalidTiming,
}

impl RegError for FsdifError {
    fn timeout() -> Self {
        FsdifError::Timeout
    }
}

pub type FsdifResult<T=()> = Result<T, FsdifError>;