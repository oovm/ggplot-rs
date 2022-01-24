use csscolorparser::ParseColorError;
use crate::GGError;

impl From<ParseColorError> for GGError {
    fn from(e: ParseColorError) -> Self {
        Self::SyntaxError(e.to_string())
    }
}