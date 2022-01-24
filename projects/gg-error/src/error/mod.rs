mod error_std;
use std::{
    convert::Infallible,
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    ops::Range,
};

/// All result about tailwind
pub type Result<T> = std::result::Result<T, GGError>;
/// Maybe have ast position
pub type MaybeRanged = Option<Range<usize>>;

/// Error type for all tailwind operators
#[derive(Debug)]
pub enum GGError {
    /// The error type for I/O operations
    IOError(std::io::Error),
    /// The error type which is returned from formatting a message into a
    /// stream.
    FormatError(std::fmt::Error),
    /// The error type which is
    SyntaxError(String),
    /// The error type which is
    TypeMismatch(String),
    /// The error type which is occurred at runtime
    RuntimeError(String),
    /// Runtime error when variable is undefined
    UndefinedVariable {
        /// The name of the undefined variable
        name: String,
    },
    /// A forbidden cst_node encountered
    Unreachable,
    // #[error(transparent)]
    // UnknownError(#[from] anyhow::Error),
}

macro_rules! error_msg {
    ($name:ident => $t:ident) => {
        /// Constructor of [`NoteErrorKind::$t`]
        pub fn $name(msg: impl Into<String>) -> Self {
            GGError::$t(msg.into())
        }
    };
    ($($name:ident => $t:ident),+ $(,)?) => (
        impl GGError { $(error_msg!($name=>$t);)+ }
    );
}

error_msg![
    syntax_error => SyntaxError,
    type_mismatch => TypeMismatch,
    runtime_error => RuntimeError,
];

impl Error for GGError {}

impl Display for GGError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::IOError(e) => {
                write!(f, "{}", e)
            }
            Self::FormatError(e) => {
                write!(f, "{}", e)
            }
            Self::SyntaxError(msg) => {
                f.write_str("SyntaxError: ")?;
                f.write_str(msg)
            }
            Self::TypeMismatch(msg) => {
                f.write_str("TypeError: ")?;
                f.write_str(msg)
            }
            Self::RuntimeError(msg) => {
                f.write_str("RuntimeError: ")?;
                f.write_str(msg)
            }
            Self::UndefinedVariable { name } => {
                write!(f, "RuntimeError: Variable {} not found in scope", name)
            }
            Self::Unreachable => {
                f.write_str("InternalError: ")?;
                f.write_str("Entered unreachable code!")
            }
        }
    }
}
