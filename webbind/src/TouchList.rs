use super::*;

#[derive(Clone, Debug)]
pub struct TouchList {
    inner: emlite::Val,
}
impl FromVal for TouchList {
    fn from_val(v: &emlite::Val) -> Self {
        TouchList {
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
impl std::ops::Deref for TouchList {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for TouchList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<TouchList> for emlite::Val {
    fn from(s: TouchList) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl TouchList {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl TouchList {
    pub fn item(&self, index: u32) -> Touch {
        self.inner.call("item", &[index.into()]).as_::<Touch>()
    }
}
