use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TouchEvent {
    inner: UIEvent,
}
impl FromVal for TouchEvent {
    fn from_val(v: &emlite::Val) -> Self {
        TouchEvent { inner: UIEvent::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for TouchEvent {
    type Target = UIEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TouchEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for TouchEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for TouchEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<TouchEvent> for emlite::Val {
    fn from(s: TouchEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(TouchEvent);



impl TouchEvent {
    pub fn new0(type_: DOMString) -> TouchEvent {
        Self {
            inner: emlite::Val::global("TouchEvent").new(&[type_.into()]).as_::<UIEvent>(),
        }
    }

    pub fn new1(type_: DOMString, event_init_dict: Any) -> TouchEvent {
        Self {
            inner: emlite::Val::global("TouchEvent").new(&[type_.into(), event_init_dict.into()]).as_::<UIEvent>(),
        }
    }

}
impl TouchEvent {
    pub fn touches(&self) -> TouchList {
        self.inner.get("touches").as_::<TouchList>()
    }

}
impl TouchEvent {
    pub fn target_touches(&self) -> TouchList {
        self.inner.get("targetTouches").as_::<TouchList>()
    }

}
impl TouchEvent {
    pub fn changed_touches(&self) -> TouchList {
        self.inner.get("changedTouches").as_::<TouchList>()
    }

}
impl TouchEvent {
    pub fn alt_key(&self) -> bool {
        self.inner.get("altKey").as_::<bool>()
    }

}
impl TouchEvent {
    pub fn meta_key(&self) -> bool {
        self.inner.get("metaKey").as_::<bool>()
    }

}
impl TouchEvent {
    pub fn ctrl_key(&self) -> bool {
        self.inner.get("ctrlKey").as_::<bool>()
    }

}
impl TouchEvent {
    pub fn shift_key(&self) -> bool {
        self.inner.get("shiftKey").as_::<bool>()
    }

}
impl TouchEvent {
    pub fn get_modifier_state(&self, key_arg: DOMString) -> bool {
        self.inner.call("getModifierState", &[key_arg.into(), ]).as_::<bool>()
    }

}
