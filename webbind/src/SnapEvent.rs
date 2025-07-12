use super::*;

#[derive(Clone, Debug)]
pub struct SnapEvent {
    inner: Event,
}
impl FromVal for SnapEvent {
    fn from_val(v: &emlite::Val) -> Self {
        SnapEvent {
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
impl std::ops::Deref for SnapEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SnapEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SnapEvent> for emlite::Val {
    fn from(s: SnapEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SnapEvent {
    pub fn new0(type_: jsbind::DOMString) -> SnapEvent {
        Self {
            inner: emlite::Val::global("SnapEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> SnapEvent {
        Self {
            inner: emlite::Val::global("SnapEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl SnapEvent {
    pub fn snap_target_block(&self) -> Node {
        self.inner.get("snapTargetBlock").as_::<Node>()
    }
}
impl SnapEvent {
    pub fn snap_target_inline(&self) -> Node {
        self.inner.get("snapTargetInline").as_::<Node>()
    }
}
