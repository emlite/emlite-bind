use super::*;

/// The ClipboardItem class.
/// [`ClipboardItem`](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardItem)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ClipboardItem {
    inner: Any,
}

impl FromVal for ClipboardItem {
    fn from_val(v: &Any) -> Self {
        ClipboardItem {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ClipboardItem {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ClipboardItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ClipboardItem {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ClipboardItem {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ClipboardItem> for Any {
    fn from(s: ClipboardItem) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ClipboardItem> for Any {
    fn from(s: &ClipboardItem) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(ClipboardItem);

impl ClipboardItem {
    /// Getter of the `presentationStyle` attribute.
    /// [`ClipboardItem.presentationStyle`](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardItem/presentationStyle)
    pub fn presentation_style(&self) -> PresentationStyle {
        self.inner
            .get("presentationStyle")
            .as_::<PresentationStyle>()
    }
}
impl ClipboardItem {
    /// Getter of the `types` attribute.
    /// [`ClipboardItem.types`](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardItem/types)
    pub fn types(&self) -> TypedArray<JsString> {
        self.inner.get("types").as_::<TypedArray<JsString>>()
    }
}

impl ClipboardItem {
    /// The `new ClipboardItem(..)` constructor, creating a new ClipboardItem instance
    pub fn new0(items: &Record<JsString, Any>) -> ClipboardItem {
        Self {
            inner: Any::global("ClipboardItem")
                .new(&[items.into()])
                .as_::<Any>(),
        }
    }

    /// The `new ClipboardItem(..)` constructor, creating a new ClipboardItem instance
    pub fn new1(items: &Record<JsString, Any>, options: &ClipboardItemOptions) -> ClipboardItem {
        Self {
            inner: Any::global("ClipboardItem")
                .new(&[items.into(), options.into()])
                .as_::<Any>(),
        }
    }
}
impl ClipboardItem {
    /// The getType method.
    /// [`ClipboardItem.getType`](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardItem/getType)
    pub fn get_type(&self, type_: &JsString) -> Promise<Blob> {
        self.inner
            .call("getType", &[type_.into()])
            .as_::<Promise<Blob>>()
    }
}
impl ClipboardItem {
    /// The supports method.
    /// [`ClipboardItem.supports`](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardItem/supports)
    pub fn supports(type_: &JsString) -> bool {
        Any::global("ClipboardItem")
            .call("supports", &[type_.into()])
            .as_::<bool>()
    }
}
