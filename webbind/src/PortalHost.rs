use super::*;

/// The PortalHost class.
/// [`PortalHost`](https://developer.mozilla.org/en-US/docs/Web/API/PortalHost)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PortalHost {
    inner: EventTarget,
}

impl FromVal for PortalHost {
    fn from_val(v: &Any) -> Self {
        PortalHost {
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

impl core::ops::Deref for PortalHost {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PortalHost {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PortalHost {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PortalHost {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PortalHost> for Any {
    fn from(s: PortalHost) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PortalHost> for Any {
    fn from(s: &PortalHost) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(PortalHost);

impl PortalHost {
    /// Getter of the `onmessage` attribute.
    /// [`PortalHost.onmessage`](https://developer.mozilla.org/en-US/docs/Web/API/PortalHost/onmessage)
    pub fn onmessage(&self) -> Any {
        self.inner.get("onmessage").as_::<Any>()
    }

    /// Setter of the `onmessage` attribute.
    /// [`PortalHost.onmessage`](https://developer.mozilla.org/en-US/docs/Web/API/PortalHost/onmessage)
    pub fn set_onmessage(&mut self, value: &Any) {
        self.inner.set("onmessage", value);
    }
}
impl PortalHost {
    /// Getter of the `onmessageerror` attribute.
    /// [`PortalHost.onmessageerror`](https://developer.mozilla.org/en-US/docs/Web/API/PortalHost/onmessageerror)
    pub fn onmessageerror(&self) -> Any {
        self.inner.get("onmessageerror").as_::<Any>()
    }

    /// Setter of the `onmessageerror` attribute.
    /// [`PortalHost.onmessageerror`](https://developer.mozilla.org/en-US/docs/Web/API/PortalHost/onmessageerror)
    pub fn set_onmessageerror(&mut self, value: &Any) {
        self.inner.set("onmessageerror", value);
    }
}
impl PortalHost {
    /// The postMessage method.
    /// [`PortalHost.postMessage`](https://developer.mozilla.org/en-US/docs/Web/API/PortalHost/postMessage)
    pub fn post_message0(&self, message: &Any) -> Undefined {
        self.inner
            .call("postMessage", &[message.into()])
            .as_::<Undefined>()
    }
    /// The postMessage method.
    /// [`PortalHost.postMessage`](https://developer.mozilla.org/en-US/docs/Web/API/PortalHost/postMessage)
    pub fn post_message1(&self, message: &Any, options: &StructuredSerializeOptions) -> Undefined {
        self.inner
            .call("postMessage", &[message.into(), options.into()])
            .as_::<Undefined>()
    }
}
