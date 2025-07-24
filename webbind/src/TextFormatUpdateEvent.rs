use super::*;

/// The TextFormatUpdateEvent class.
/// [`TextFormatUpdateEvent`](https://developer.mozilla.org/en-US/docs/Web/API/TextFormatUpdateEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TextFormatUpdateEvent {
    inner: Event,
}
impl FromVal for TextFormatUpdateEvent {
    fn from_val(v: &Any) -> Self {
        TextFormatUpdateEvent {
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
impl core::ops::Deref for TextFormatUpdateEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TextFormatUpdateEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for TextFormatUpdateEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for TextFormatUpdateEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<TextFormatUpdateEvent> for Any {
    fn from(s: TextFormatUpdateEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&TextFormatUpdateEvent> for Any {
    fn from(s: &TextFormatUpdateEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(TextFormatUpdateEvent);

impl TextFormatUpdateEvent {
    /// The `new TextFormatUpdateEvent(..)` constructor, creating a new TextFormatUpdateEvent instance
    pub fn new0(type_: &DOMString) -> TextFormatUpdateEvent {
        Self {
            inner: Any::global("TextFormatUpdateEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new TextFormatUpdateEvent(..)` constructor, creating a new TextFormatUpdateEvent instance
    pub fn new1(type_: &DOMString, options: &Any) -> TextFormatUpdateEvent {
        Self {
            inner: Any::global("TextFormatUpdateEvent")
                .new(&[type_.into(), options.into()])
                .as_::<Event>(),
        }
    }
}
impl TextFormatUpdateEvent {
    /// The getTextFormats method.
    /// [`TextFormatUpdateEvent.getTextFormats`](https://developer.mozilla.org/en-US/docs/Web/API/TextFormatUpdateEvent/getTextFormats)
    pub fn get_text_formats(&self) -> Sequence<TextFormat> {
        self.inner
            .call("getTextFormats", &[])
            .as_::<Sequence<TextFormat>>()
    }
}
