use super::*;

/// The HTMLCanvasElement class.
/// [`HTMLCanvasElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLCanvasElement {
    inner: HTMLElement,
}
impl FromVal for HTMLCanvasElement {
    fn from_val(v: &Any) -> Self {
        HTMLCanvasElement {
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
impl core::ops::Deref for HTMLCanvasElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLCanvasElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLCanvasElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLCanvasElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLCanvasElement> for Any {
    fn from(s: HTMLCanvasElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLCanvasElement> for Any {
    fn from(s: &HTMLCanvasElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLCanvasElement);

impl HTMLCanvasElement {
    /// The `new HTMLCanvasElement(..)` constructor, creating a new HTMLCanvasElement instance
    pub fn new() -> HTMLCanvasElement {
        Self {
            inner: Any::global("HTMLCanvasElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLCanvasElement {
    /// Getter of the `width` attribute.
    /// [`HTMLCanvasElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/width)
    pub fn width(&self) -> u32 {
        self.inner.get("width").as_::<u32>()
    }

    /// Setter of the `width` attribute.
    /// [`HTMLCanvasElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/width)
    pub fn set_width(&mut self, value: u32) {
        self.inner.set("width", value);
    }
}
impl HTMLCanvasElement {
    /// Getter of the `height` attribute.
    /// [`HTMLCanvasElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/height)
    pub fn height(&self) -> u32 {
        self.inner.get("height").as_::<u32>()
    }

    /// Setter of the `height` attribute.
    /// [`HTMLCanvasElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/height)
    pub fn set_height(&mut self, value: u32) {
        self.inner.set("height", value);
    }
}
impl HTMLCanvasElement {
    /// The getContext method.
    /// [`HTMLCanvasElement.getContext`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/getContext)
    pub fn get_context0(&self, context_id: &DOMString) -> Any {
        self.inner
            .call("getContext", &[context_id.into()])
            .as_::<Any>()
    }
    /// The getContext method.
    /// [`HTMLCanvasElement.getContext`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/getContext)
    pub fn get_context1(&self, context_id: &DOMString, options: &Any) -> Any {
        self.inner
            .call("getContext", &[context_id.into(), options.into()])
            .as_::<Any>()
    }
}
impl HTMLCanvasElement {
    /// The toDataURL method.
    /// [`HTMLCanvasElement.toDataURL`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/toDataURL)
    pub fn to_data_url0(&self) -> USVString {
        self.inner.call("toDataURL", &[]).as_::<USVString>()
    }
    /// The toDataURL method.
    /// [`HTMLCanvasElement.toDataURL`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/toDataURL)
    pub fn to_data_url1(&self, type_: &DOMString) -> USVString {
        self.inner
            .call("toDataURL", &[type_.into()])
            .as_::<USVString>()
    }
    /// The toDataURL method.
    /// [`HTMLCanvasElement.toDataURL`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/toDataURL)
    pub fn to_data_url2(&self, type_: &DOMString, quality: &Any) -> USVString {
        self.inner
            .call("toDataURL", &[type_.into(), quality.into()])
            .as_::<USVString>()
    }
}
impl HTMLCanvasElement {
    /// The toBlob method.
    /// [`HTMLCanvasElement.toBlob`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/toBlob)
    pub fn to_blob0(&self, callback: &Function) -> Undefined {
        self.inner
            .call("toBlob", &[callback.into()])
            .as_::<Undefined>()
    }
    /// The toBlob method.
    /// [`HTMLCanvasElement.toBlob`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/toBlob)
    pub fn to_blob1(&self, callback: &Function, type_: &DOMString) -> Undefined {
        self.inner
            .call("toBlob", &[callback.into(), type_.into()])
            .as_::<Undefined>()
    }
    /// The toBlob method.
    /// [`HTMLCanvasElement.toBlob`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/toBlob)
    pub fn to_blob2(&self, callback: &Function, type_: &DOMString, quality: &Any) -> Undefined {
        self.inner
            .call("toBlob", &[callback.into(), type_.into(), quality.into()])
            .as_::<Undefined>()
    }
}
impl HTMLCanvasElement {
    /// The transferControlToOffscreen method.
    /// [`HTMLCanvasElement.transferControlToOffscreen`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/transferControlToOffscreen)
    pub fn transfer_control_to_offscreen(&self) -> OffscreenCanvas {
        self.inner
            .call("transferControlToOffscreen", &[])
            .as_::<OffscreenCanvas>()
    }
}
impl HTMLCanvasElement {
    /// The captureStream method.
    /// [`HTMLCanvasElement.captureStream`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/captureStream)
    pub fn capture_stream0(&self) -> MediaStream {
        self.inner.call("captureStream", &[]).as_::<MediaStream>()
    }
    /// The captureStream method.
    /// [`HTMLCanvasElement.captureStream`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/captureStream)
    pub fn capture_stream1(&self, frame_request_rate: f64) -> MediaStream {
        self.inner
            .call("captureStream", &[frame_request_rate.into()])
            .as_::<MediaStream>()
    }
}
