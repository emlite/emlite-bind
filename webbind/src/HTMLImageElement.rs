use super::*;

/// The HTMLImageElement class.
/// [`HTMLImageElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLImageElement {
    inner: HTMLElement,
}
impl FromVal for HTMLImageElement {
    fn from_val(v: &Any) -> Self {
        HTMLImageElement {
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
impl core::ops::Deref for HTMLImageElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLImageElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HTMLImageElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLImageElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLImageElement> for Any {
    fn from(s: HTMLImageElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLImageElement> for Any {
    fn from(s: &HTMLImageElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLImageElement);

impl HTMLImageElement {
    /// The `new HTMLImageElement(..)` constructor, creating a new HTMLImageElement instance
    pub fn new() -> HTMLImageElement {
        Self {
            inner: Any::global("HTMLImageElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLImageElement {
    /// Getter of the `alt` attribute.
    /// [`HTMLImageElement.alt`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/alt)
    pub fn alt(&self) -> String {
        self.inner.get("alt").as_::<String>()
    }

    /// Setter of the `alt` attribute.
    /// [`HTMLImageElement.alt`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/alt)
    pub fn set_alt(&mut self, value: &str) {
        self.inner.set("alt", value);
    }
}
impl HTMLImageElement {
    /// Getter of the `src` attribute.
    /// [`HTMLImageElement.src`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/src)
    pub fn src(&self) -> String {
        self.inner.get("src").as_::<String>()
    }

    /// Setter of the `src` attribute.
    /// [`HTMLImageElement.src`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/src)
    pub fn set_src(&mut self, value: &str) {
        self.inner.set("src", value);
    }
}
impl HTMLImageElement {
    /// Getter of the `srcset` attribute.
    /// [`HTMLImageElement.srcset`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/srcset)
    pub fn srcset(&self) -> String {
        self.inner.get("srcset").as_::<String>()
    }

    /// Setter of the `srcset` attribute.
    /// [`HTMLImageElement.srcset`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/srcset)
    pub fn set_srcset(&mut self, value: &str) {
        self.inner.set("srcset", value);
    }
}
impl HTMLImageElement {
    /// Getter of the `sizes` attribute.
    /// [`HTMLImageElement.sizes`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/sizes)
    pub fn sizes(&self) -> String {
        self.inner.get("sizes").as_::<String>()
    }

    /// Setter of the `sizes` attribute.
    /// [`HTMLImageElement.sizes`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/sizes)
    pub fn set_sizes(&mut self, value: &str) {
        self.inner.set("sizes", value);
    }
}
impl HTMLImageElement {
    /// Getter of the `crossOrigin` attribute.
    /// [`HTMLImageElement.crossOrigin`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/crossOrigin)
    pub fn cross_origin(&self) -> String {
        self.inner.get("crossOrigin").as_::<String>()
    }

    /// Setter of the `crossOrigin` attribute.
    /// [`HTMLImageElement.crossOrigin`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/crossOrigin)
    pub fn set_cross_origin(&mut self, value: &str) {
        self.inner.set("crossOrigin", value);
    }
}
impl HTMLImageElement {
    /// Getter of the `useMap` attribute.
    /// [`HTMLImageElement.useMap`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/useMap)
    pub fn use_map(&self) -> String {
        self.inner.get("useMap").as_::<String>()
    }

    /// Setter of the `useMap` attribute.
    /// [`HTMLImageElement.useMap`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/useMap)
    pub fn set_use_map(&mut self, value: &str) {
        self.inner.set("useMap", value);
    }
}
impl HTMLImageElement {
    /// Getter of the `isMap` attribute.
    /// [`HTMLImageElement.isMap`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/isMap)
    pub fn is_map(&self) -> bool {
        self.inner.get("isMap").as_::<bool>()
    }

    /// Setter of the `isMap` attribute.
    /// [`HTMLImageElement.isMap`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/isMap)
    pub fn set_is_map(&mut self, value: bool) {
        self.inner.set("isMap", value);
    }
}
impl HTMLImageElement {
    /// Getter of the `width` attribute.
    /// [`HTMLImageElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/width)
    pub fn width(&self) -> u32 {
        self.inner.get("width").as_::<u32>()
    }

    /// Setter of the `width` attribute.
    /// [`HTMLImageElement.width`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/width)
    pub fn set_width(&mut self, value: u32) {
        self.inner.set("width", value);
    }
}
impl HTMLImageElement {
    /// Getter of the `height` attribute.
    /// [`HTMLImageElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/height)
    pub fn height(&self) -> u32 {
        self.inner.get("height").as_::<u32>()
    }

    /// Setter of the `height` attribute.
    /// [`HTMLImageElement.height`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/height)
    pub fn set_height(&mut self, value: u32) {
        self.inner.set("height", value);
    }
}
impl HTMLImageElement {
    /// Getter of the `naturalWidth` attribute.
    /// [`HTMLImageElement.naturalWidth`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/naturalWidth)
    pub fn natural_width(&self) -> u32 {
        self.inner.get("naturalWidth").as_::<u32>()
    }
}
impl HTMLImageElement {
    /// Getter of the `naturalHeight` attribute.
    /// [`HTMLImageElement.naturalHeight`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/naturalHeight)
    pub fn natural_height(&self) -> u32 {
        self.inner.get("naturalHeight").as_::<u32>()
    }
}
impl HTMLImageElement {
    /// Getter of the `complete` attribute.
    /// [`HTMLImageElement.complete`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/complete)
    pub fn complete(&self) -> bool {
        self.inner.get("complete").as_::<bool>()
    }
}
impl HTMLImageElement {
    /// Getter of the `currentSrc` attribute.
    /// [`HTMLImageElement.currentSrc`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/currentSrc)
    pub fn current_src(&self) -> String {
        self.inner.get("currentSrc").as_::<String>()
    }
}
impl HTMLImageElement {
    /// Getter of the `referrerPolicy` attribute.
    /// [`HTMLImageElement.referrerPolicy`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/referrerPolicy)
    pub fn referrer_policy(&self) -> String {
        self.inner.get("referrerPolicy").as_::<String>()
    }

    /// Setter of the `referrerPolicy` attribute.
    /// [`HTMLImageElement.referrerPolicy`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/referrerPolicy)
    pub fn set_referrer_policy(&mut self, value: &str) {
        self.inner.set("referrerPolicy", value);
    }
}
impl HTMLImageElement {
    /// Getter of the `decoding` attribute.
    /// [`HTMLImageElement.decoding`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/decoding)
    pub fn decoding(&self) -> String {
        self.inner.get("decoding").as_::<String>()
    }

    /// Setter of the `decoding` attribute.
    /// [`HTMLImageElement.decoding`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/decoding)
    pub fn set_decoding(&mut self, value: &str) {
        self.inner.set("decoding", value);
    }
}
impl HTMLImageElement {
    /// Getter of the `loading` attribute.
    /// [`HTMLImageElement.loading`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/loading)
    pub fn loading(&self) -> String {
        self.inner.get("loading").as_::<String>()
    }

    /// Setter of the `loading` attribute.
    /// [`HTMLImageElement.loading`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/loading)
    pub fn set_loading(&mut self, value: &str) {
        self.inner.set("loading", value);
    }
}
impl HTMLImageElement {
    /// Getter of the `fetchPriority` attribute.
    /// [`HTMLImageElement.fetchPriority`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/fetchPriority)
    pub fn fetch_priority(&self) -> String {
        self.inner.get("fetchPriority").as_::<String>()
    }

    /// Setter of the `fetchPriority` attribute.
    /// [`HTMLImageElement.fetchPriority`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/fetchPriority)
    pub fn set_fetch_priority(&mut self, value: &str) {
        self.inner.set("fetchPriority", value);
    }
}
impl HTMLImageElement {
    /// The decode method.
    /// [`HTMLImageElement.decode`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/decode)
    pub fn decode(&self) -> Promise<Undefined> {
        self.inner.call("decode", &[]).as_::<Promise<Undefined>>()
    }
}
impl HTMLImageElement {
    /// Getter of the `x` attribute.
    /// [`HTMLImageElement.x`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/x)
    pub fn x(&self) -> i32 {
        self.inner.get("x").as_::<i32>()
    }
}
impl HTMLImageElement {
    /// Getter of the `y` attribute.
    /// [`HTMLImageElement.y`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/y)
    pub fn y(&self) -> i32 {
        self.inner.get("y").as_::<i32>()
    }
}
impl HTMLImageElement {
    /// Getter of the `name` attribute.
    /// [`HTMLImageElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/name)
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }

    /// Setter of the `name` attribute.
    /// [`HTMLImageElement.name`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/name)
    pub fn set_name(&mut self, value: &str) {
        self.inner.set("name", value);
    }
}
impl HTMLImageElement {
    /// Getter of the `lowsrc` attribute.
    /// [`HTMLImageElement.lowsrc`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/lowsrc)
    pub fn lowsrc(&self) -> String {
        self.inner.get("lowsrc").as_::<String>()
    }

    /// Setter of the `lowsrc` attribute.
    /// [`HTMLImageElement.lowsrc`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/lowsrc)
    pub fn set_lowsrc(&mut self, value: &str) {
        self.inner.set("lowsrc", value);
    }
}
impl HTMLImageElement {
    /// Getter of the `align` attribute.
    /// [`HTMLImageElement.align`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/align)
    pub fn align(&self) -> String {
        self.inner.get("align").as_::<String>()
    }

    /// Setter of the `align` attribute.
    /// [`HTMLImageElement.align`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/align)
    pub fn set_align(&mut self, value: &str) {
        self.inner.set("align", value);
    }
}
impl HTMLImageElement {
    /// Getter of the `hspace` attribute.
    /// [`HTMLImageElement.hspace`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/hspace)
    pub fn hspace(&self) -> u32 {
        self.inner.get("hspace").as_::<u32>()
    }

    /// Setter of the `hspace` attribute.
    /// [`HTMLImageElement.hspace`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/hspace)
    pub fn set_hspace(&mut self, value: u32) {
        self.inner.set("hspace", value);
    }
}
impl HTMLImageElement {
    /// Getter of the `vspace` attribute.
    /// [`HTMLImageElement.vspace`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/vspace)
    pub fn vspace(&self) -> u32 {
        self.inner.get("vspace").as_::<u32>()
    }

    /// Setter of the `vspace` attribute.
    /// [`HTMLImageElement.vspace`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/vspace)
    pub fn set_vspace(&mut self, value: u32) {
        self.inner.set("vspace", value);
    }
}
impl HTMLImageElement {
    /// Getter of the `longDesc` attribute.
    /// [`HTMLImageElement.longDesc`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/longDesc)
    pub fn long_desc(&self) -> String {
        self.inner.get("longDesc").as_::<String>()
    }

    /// Setter of the `longDesc` attribute.
    /// [`HTMLImageElement.longDesc`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/longDesc)
    pub fn set_long_desc(&mut self, value: &str) {
        self.inner.set("longDesc", value);
    }
}
impl HTMLImageElement {
    /// Getter of the `border` attribute.
    /// [`HTMLImageElement.border`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/border)
    pub fn border(&self) -> String {
        self.inner.get("border").as_::<String>()
    }

    /// Setter of the `border` attribute.
    /// [`HTMLImageElement.border`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/border)
    pub fn set_border(&mut self, value: &str) {
        self.inner.set("border", value);
    }
}
impl HTMLImageElement {
    /// Getter of the `attributionSrc` attribute.
    /// [`HTMLImageElement.attributionSrc`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/attributionSrc)
    pub fn attribution_src(&self) -> String {
        self.inner.get("attributionSrc").as_::<String>()
    }

    /// Setter of the `attributionSrc` attribute.
    /// [`HTMLImageElement.attributionSrc`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/attributionSrc)
    pub fn set_attribution_src(&mut self, value: &str) {
        self.inner.set("attributionSrc", value);
    }
}
impl HTMLImageElement {
    /// Getter of the `sharedStorageWritable` attribute.
    /// [`HTMLImageElement.sharedStorageWritable`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/sharedStorageWritable)
    pub fn shared_storage_writable(&self) -> bool {
        self.inner.get("sharedStorageWritable").as_::<bool>()
    }

    /// Setter of the `sharedStorageWritable` attribute.
    /// [`HTMLImageElement.sharedStorageWritable`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLImageElement/sharedStorageWritable)
    pub fn set_shared_storage_writable(&mut self, value: bool) {
        self.inner.set("sharedStorageWritable", value);
    }
}
