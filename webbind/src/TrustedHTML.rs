use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TrustedHTML {
    inner: emlite::Val,
}
impl FromVal for TrustedHTML {
    fn from_val(v: &emlite::Val) -> Self {
        TrustedHTML {
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
impl core::ops::Deref for TrustedHTML {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TrustedHTML {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<TrustedHTML> for emlite::Val {
    fn from(s: TrustedHTML) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl TrustedHTML {
    pub fn to_json(&self) -> jsbind::DOMString {
        self.inner.call("toJSON", &[]).as_::<jsbind::DOMString>()
    }
}
