use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BroadcastChannel {
    inner: EventTarget,
}
impl FromVal for BroadcastChannel {
    fn from_val(v: &emlite::Val) -> Self {
        BroadcastChannel {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for BroadcastChannel {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for BroadcastChannel {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<BroadcastChannel> for emlite::Val {
    fn from(s: BroadcastChannel) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(BroadcastChannel);

impl BroadcastChannel {
    pub fn new(name: jsbind::DOMString) -> BroadcastChannel {
        Self {
            inner: emlite::Val::global("BroadcastChannel")
                .new(&[name.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl BroadcastChannel {
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
    }
}
impl BroadcastChannel {
    pub fn post_message(&self, message: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("postMessage", &[message.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl BroadcastChannel {
    pub fn close(&self) -> jsbind::Undefined {
        self.inner.call("close", &[]).as_::<jsbind::Undefined>()
    }
}
impl BroadcastChannel {
    pub fn onmessage(&self) -> jsbind::Any {
        self.inner.get("onmessage").as_::<jsbind::Any>()
    }

    pub fn set_onmessage(&mut self, value: jsbind::Any) {
        self.inner.set("onmessage", value);
    }
}
impl BroadcastChannel {
    pub fn onmessageerror(&self) -> jsbind::Any {
        self.inner.get("onmessageerror").as_::<jsbind::Any>()
    }

    pub fn set_onmessageerror(&mut self, value: jsbind::Any) {
        self.inner.set("onmessageerror", value);
    }
}
