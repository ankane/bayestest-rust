use core::error;
use core::fmt;

/// An error.
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    TooManyVariants,
    // TODO better name
    ConversionsParticipants,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::TooManyVariants => f.write_str("too many variants"),
            Error::ConversionsParticipants => {
                f.write_str("conversions cannot be greater than participants")
            }
        }
    }
}
