use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ClipboardEvent {
    inner: Event,
}
impl FromVal for ClipboardEvent {
    fn from_val(v: &emlite::Val) -> Self {
        ClipboardEvent {
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
impl core::ops::Deref for ClipboardEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ClipboardEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ClipboardEvent> for emlite::Val {
    fn from(s: ClipboardEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ClipboardEvent {
    pub fn new0(type_: jsbind::DOMString) -> ClipboardEvent {
        Self {
            inner: emlite::Val::global("ClipboardEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> ClipboardEvent {
        Self {
            inner: emlite::Val::global("ClipboardEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl ClipboardEvent {
    pub fn clipboard_data(&self) -> DataTransfer {
        self.inner.get("clipboardData").as_::<DataTransfer>()
    }
}
