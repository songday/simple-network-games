pub(crate) enum Error {
    Message(String),
}

pub(crate) type Result<T> = core::result::Result<T, Error>;
