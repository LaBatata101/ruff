use crate::docstrings::google::GOOGLE_SECTIONS;
use crate::docstrings::numpy::NUMPY_SECTIONS;
use crate::docstrings::sections::SectionKind;

#[derive(Copy, Clone, Debug, is_macro::Is)]
pub enum SectionStyle {
    Numpy,
    Google,
}

impl SectionStyle {
    pub fn sections(&self) -> &[SectionKind] {
        match self {
            SectionStyle::Numpy => NUMPY_SECTIONS,
            SectionStyle::Google => GOOGLE_SECTIONS,
        }
    }
}
