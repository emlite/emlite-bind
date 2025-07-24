use super::*;

/// The CharacterBoundsUpdateEvent class.
/// [`CharacterBoundsUpdateEvent`](https://developer.mozilla.org/en-US/docs/Web/API/CharacterBoundsUpdateEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CharacterBoundsUpdateEvent {
    inner: Event,
}
impl FromVal for CharacterBoundsUpdateEvent {
    fn from_val(v: &Any) -> Self {
        CharacterBoundsUpdateEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for CharacterBoundsUpdateEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CharacterBoundsUpdateEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CharacterBoundsUpdateEvent> for Any {
    fn from(s: CharacterBoundsUpdateEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CharacterBoundsUpdateEvent> for Any {
    fn from(s: &CharacterBoundsUpdateEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CharacterBoundsUpdateEvent);

impl CharacterBoundsUpdateEvent {
    /// The `new CharacterBoundsUpdateEvent(..)` constructor, creating a new CharacterBoundsUpdateEvent instance
    pub fn new0(type_: &DOMString) -> CharacterBoundsUpdateEvent {
        Self {
            inner: Any::global("CharacterBoundsUpdateEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new CharacterBoundsUpdateEvent(..)` constructor, creating a new CharacterBoundsUpdateEvent instance
    pub fn new1(type_: &DOMString, options: &Any) -> CharacterBoundsUpdateEvent {
        Self {
            inner: Any::global("CharacterBoundsUpdateEvent")
                .new(&[type_.into(), options.into()])
                .as_::<Event>(),
        }
    }
}
impl CharacterBoundsUpdateEvent {
    /// Getter of the `rangeStart` attribute.
    /// [`CharacterBoundsUpdateEvent.rangeStart`](https://developer.mozilla.org/en-US/docs/Web/API/CharacterBoundsUpdateEvent/rangeStart)
    pub fn range_start(&self) -> u32 {
        self.inner.get("rangeStart").as_::<u32>()
    }
}
impl CharacterBoundsUpdateEvent {
    /// Getter of the `rangeEnd` attribute.
    /// [`CharacterBoundsUpdateEvent.rangeEnd`](https://developer.mozilla.org/en-US/docs/Web/API/CharacterBoundsUpdateEvent/rangeEnd)
    pub fn range_end(&self) -> u32 {
        self.inner.get("rangeEnd").as_::<u32>()
    }
}
