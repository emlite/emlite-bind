use super::*;

/// The WakeLockSentinel class.
/// [`WakeLockSentinel`](https://developer.mozilla.org/en-US/docs/Web/API/WakeLockSentinel)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WakeLockSentinel {
    inner: EventTarget,
}
impl FromVal for WakeLockSentinel {
    fn from_val(v: &Any) -> Self {
        WakeLockSentinel {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WakeLockSentinel {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WakeLockSentinel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for WakeLockSentinel {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for WakeLockSentinel {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<WakeLockSentinel> for Any {
    fn from(s: WakeLockSentinel) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&WakeLockSentinel> for Any {
    fn from(s: &WakeLockSentinel) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(WakeLockSentinel);

impl WakeLockSentinel {
    /// Getter of the `released` attribute.
    /// [`WakeLockSentinel.released`](https://developer.mozilla.org/en-US/docs/Web/API/WakeLockSentinel/released)
    pub fn released(&self) -> bool {
        self.inner.get("released").as_::<bool>()
    }
}
impl WakeLockSentinel {
    /// Getter of the `type` attribute.
    /// [`WakeLockSentinel.type`](https://developer.mozilla.org/en-US/docs/Web/API/WakeLockSentinel/type)
    pub fn type_(&self) -> WakeLockType {
        self.inner.get("type").as_::<WakeLockType>()
    }
}
impl WakeLockSentinel {
    /// The release method.
    /// [`WakeLockSentinel.release`](https://developer.mozilla.org/en-US/docs/Web/API/WakeLockSentinel/release)
    pub fn release(&self) -> Promise {
        self.inner.call("release", &[]).as_::<Promise>()
    }
}
impl WakeLockSentinel {
    /// Getter of the `onrelease` attribute.
    /// [`WakeLockSentinel.onrelease`](https://developer.mozilla.org/en-US/docs/Web/API/WakeLockSentinel/onrelease)
    pub fn onrelease(&self) -> Any {
        self.inner.get("onrelease").as_::<Any>()
    }

    /// Setter of the `onrelease` attribute.
    /// [`WakeLockSentinel.onrelease`](https://developer.mozilla.org/en-US/docs/Web/API/WakeLockSentinel/onrelease)
    pub fn set_onrelease(&mut self, value: &Any) {
        self.inner.set("onrelease", value);
    }
}
