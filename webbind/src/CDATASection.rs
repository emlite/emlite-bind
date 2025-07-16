use super::*;

/// The CDATASection class.
/// [`CDATASection`](https://developer.mozilla.org/en-US/docs/Web/API/CDATASection)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CDATASection {
    inner: Text,
}
impl FromVal for CDATASection {
    fn from_val(v: &Any) -> Self {
        CDATASection {
            inner: Text::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CDATASection {
    type Target = Text;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CDATASection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CDATASection {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CDATASection {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CDATASection> for Any {
    fn from(s: CDATASection) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CDATASection> for Any {
    fn from(s: &CDATASection) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CDATASection);
