use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for DragEvent {
    type Target = MouseEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DragEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for DragEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DragEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<DragEvent> for emlite::Val {
    fn from(s: DragEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(DragEvent);

impl DragEvent {
    pub fn new0(type_: DOMString) -> DragEvent {
        Self {
            inner: emlite::Val::global("DragEvent")
                .new(&[type_.into()])
                .as_::<MouseEvent>(),
        }
    }

    pub fn new1(type_: DOMString, event_init_dict: Any) -> DragEvent {
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
