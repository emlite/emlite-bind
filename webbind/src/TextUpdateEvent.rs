use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for TextUpdateEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for TextUpdateEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
impl From<&TextUpdateEvent> for emlite::Val {
    fn from(s: &TextUpdateEvent) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(TextUpdateEvent);

impl TextUpdateEvent {
    pub fn new0(type_: DOMString) -> TextUpdateEvent {
        Self {
            inner: emlite::Val::global("TextUpdateEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: DOMString, options: Any) -> TextUpdateEvent {
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
    pub fn text(&self) -> DOMString {
        self.inner.get("text").as_::<DOMString>()
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
