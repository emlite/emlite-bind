use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BufferedChangeEvent {
    inner: Event,
}
impl FromVal for BufferedChangeEvent {
    fn from_val(v: &emlite::Val) -> Self {
        BufferedChangeEvent {
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
impl core::ops::Deref for BufferedChangeEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BufferedChangeEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for BufferedChangeEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for BufferedChangeEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<BufferedChangeEvent> for emlite::Val {
    fn from(s: BufferedChangeEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(BufferedChangeEvent);

impl BufferedChangeEvent {
    pub fn new0(type_: jsbind::DOMString) -> BufferedChangeEvent {
        Self {
            inner: emlite::Val::global("BufferedChangeEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> BufferedChangeEvent {
        Self {
            inner: emlite::Val::global("BufferedChangeEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl BufferedChangeEvent {
    pub fn added_ranges(&self) -> TimeRanges {
        self.inner.get("addedRanges").as_::<TimeRanges>()
    }
}
impl BufferedChangeEvent {
    pub fn removed_ranges(&self) -> TimeRanges {
        self.inner.get("removedRanges").as_::<TimeRanges>()
    }
}
