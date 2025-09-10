use super::*;

/// The ReportBody dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ReportBody {
    inner: Any,
}

impl FromVal for ReportBody {
    fn from_val(v: &Any) -> Self {
        ReportBody { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ReportBody {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ReportBody {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ReportBody {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ReportBody {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ReportBody> for Any {
    fn from(s: ReportBody) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ReportBody> for Any {
    fn from(s: &ReportBody) -> Any {
        s.inner.clone()
    }
}
