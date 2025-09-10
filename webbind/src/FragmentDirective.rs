use super::*;

/// The FragmentDirective class.
/// [`FragmentDirective`](https://developer.mozilla.org/en-US/docs/Web/API/FragmentDirective)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FragmentDirective {
    inner: Any,
}

impl FromVal for FragmentDirective {
    fn from_val(v: &Any) -> Self {
        FragmentDirective {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for FragmentDirective {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for FragmentDirective {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for FragmentDirective {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for FragmentDirective {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<FragmentDirective> for Any {
    fn from(s: FragmentDirective) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&FragmentDirective> for Any {
    fn from(s: &FragmentDirective) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(FragmentDirective);
