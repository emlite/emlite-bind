use super::*;

/// The Lock class.
/// [`Lock`](https://developer.mozilla.org/en-US/docs/Web/API/Lock)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Lock {
    inner: Any,
}
impl FromVal for Lock {
    fn from_val(v: &Any) -> Self {
        Lock {
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
impl core::ops::Deref for Lock {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Lock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for Lock {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Lock {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Lock> for Any {
    fn from(s: Lock) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Lock> for Any {
    fn from(s: &Lock) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Lock);

impl Lock {
    /// Getter of the `name` attribute.
    /// [`Lock.name`](https://developer.mozilla.org/en-US/docs/Web/API/Lock/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }
}
impl Lock {
    /// Getter of the `mode` attribute.
    /// [`Lock.mode`](https://developer.mozilla.org/en-US/docs/Web/API/Lock/mode)
    pub fn mode(&self) -> LockMode {
        self.inner.get("mode").as_::<LockMode>()
    }
}
