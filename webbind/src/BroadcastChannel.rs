use super::*;

/// The BroadcastChannel class.
/// [`BroadcastChannel`](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BroadcastChannel {
    inner: EventTarget,
}
impl FromVal for BroadcastChannel {
    fn from_val(v: &Any) -> Self {
        BroadcastChannel {
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
impl core::ops::Deref for BroadcastChannel {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BroadcastChannel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for BroadcastChannel {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for BroadcastChannel {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<BroadcastChannel> for Any {
    fn from(s: BroadcastChannel) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&BroadcastChannel> for Any {
    fn from(s: &BroadcastChannel) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(BroadcastChannel);

impl BroadcastChannel {
    /// The `new BroadcastChannel(..)` constructor, creating a new BroadcastChannel instance
    pub fn new(name: &str) -> BroadcastChannel {
        Self {
            inner: Any::global("BroadcastChannel")
                .new(&[name.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl BroadcastChannel {
    /// Getter of the `name` attribute.
    /// [`BroadcastChannel.name`](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/name)
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }
}
impl BroadcastChannel {
    /// The postMessage method.
    /// [`BroadcastChannel.postMessage`](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/postMessage)
    pub fn post_message(&self, message: &Any) -> Undefined {
        self.inner
            .call("postMessage", &[message.into()])
            .as_::<Undefined>()
    }
}
impl BroadcastChannel {
    /// The close method.
    /// [`BroadcastChannel.close`](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/close)
    pub fn close(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
impl BroadcastChannel {
    /// Getter of the `onmessage` attribute.
    /// [`BroadcastChannel.onmessage`](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/onmessage)
    pub fn onmessage(&self) -> Any {
        self.inner.get("onmessage").as_::<Any>()
    }

    /// Setter of the `onmessage` attribute.
    /// [`BroadcastChannel.onmessage`](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/onmessage)
    pub fn set_onmessage(&mut self, value: &Any) {
        self.inner.set("onmessage", value);
    }
}
impl BroadcastChannel {
    /// Getter of the `onmessageerror` attribute.
    /// [`BroadcastChannel.onmessageerror`](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/onmessageerror)
    pub fn onmessageerror(&self) -> Any {
        self.inner.get("onmessageerror").as_::<Any>()
    }

    /// Setter of the `onmessageerror` attribute.
    /// [`BroadcastChannel.onmessageerror`](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/onmessageerror)
    pub fn set_onmessageerror(&mut self, value: &Any) {
        self.inner.set("onmessageerror", value);
    }
}
