use super::*;

/// The HTMLTrackElement class.
/// [`HTMLTrackElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLTrackElement {
    inner: HTMLElement,
}

impl FromVal for HTMLTrackElement {
    fn from_val(v: &Any) -> Self {
        HTMLTrackElement {
            inner: HTMLElement::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HTMLTrackElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HTMLTrackElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HTMLTrackElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLTrackElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<HTMLTrackElement> for Any {
    fn from(s: HTMLTrackElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLTrackElement> for Any {
    fn from(s: &HTMLTrackElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLTrackElement);

impl HTMLTrackElement {
    /// Getter of the `kind` attribute.
    /// [`HTMLTrackElement.kind`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/kind)
    pub fn kind(&self) -> JsString {
        self.inner.get("kind").as_::<JsString>()
    }

    /// Setter of the `kind` attribute.
    /// [`HTMLTrackElement.kind`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/kind)
    pub fn set_kind(&mut self, value: &JsString) {
        self.inner.set("kind", value);
    }
}
impl HTMLTrackElement {
    /// Getter of the `src` attribute.
    /// [`HTMLTrackElement.src`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/src)
    pub fn src(&self) -> JsString {
        self.inner.get("src").as_::<JsString>()
    }

    /// Setter of the `src` attribute.
    /// [`HTMLTrackElement.src`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/src)
    pub fn set_src(&mut self, value: &JsString) {
        self.inner.set("src", value);
    }
}
impl HTMLTrackElement {
    /// Getter of the `srclang` attribute.
    /// [`HTMLTrackElement.srclang`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/srclang)
    pub fn srclang(&self) -> JsString {
        self.inner.get("srclang").as_::<JsString>()
    }

    /// Setter of the `srclang` attribute.
    /// [`HTMLTrackElement.srclang`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/srclang)
    pub fn set_srclang(&mut self, value: &JsString) {
        self.inner.set("srclang", value);
    }
}
impl HTMLTrackElement {
    /// Getter of the `label` attribute.
    /// [`HTMLTrackElement.label`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/label)
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }

    /// Setter of the `label` attribute.
    /// [`HTMLTrackElement.label`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/label)
    pub fn set_label(&mut self, value: &JsString) {
        self.inner.set("label", value);
    }
}
impl HTMLTrackElement {
    /// Getter of the `default` attribute.
    /// [`HTMLTrackElement.default`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/default)
    pub fn default(&self) -> bool {
        self.inner.get("default").as_::<bool>()
    }

    /// Setter of the `default` attribute.
    /// [`HTMLTrackElement.default`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/default)
    pub fn set_default(&mut self, value: bool) {
        self.inner.set("default", value);
    }
}
impl HTMLTrackElement {
    /// Getter of the `readyState` attribute.
    /// [`HTMLTrackElement.readyState`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/readyState)
    pub fn ready_state(&self) -> u16 {
        self.inner.get("readyState").as_::<u16>()
    }
}
impl HTMLTrackElement {
    /// Getter of the `track` attribute.
    /// [`HTMLTrackElement.track`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTrackElement/track)
    pub fn track(&self) -> TextTrack {
        self.inner.get("track").as_::<TextTrack>()
    }
}

impl HTMLTrackElement {
    /// The `new HTMLTrackElement(..)` constructor, creating a new HTMLTrackElement instance
    pub fn new() -> HTMLTrackElement {
        Self {
            inner: Any::global("HTMLTrackElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
