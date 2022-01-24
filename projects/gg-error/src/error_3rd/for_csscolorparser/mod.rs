use crate::GGError;
use csscolorparser::ParseColorError;

impl From<ParseColorError> for GGError {
    fn from(e: ParseColorError) -> Self {
        Self::SyntaxError(e.to_string())
    }
}
