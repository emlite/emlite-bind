use crate::utils::bind;

pub struct Null {
    inner: emlite::Val,
}

impl Null {
    pub const VALUE: Null = Null {
        inner: emlite::Val::null(),
    };
    pub fn is_null(&self) -> bool {
        true
    }
}

bind!(Null);
