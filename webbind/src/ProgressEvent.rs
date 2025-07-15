use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ProgressEvent {
    inner: Event,
}
impl FromVal for ProgressEvent {
    fn from_val(v: &emlite::Val) -> Self {
        ProgressEvent {
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
impl core::ops::Deref for ProgressEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ProgressEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ProgressEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ProgressEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ProgressEvent> for emlite::Val {
    fn from(s: ProgressEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&ProgressEvent> for emlite::Val {
    fn from(s: &ProgressEvent) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ProgressEvent);

impl ProgressEvent {
    pub fn new0(type_: DOMString) -> ProgressEvent {
        Self {
            inner: emlite::Val::global("ProgressEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: DOMString, event_init_dict: Any) -> ProgressEvent {
        Self {
            inner: emlite::Val::global("ProgressEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl ProgressEvent {
    pub fn length_computable(&self) -> bool {
        self.inner.get("lengthComputable").as_::<bool>()
    }
}
impl ProgressEvent {
    pub fn loaded(&self) -> f64 {
        self.inner.get("loaded").as_::<f64>()
    }
}
impl ProgressEvent {
    pub fn total(&self) -> f64 {
        self.inner.get("total").as_::<f64>()
    }
}
