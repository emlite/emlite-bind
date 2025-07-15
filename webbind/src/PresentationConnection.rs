use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PresentationConnection {
    inner: EventTarget,
}
impl FromVal for PresentationConnection {
    fn from_val(v: &emlite::Val) -> Self {
        PresentationConnection { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PresentationConnection {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PresentationConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PresentationConnection {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PresentationConnection {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<PresentationConnection> for emlite::Val {
    fn from(s: PresentationConnection) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(PresentationConnection);


impl PresentationConnection {
    pub fn id(&self) -> USVString {
        self.inner.get("id").as_::<USVString>()
    }

}
impl PresentationConnection {
    pub fn url(&self) -> USVString {
        self.inner.get("url").as_::<USVString>()
    }

}
impl PresentationConnection {
    pub fn state(&self) -> PresentationConnectionState {
        self.inner.get("state").as_::<PresentationConnectionState>()
    }

}
impl PresentationConnection {
    pub fn close(&self, ) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }

}
impl PresentationConnection {
    pub fn terminate(&self, ) -> Undefined {
        self.inner.call("terminate", &[]).as_::<Undefined>()
    }

}
impl PresentationConnection {
    pub fn onconnect(&self) -> Any {
        self.inner.get("onconnect").as_::<Any>()
    }

    pub fn set_onconnect(&mut self, value: Any) {
        self.inner.set("onconnect", value);
    }

}
impl PresentationConnection {
    pub fn onclose(&self) -> Any {
        self.inner.get("onclose").as_::<Any>()
    }

    pub fn set_onclose(&mut self, value: Any) {
        self.inner.set("onclose", value);
    }

}
impl PresentationConnection {
    pub fn onterminate(&self) -> Any {
        self.inner.get("onterminate").as_::<Any>()
    }

    pub fn set_onterminate(&mut self, value: Any) {
        self.inner.set("onterminate", value);
    }

}
impl PresentationConnection {
    pub fn binary_type(&self) -> BinaryType {
        self.inner.get("binaryType").as_::<BinaryType>()
    }

    pub fn set_binary_type(&mut self, value: BinaryType) {
        self.inner.set("binaryType", value);
    }

}
impl PresentationConnection {
    pub fn onmessage(&self) -> Any {
        self.inner.get("onmessage").as_::<Any>()
    }

    pub fn set_onmessage(&mut self, value: Any) {
        self.inner.set("onmessage", value);
    }

}
impl PresentationConnection {
    pub fn send(&self, data: Any) -> Undefined {
        self.inner.call("send", &[data.into(), ]).as_::<Undefined>()
    }

}
