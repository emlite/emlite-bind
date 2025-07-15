use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ExtendableEvent {
    inner: Event,
}
impl FromVal for ExtendableEvent {
    fn from_val(v: &emlite::Val) -> Self {
        ExtendableEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ExtendableEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ExtendableEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ExtendableEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ExtendableEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ExtendableEvent> for emlite::Val {
    fn from(s: ExtendableEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(ExtendableEvent);

impl ExtendableEvent {
    pub fn new0(type_: DOMString) -> ExtendableEvent {
        Self {
            inner: emlite::Val::global("ExtendableEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: DOMString, event_init_dict: Any) -> ExtendableEvent {
        Self {
            inner: emlite::Val::global("ExtendableEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl ExtendableEvent {
    pub fn wait_until(&self, f: Promise) -> Undefined {
        self.inner.call("waitUntil", &[f.into()]).as_::<Undefined>()
    }
}
