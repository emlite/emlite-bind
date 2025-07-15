use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ExtendableMessageEvent {
    inner: ExtendableEvent,
}
impl FromVal for ExtendableMessageEvent {
    fn from_val(v: &emlite::Val) -> Self {
        ExtendableMessageEvent { inner: ExtendableEvent::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ExtendableMessageEvent {
    type Target = ExtendableEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ExtendableMessageEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ExtendableMessageEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ExtendableMessageEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<ExtendableMessageEvent> for emlite::Val {
    fn from(s: ExtendableMessageEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(ExtendableMessageEvent);



impl ExtendableMessageEvent {
    pub fn new0(type_: DOMString) -> ExtendableMessageEvent {
        Self {
            inner: emlite::Val::global("ExtendableMessageEvent").new(&[type_.into()]).as_::<ExtendableEvent>(),
        }
    }

    pub fn new1(type_: DOMString, event_init_dict: Any) -> ExtendableMessageEvent {
        Self {
            inner: emlite::Val::global("ExtendableMessageEvent").new(&[type_.into(), event_init_dict.into()]).as_::<ExtendableEvent>(),
        }
    }

}
impl ExtendableMessageEvent {
    pub fn data(&self) -> Any {
        self.inner.get("data").as_::<Any>()
    }

}
impl ExtendableMessageEvent {
    pub fn origin(&self) -> USVString {
        self.inner.get("origin").as_::<USVString>()
    }

}
impl ExtendableMessageEvent {
    pub fn last_event_id(&self) -> DOMString {
        self.inner.get("lastEventId").as_::<DOMString>()
    }

}
impl ExtendableMessageEvent {
    pub fn source(&self) -> Any {
        self.inner.get("source").as_::<Any>()
    }

}
impl ExtendableMessageEvent {
    pub fn ports(&self) -> FrozenArray<Any> {
        self.inner.get("ports").as_::<FrozenArray<Any>>()
    }

}
