use super::*;

/// The TextUpdateEvent class.
/// [`TextUpdateEvent`](https://developer.mozilla.org/en-US/docs/Web/API/TextUpdateEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TextUpdateEvent {
    inner: Event,
}

impl FromVal for TextUpdateEvent {
    fn from_val(v: &Any) -> Self {
        TextUpdateEvent {
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

impl AsRef<Any> for TextUpdateEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for TextUpdateEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<TextUpdateEvent> for Any {
    fn from(s: TextUpdateEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&TextUpdateEvent> for Any {
    fn from(s: &TextUpdateEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(TextUpdateEvent);

impl TextUpdateEvent {
    /// Getter of the `updateRangeStart` attribute.
    /// [`TextUpdateEvent.updateRangeStart`](https://developer.mozilla.org/en-US/docs/Web/API/TextUpdateEvent/updateRangeStart)
    pub fn update_range_start(&self) -> u32 {
        self.inner.get("updateRangeStart").as_::<u32>()
    }
}
impl TextUpdateEvent {
    /// Getter of the `updateRangeEnd` attribute.
    /// [`TextUpdateEvent.updateRangeEnd`](https://developer.mozilla.org/en-US/docs/Web/API/TextUpdateEvent/updateRangeEnd)
    pub fn update_range_end(&self) -> u32 {
        self.inner.get("updateRangeEnd").as_::<u32>()
    }
}
impl TextUpdateEvent {
    /// Getter of the `text` attribute.
    /// [`TextUpdateEvent.text`](https://developer.mozilla.org/en-US/docs/Web/API/TextUpdateEvent/text)
    pub fn text(&self) -> JsString {
        self.inner.get("text").as_::<JsString>()
    }
}
impl TextUpdateEvent {
    /// Getter of the `selectionStart` attribute.
    /// [`TextUpdateEvent.selectionStart`](https://developer.mozilla.org/en-US/docs/Web/API/TextUpdateEvent/selectionStart)
    pub fn selection_start(&self) -> u32 {
        self.inner.get("selectionStart").as_::<u32>()
    }
}
impl TextUpdateEvent {
    /// Getter of the `selectionEnd` attribute.
    /// [`TextUpdateEvent.selectionEnd`](https://developer.mozilla.org/en-US/docs/Web/API/TextUpdateEvent/selectionEnd)
    pub fn selection_end(&self) -> u32 {
        self.inner.get("selectionEnd").as_::<u32>()
    }
}

impl TextUpdateEvent {
    /// The `new TextUpdateEvent(..)` constructor, creating a new TextUpdateEvent instance
    pub fn new0(type_: &JsString) -> TextUpdateEvent {
        Self {
            inner: Any::global("TextUpdateEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new TextUpdateEvent(..)` constructor, creating a new TextUpdateEvent instance
    pub fn new1(type_: &JsString, options: &TextUpdateEventInit) -> TextUpdateEvent {
        Self {
            inner: Any::global("TextUpdateEvent")
                .new(&[type_.into(), options.into()])
                .as_::<Event>(),
        }
    }
}
