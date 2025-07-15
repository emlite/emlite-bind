use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for ClipboardEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ClipboardEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
impl From<&ClipboardEvent> for emlite::Val {
    fn from(s: &ClipboardEvent) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ClipboardEvent);

impl ClipboardEvent {
    pub fn new0(type_: &str) -> ClipboardEvent {
        Self {
            inner: emlite::Val::global("ClipboardEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: &str, event_init_dict: &Any) -> ClipboardEvent {
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
