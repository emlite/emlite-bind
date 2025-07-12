use super::*;

#[derive(Clone, Debug)]
pub struct FormDataEvent {
    inner: Event,
}
impl FromVal for FormDataEvent {
    fn from_val(v: &emlite::Val) -> Self {
        FormDataEvent {
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
impl std::ops::Deref for FormDataEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for FormDataEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<FormDataEvent> for emlite::Val {
    fn from(s: FormDataEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FormDataEvent {
    pub fn new(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> FormDataEvent {
        Self {
            inner: emlite::Val::global("FormDataEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl FormDataEvent {
    pub fn form_data(&self) -> FormData {
        self.inner.get("formData").as_::<FormData>()
    }
}
