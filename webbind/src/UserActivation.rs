use super::*;

/// The UserActivation class.
/// [`UserActivation`](https://developer.mozilla.org/en-US/docs/Web/API/UserActivation)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct UserActivation {
    inner: Any,
}
impl FromVal for UserActivation {
    fn from_val(v: &Any) -> Self {
        UserActivation {
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
impl core::ops::Deref for UserActivation {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for UserActivation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for UserActivation {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for UserActivation {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<UserActivation> for Any {
    fn from(s: UserActivation) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&UserActivation> for Any {
    fn from(s: &UserActivation) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(UserActivation);

impl UserActivation {
    /// Getter of the `hasBeenActive` attribute.
    /// [`UserActivation.hasBeenActive`](https://developer.mozilla.org/en-US/docs/Web/API/UserActivation/hasBeenActive)
    pub fn has_been_active(&self) -> bool {
        self.inner.get("hasBeenActive").as_::<bool>()
    }
}
impl UserActivation {
    /// Getter of the `isActive` attribute.
    /// [`UserActivation.isActive`](https://developer.mozilla.org/en-US/docs/Web/API/UserActivation/isActive)
    pub fn is_active(&self) -> bool {
        self.inner.get("isActive").as_::<bool>()
    }
}
