use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ErrorEvent {
    inner: Event,
}
impl FromVal for ErrorEvent {
    fn from_val(v: &emlite::Val) -> Self {
        ErrorEvent {
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
impl core::ops::Deref for ErrorEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ErrorEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ErrorEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ErrorEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ErrorEvent> for emlite::Val {
    fn from(s: ErrorEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(ErrorEvent);

impl ErrorEvent {
    pub fn new0(type_: DOMString) -> ErrorEvent {
        Self {
            inner: emlite::Val::global("ErrorEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: DOMString, event_init_dict: Any) -> ErrorEvent {
        Self {
            inner: emlite::Val::global("ErrorEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl ErrorEvent {
    pub fn message(&self) -> DOMString {
        self.inner.get("message").as_::<DOMString>()
    }
}
impl ErrorEvent {
    pub fn filename(&self) -> USVString {
        self.inner.get("filename").as_::<USVString>()
    }
}
impl ErrorEvent {
    pub fn lineno(&self) -> u32 {
        self.inner.get("lineno").as_::<u32>()
    }
}
impl ErrorEvent {
    pub fn colno(&self) -> u32 {
        self.inner.get("colno").as_::<u32>()
    }
}
impl ErrorEvent {
    pub fn error(&self) -> Any {
        self.inner.get("error").as_::<Any>()
    }
}
