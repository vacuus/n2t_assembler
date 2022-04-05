use crate::types::{Pars, Label};


pub fn parse_source(source: &str) -> (impl Iterator<Item = Pars>, Vec<Label>) {
    // apparently, `!` can't coerce to an opaque type, which is a shame
    (::core::iter::empty(), todo!())
}
