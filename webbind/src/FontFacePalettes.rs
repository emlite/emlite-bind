use super::*;

#[derive(Clone, Debug)]
pub struct FontFacePalettes {
    inner: emlite::Val,
}
impl FromVal for FontFacePalettes {
    fn from_val(v: &emlite::Val) -> Self {
        FontFacePalettes {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for FontFacePalettes {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for FontFacePalettes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<FontFacePalettes> for emlite::Val {
    fn from(s: FontFacePalettes) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FontFacePalettes {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
