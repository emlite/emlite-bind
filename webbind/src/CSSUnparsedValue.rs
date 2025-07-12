use super::*;

#[derive(Clone, Debug)]
pub struct CSSUnparsedValue {
    inner: CSSStyleValue,
}
impl FromVal for CSSUnparsedValue {
    fn from_val(v: &emlite::Val) -> Self {
        CSSUnparsedValue {
            inner: CSSStyleValue::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CSSUnparsedValue {
    type Target = CSSStyleValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CSSUnparsedValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSUnparsedValue> for emlite::Val {
    fn from(s: CSSUnparsedValue) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSUnparsedValue {
    pub fn new(members: jsbind::Sequence<jsbind::Any>) -> CSSUnparsedValue {
        Self {
            inner: emlite::Val::global("CSSUnparsedValue")
                .new(&[members.into()])
                .as_::<CSSStyleValue>(),
        }
    }
}
impl CSSUnparsedValue {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
