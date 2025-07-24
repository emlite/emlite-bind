use super::*;

/// The OverconstrainedError class.
/// [`OverconstrainedError`](https://developer.mozilla.org/en-US/docs/Web/API/OverconstrainedError)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OverconstrainedError {
    inner: DOMException,
}
impl FromVal for OverconstrainedError {
    fn from_val(v: &Any) -> Self {
        OverconstrainedError {
            inner: DOMException::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for OverconstrainedError {
    type Target = DOMException;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for OverconstrainedError {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for OverconstrainedError {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for OverconstrainedError {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<OverconstrainedError> for Any {
    fn from(s: OverconstrainedError) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&OverconstrainedError> for Any {
    fn from(s: &OverconstrainedError) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(OverconstrainedError);

impl OverconstrainedError {
    /// The `new OverconstrainedError(..)` constructor, creating a new OverconstrainedError instance
    pub fn new0(constraint: &DOMString) -> OverconstrainedError {
        Self {
            inner: Any::global("OverconstrainedError")
                .new(&[constraint.into()])
                .as_::<DOMException>(),
        }
    }

    /// The `new OverconstrainedError(..)` constructor, creating a new OverconstrainedError instance
    pub fn new1(constraint: &DOMString, message: &DOMString) -> OverconstrainedError {
        Self {
            inner: Any::global("OverconstrainedError")
                .new(&[constraint.into(), message.into()])
                .as_::<DOMException>(),
        }
    }
}
impl OverconstrainedError {
    /// Getter of the `constraint` attribute.
    /// [`OverconstrainedError.constraint`](https://developer.mozilla.org/en-US/docs/Web/API/OverconstrainedError/constraint)
    pub fn constraint(&self) -> DOMString {
        self.inner.get("constraint").as_::<DOMString>()
    }
}
