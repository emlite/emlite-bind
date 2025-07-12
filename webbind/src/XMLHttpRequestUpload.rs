use super::*;

#[derive(Clone, Debug)]
pub struct XMLHttpRequestUpload {
    inner: XMLHttpRequestEventTarget,
}
impl FromVal for XMLHttpRequestUpload {
    fn from_val(v: &emlite::Val) -> Self {
        XMLHttpRequestUpload {
            inner: XMLHttpRequestEventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for XMLHttpRequestUpload {
    type Target = XMLHttpRequestEventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XMLHttpRequestUpload {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XMLHttpRequestUpload> for emlite::Val {
    fn from(s: XMLHttpRequestUpload) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
