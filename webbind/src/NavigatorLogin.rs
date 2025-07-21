use super::*;

/// The NavigatorLogin class.
/// [`NavigatorLogin`](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorLogin)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigatorLogin {
    inner: Any,
}
impl FromVal for NavigatorLogin {
    fn from_val(v: &Any) -> Self {
        NavigatorLogin {
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
impl core::ops::Deref for NavigatorLogin {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NavigatorLogin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for NavigatorLogin {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for NavigatorLogin {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<NavigatorLogin> for Any {
    fn from(s: NavigatorLogin) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&NavigatorLogin> for Any {
    fn from(s: &NavigatorLogin) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(NavigatorLogin);

impl NavigatorLogin {
    /// The setStatus method.
    /// [`NavigatorLogin.setStatus`](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorLogin/setStatus)
    pub fn set_status(&self, status: &LoginStatus) -> Promise<Undefined> {
        self.inner
            .call("setStatus", &[status.into()])
            .as_::<Promise<Undefined>>()
    }
}
