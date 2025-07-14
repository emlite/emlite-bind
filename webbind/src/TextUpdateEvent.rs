use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TextUpdateEvent {
    inner: Event,
}
impl FromVal for TextUpdateEvent {
    fn from_val(v: &emlite::Val) -> Self {
        TextUpdateEvent {
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
impl core::ops::Deref for TextUpdateEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TextUpdateEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<TextUpdateEvent> for emlite::Val {
    fn from(s: TextUpdateEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl TextUpdateEvent {
    pub fn new0(type_: jsbind::DOMString) -> TextUpdateEvent {
        Self {
            inner: emlite::Val::global("TextUpdateEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, options: jsbind::Any) -> TextUpdateEvent {
        Self {
            inner: emlite::Val::global("TextUpdateEvent")
                .new(&[type_.into(), options.into()])
                .as_::<Event>(),
        }
    }
}
impl TextUpdateEvent {
    pub fn update_range_start(&self) -> u32 {
        self.inner.get("updateRangeStart").as_::<u32>()
    }
}
impl TextUpdateEvent {
    pub fn update_range_end(&self) -> u32 {
        self.inner.get("updateRangeEnd").as_::<u32>()
    }
}
impl TextUpdateEvent {
    pub fn text(&self) -> jsbind::DOMString {
        self.inner.get("text").as_::<jsbind::DOMString>()
    }
}
impl TextUpdateEvent {
    pub fn selection_start(&self) -> u32 {
        self.inner.get("selectionStart").as_::<u32>()
    }
}
impl TextUpdateEvent {
    pub fn selection_end(&self) -> u32 {
        self.inner.get("selectionEnd").as_::<u32>()
    }
}
