use super::*;

#[derive(Clone, Debug)]
pub struct BarProp {
    inner: emlite::Val,
}
impl FromVal for BarProp {
    fn from_val(v: &emlite::Val) -> Self {
        BarProp {
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
impl std::ops::Deref for BarProp {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for BarProp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<BarProp> for emlite::Val {
    fn from(s: BarProp) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl BarProp {
    pub fn visible(&self) -> bool {
        self.inner.get("visible").as_::<bool>()
    }
}
