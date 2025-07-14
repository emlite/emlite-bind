use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebGLContextEvent {
    inner: Event,
}
impl FromVal for WebGLContextEvent {
    fn from_val(v: &emlite::Val) -> Self {
        WebGLContextEvent {
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
impl core::ops::Deref for WebGLContextEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebGLContextEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WebGLContextEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WebGLContextEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<WebGLContextEvent> for emlite::Val {
    fn from(s: WebGLContextEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(WebGLContextEvent);

impl WebGLContextEvent {
    pub fn new0(type_: jsbind::DOMString) -> WebGLContextEvent {
        Self {
            inner: emlite::Val::global("WebGLContextEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, event_init: jsbind::Any) -> WebGLContextEvent {
        Self {
            inner: emlite::Val::global("WebGLContextEvent")
                .new(&[type_.into(), event_init.into()])
                .as_::<Event>(),
        }
    }
}
impl WebGLContextEvent {
    pub fn status_message(&self) -> jsbind::DOMString {
        self.inner.get("statusMessage").as_::<jsbind::DOMString>()
    }
}
