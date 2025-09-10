use super::*;

/// The WEBGL_lose_context class.
/// [`WEBGL_lose_context`](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_lose_context)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WEBGL_lose_context {
    inner: Any,
}

impl FromVal for WEBGL_lose_context {
    fn from_val(v: &Any) -> Self {
        WEBGL_lose_context {
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

impl core::ops::Deref for WEBGL_lose_context {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WEBGL_lose_context {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WEBGL_lose_context {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WEBGL_lose_context {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<WEBGL_lose_context> for Any {
    fn from(s: WEBGL_lose_context) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WEBGL_lose_context> for Any {
    fn from(s: &WEBGL_lose_context) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WEBGL_lose_context);

impl WEBGL_lose_context {
    /// The loseContext method.
    /// [`WEBGL_lose_context.loseContext`](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_lose_context/loseContext)
    pub fn lose_context(&self) -> Undefined {
        self.inner.call("loseContext", &[]).as_::<Undefined>()
    }
}
impl WEBGL_lose_context {
    /// The restoreContext method.
    /// [`WEBGL_lose_context.restoreContext`](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_lose_context/restoreContext)
    pub fn restore_context(&self) -> Undefined {
        self.inner.call("restoreContext", &[]).as_::<Undefined>()
    }
}
