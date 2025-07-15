use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StructuredSerializeOptions {
    inner: emlite::Val,
}
impl FromVal for StructuredSerializeOptions {
    fn from_val(v: &emlite::Val) -> Self {
        StructuredSerializeOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for StructuredSerializeOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for StructuredSerializeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for StructuredSerializeOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for StructuredSerializeOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<StructuredSerializeOptions> for emlite::Val {
    fn from(s: StructuredSerializeOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl StructuredSerializeOptions {
    pub fn transfer(&self) -> Sequence<Object> {
        self.inner.get("transfer").as_::<Sequence<Object>>()
    }

    pub fn set_transfer(&mut self, value: Sequence<Object>) {
        self.inner.set("transfer", value);
    }

}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MessagePort {
    inner: EventTarget,
}
impl FromVal for MessagePort {
    fn from_val(v: &emlite::Val) -> Self {
        MessagePort { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MessagePort {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MessagePort {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MessagePort {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MessagePort {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<MessagePort> for emlite::Val {
    fn from(s: MessagePort) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(MessagePort);


impl MessagePort {
    pub fn post_message0(&self, message: Any) -> Undefined {
        self.inner.call("postMessage", &[message.into(), ]).as_::<Undefined>()
    }

    pub fn post_message1(&self, message: Any, options: StructuredSerializeOptions) -> Undefined {
        self.inner.call("postMessage", &[message.into(), options.into(), ]).as_::<Undefined>()
    }

}
impl MessagePort {
    pub fn start(&self, ) -> Undefined {
        self.inner.call("start", &[]).as_::<Undefined>()
    }

}
impl MessagePort {
    pub fn close(&self, ) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }

}
impl MessagePort {
    pub fn onclose(&self) -> Any {
        self.inner.get("onclose").as_::<Any>()
    }

    pub fn set_onclose(&mut self, value: Any) {
        self.inner.set("onclose", value);
    }

}
impl MessagePort {
    pub fn onmessage(&self) -> Any {
        self.inner.get("onmessage").as_::<Any>()
    }

    pub fn set_onmessage(&mut self, value: Any) {
        self.inner.set("onmessage", value);
    }

}
impl MessagePort {
    pub fn onmessageerror(&self) -> Any {
        self.inner.get("onmessageerror").as_::<Any>()
    }

    pub fn set_onmessageerror(&mut self, value: Any) {
        self.inner.set("onmessageerror", value);
    }

}
