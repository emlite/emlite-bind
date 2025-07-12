use super::*;

#[derive(Clone, Debug)]
pub struct TouchEvent {
    inner: UIEvent,
}
impl FromVal for TouchEvent {
    fn from_val(v: &emlite::Val) -> Self {
        TouchEvent {
            inner: UIEvent::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for TouchEvent {
    type Target = UIEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for TouchEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<TouchEvent> for emlite::Val {
    fn from(s: TouchEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl TouchEvent {
    pub fn new0(type_: jsbind::DOMString) -> TouchEvent {
        Self {
            inner: emlite::Val::global("TouchEvent")
                .new(&[type_.into()])
                .as_::<UIEvent>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> TouchEvent {
        Self {
            inner: emlite::Val::global("TouchEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<UIEvent>(),
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
    pub fn get_modifier_state(&self, key_arg: jsbind::DOMString) -> bool {
        self.inner
            .call("getModifierState", &[key_arg.into()])
            .as_::<bool>()
    }
}
