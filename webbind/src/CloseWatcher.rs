use super::*;

/// The CloseWatcher class.
/// [`CloseWatcher`](https://developer.mozilla.org/en-US/docs/Web/API/CloseWatcher)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CloseWatcher {
    inner: EventTarget,
}
impl FromVal for CloseWatcher {
    fn from_val(v: &Any) -> Self {
        CloseWatcher {
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
impl core::ops::Deref for CloseWatcher {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CloseWatcher {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CloseWatcher {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CloseWatcher {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CloseWatcher> for Any {
    fn from(s: CloseWatcher) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CloseWatcher> for Any {
    fn from(s: &CloseWatcher) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CloseWatcher);

impl CloseWatcher {
    /// The `new CloseWatcher(..)` constructor, creating a new CloseWatcher instance
    pub fn new0() -> CloseWatcher {
        Self {
            inner: Any::global("CloseWatcher").new(&[]).as_::<EventTarget>(),
        }
    }

    /// The `new CloseWatcher(..)` constructor, creating a new CloseWatcher instance
    pub fn new1(options: &Any) -> CloseWatcher {
        Self {
            inner: Any::global("CloseWatcher")
                .new(&[options.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl CloseWatcher {
    /// The requestClose method.
    /// [`CloseWatcher.requestClose`](https://developer.mozilla.org/en-US/docs/Web/API/CloseWatcher/requestClose)
    pub fn request_close(&self) -> Undefined {
        self.inner.call("requestClose", &[]).as_::<Undefined>()
    }
}
impl CloseWatcher {
    /// The close method.
    /// [`CloseWatcher.close`](https://developer.mozilla.org/en-US/docs/Web/API/CloseWatcher/close)
    pub fn close(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
impl CloseWatcher {
    /// The destroy method.
    /// [`CloseWatcher.destroy`](https://developer.mozilla.org/en-US/docs/Web/API/CloseWatcher/destroy)
    pub fn destroy(&self) -> Undefined {
        self.inner.call("destroy", &[]).as_::<Undefined>()
    }
}
impl CloseWatcher {
    /// Getter of the `oncancel` attribute.
    /// [`CloseWatcher.oncancel`](https://developer.mozilla.org/en-US/docs/Web/API/CloseWatcher/oncancel)
    pub fn oncancel(&self) -> Any {
        self.inner.get("oncancel").as_::<Any>()
    }

    /// Setter of the `oncancel` attribute.
    /// [`CloseWatcher.oncancel`](https://developer.mozilla.org/en-US/docs/Web/API/CloseWatcher/oncancel)
    pub fn set_oncancel(&mut self, value: &Any) {
        self.inner.set("oncancel", value);
    }
}
impl CloseWatcher {
    /// Getter of the `onclose` attribute.
    /// [`CloseWatcher.onclose`](https://developer.mozilla.org/en-US/docs/Web/API/CloseWatcher/onclose)
    pub fn onclose(&self) -> Any {
        self.inner.get("onclose").as_::<Any>()
    }

    /// Setter of the `onclose` attribute.
    /// [`CloseWatcher.onclose`](https://developer.mozilla.org/en-US/docs/Web/API/CloseWatcher/onclose)
    pub fn set_onclose(&mut self, value: &Any) {
        self.inner.set("onclose", value);
    }
}
