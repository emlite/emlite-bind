use super::*;

#[derive(Clone, Debug)]
pub struct DragEvent {
    inner: MouseEvent,
}
impl FromVal for DragEvent {
    fn from_val(v: &emlite::Val) -> Self {
        DragEvent {
            inner: MouseEvent::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for DragEvent {
    type Target = MouseEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for DragEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<DragEvent> for emlite::Val {
    fn from(s: DragEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl DragEvent {
    pub fn new0(type_: jsbind::DOMString) -> DragEvent {
        Self {
            inner: emlite::Val::global("DragEvent")
                .new(&[type_.into()])
                .as_::<MouseEvent>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> DragEvent {
        Self {
            inner: emlite::Val::global("DragEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<MouseEvent>(),
        }
    }
}
impl DragEvent {
    pub fn data_transfer(&self) -> DataTransfer {
        self.inner.get("dataTransfer").as_::<DataTransfer>()
    }
}
