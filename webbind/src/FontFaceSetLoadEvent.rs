use super::*;

/// The FontFaceSetLoadEvent class.
/// [`FontFaceSetLoadEvent`](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSetLoadEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FontFaceSetLoadEvent {
    inner: Event,
}
impl FromVal for FontFaceSetLoadEvent {
    fn from_val(v: &Any) -> Self {
        FontFaceSetLoadEvent {
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
impl core::ops::Deref for FontFaceSetLoadEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FontFaceSetLoadEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for FontFaceSetLoadEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for FontFaceSetLoadEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<FontFaceSetLoadEvent> for Any {
    fn from(s: FontFaceSetLoadEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&FontFaceSetLoadEvent> for Any {
    fn from(s: &FontFaceSetLoadEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(FontFaceSetLoadEvent);

impl FontFaceSetLoadEvent {
    /// The `new FontFaceSetLoadEvent(..)` constructor, creating a new FontFaceSetLoadEvent instance
    pub fn new0(type_: &CSSOMString) -> FontFaceSetLoadEvent {
        Self {
            inner: Any::global("FontFaceSetLoadEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new FontFaceSetLoadEvent(..)` constructor, creating a new FontFaceSetLoadEvent instance
    pub fn new1(type_: &CSSOMString, event_init_dict: &Any) -> FontFaceSetLoadEvent {
        Self {
            inner: Any::global("FontFaceSetLoadEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl FontFaceSetLoadEvent {
    /// Getter of the `fontfaces` attribute.
    /// [`FontFaceSetLoadEvent.fontfaces`](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSetLoadEvent/fontfaces)
    pub fn fontfaces(&self) -> FrozenArray<FontFace> {
        self.inner.get("fontfaces").as_::<FrozenArray<FontFace>>()
    }
}
