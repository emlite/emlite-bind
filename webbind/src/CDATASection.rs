use super::*;

#[derive(Clone, Debug)]
pub struct CDATASection {
    inner: Text,
}
impl FromVal for CDATASection {
    fn from_val(v: &emlite::Val) -> Self {
        CDATASection {
            inner: Text::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CDATASection {
    type Target = Text;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CDATASection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CDATASection> for emlite::Val {
    fn from(s: CDATASection) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
