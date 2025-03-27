use core::fmt;

#[derive(Debug)]
pub struct Error {
    pub cause: String,
}

impl Error {
    #[inline]
    pub fn new<C: Into<String>>(cause: C) -> Self {
        Self {
            cause: cause.into(),
        }
    }
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(
        &self,
        #[expect(clippy::min_ident_chars, reason = "trait impl")] f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "{}", self.cause)
    }
}
