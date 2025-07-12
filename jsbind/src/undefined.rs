use crate::utils::bind;

pub struct Undefined {
    inner: emlite::Val,
}

impl Undefined {
    pub const VALUE: Undefined = Undefined {
        inner: emlite::Val::undefined(),
    };
    pub fn is_null(&self) -> bool {
        false
    }
    pub fn is_undefined(&self) -> bool {
        true
    }
}

bind!(Undefined);
