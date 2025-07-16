use super::*;

/// The ViewTransitionTypeSet class.
/// [`ViewTransitionTypeSet`](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransitionTypeSet)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ViewTransitionTypeSet {
    inner: Any,
}
impl FromVal for ViewTransitionTypeSet {
    fn from_val(v: &Any) -> Self {
        ViewTransitionTypeSet {
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
impl core::ops::Deref for ViewTransitionTypeSet {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ViewTransitionTypeSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ViewTransitionTypeSet {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ViewTransitionTypeSet {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ViewTransitionTypeSet> for Any {
    fn from(s: ViewTransitionTypeSet) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ViewTransitionTypeSet> for Any {
    fn from(s: &ViewTransitionTypeSet) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ViewTransitionTypeSet);
