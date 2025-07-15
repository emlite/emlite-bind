use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CharacterBoundsUpdateEvent {
    inner: Event,
}
impl FromVal for CharacterBoundsUpdateEvent {
    fn from_val(v: &emlite::Val) -> Self {
        CharacterBoundsUpdateEvent {
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
impl core::ops::Deref for CharacterBoundsUpdateEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CharacterBoundsUpdateEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CharacterBoundsUpdateEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CharacterBoundsUpdateEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CharacterBoundsUpdateEvent> for emlite::Val {
    fn from(s: CharacterBoundsUpdateEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CharacterBoundsUpdateEvent> for emlite::Val {
    fn from(s: &CharacterBoundsUpdateEvent) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CharacterBoundsUpdateEvent);

impl CharacterBoundsUpdateEvent {
    pub fn new0(type_: DOMString) -> CharacterBoundsUpdateEvent {
        Self {
            inner: emlite::Val::global("CharacterBoundsUpdateEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: DOMString, options: Any) -> CharacterBoundsUpdateEvent {
        Self {
            inner: emlite::Val::global("CharacterBoundsUpdateEvent")
                .new(&[type_.into(), options.into()])
                .as_::<Event>(),
        }
    }
}
impl CharacterBoundsUpdateEvent {
    pub fn range_start(&self) -> u32 {
        self.inner.get("rangeStart").as_::<u32>()
    }
}
impl CharacterBoundsUpdateEvent {
    pub fn range_end(&self) -> u32 {
        self.inner.get("rangeEnd").as_::<u32>()
    }
}
