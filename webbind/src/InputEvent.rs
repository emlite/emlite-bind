use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct InputEvent {
    inner: UIEvent,
}
impl FromVal for InputEvent {
    fn from_val(v: &emlite::Val) -> Self {
        InputEvent {
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
impl core::ops::Deref for InputEvent {
    type Target = UIEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for InputEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for InputEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for InputEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<InputEvent> for emlite::Val {
    fn from(s: InputEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(InputEvent);

impl InputEvent {
    pub fn new0(type_: jsbind::DOMString) -> InputEvent {
        Self {
            inner: emlite::Val::global("InputEvent")
                .new(&[type_.into()])
                .as_::<UIEvent>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> InputEvent {
        Self {
            inner: emlite::Val::global("InputEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<UIEvent>(),
        }
    }
}
impl InputEvent {
    pub fn data(&self) -> jsbind::USVString {
        self.inner.get("data").as_::<jsbind::USVString>()
    }
}
impl InputEvent {
    pub fn is_composing(&self) -> bool {
        self.inner.get("isComposing").as_::<bool>()
    }
}
impl InputEvent {
    pub fn input_type(&self) -> jsbind::DOMString {
        self.inner.get("inputType").as_::<jsbind::DOMString>()
    }
}
impl InputEvent {
    pub fn data_transfer(&self) -> DataTransfer {
        self.inner.get("dataTransfer").as_::<DataTransfer>()
    }
}
impl InputEvent {
    pub fn get_target_ranges(&self) -> jsbind::Sequence<StaticRange> {
        self.inner
            .call("getTargetRanges", &[])
            .as_::<jsbind::Sequence<StaticRange>>()
    }
}
