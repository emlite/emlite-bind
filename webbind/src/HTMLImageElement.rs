use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLImageElement {
    inner: HTMLElement,
}
impl FromVal for HTMLImageElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLImageElement {
            inner: HTMLElement::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for HTMLImageElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLImageElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLImageElement> for emlite::Val {
    fn from(s: HTMLImageElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&HTMLImageElement> for emlite::Val {
    fn from(s: &HTMLImageElement) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLImageElement);

impl HTMLImageElement {
    pub fn new() -> HTMLImageElement {
        Self {
            inner: emlite::Val::global("HTMLImageElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLImageElement {
    pub fn alt(&self) -> String {
        self.inner.get("alt").as_::<String>()
    }

    pub fn set_alt(&mut self, value: &str) {
        self.inner.set("alt", value);
    }
}
impl HTMLImageElement {
    pub fn src(&self) -> String {
        self.inner.get("src").as_::<String>()
    }

    pub fn set_src(&mut self, value: &str) {
        self.inner.set("src", value);
    }
}
impl HTMLImageElement {
    pub fn srcset(&self) -> String {
        self.inner.get("srcset").as_::<String>()
    }

    pub fn set_srcset(&mut self, value: &str) {
        self.inner.set("srcset", value);
    }
}
impl HTMLImageElement {
    pub fn sizes(&self) -> String {
        self.inner.get("sizes").as_::<String>()
    }

    pub fn set_sizes(&mut self, value: &str) {
        self.inner.set("sizes", value);
    }
}
impl HTMLImageElement {
    pub fn cross_origin(&self) -> String {
        self.inner.get("crossOrigin").as_::<String>()
    }

    pub fn set_cross_origin(&mut self, value: &str) {
        self.inner.set("crossOrigin", value);
    }
}
impl HTMLImageElement {
    pub fn use_map(&self) -> String {
        self.inner.get("useMap").as_::<String>()
    }

    pub fn set_use_map(&mut self, value: &str) {
        self.inner.set("useMap", value);
    }
}
impl HTMLImageElement {
    pub fn is_map(&self) -> bool {
        self.inner.get("isMap").as_::<bool>()
    }

    pub fn set_is_map(&mut self, value: bool) {
        self.inner.set("isMap", value);
    }
}
impl HTMLImageElement {
    pub fn width(&self) -> u32 {
        self.inner.get("width").as_::<u32>()
    }

    pub fn set_width(&mut self, value: u32) {
        self.inner.set("width", value);
    }
}
impl HTMLImageElement {
    pub fn height(&self) -> u32 {
        self.inner.get("height").as_::<u32>()
    }

    pub fn set_height(&mut self, value: u32) {
        self.inner.set("height", value);
    }
}
impl HTMLImageElement {
    pub fn natural_width(&self) -> u32 {
        self.inner.get("naturalWidth").as_::<u32>()
    }
}
impl HTMLImageElement {
    pub fn natural_height(&self) -> u32 {
        self.inner.get("naturalHeight").as_::<u32>()
    }
}
impl HTMLImageElement {
    pub fn complete(&self) -> bool {
        self.inner.get("complete").as_::<bool>()
    }
}
impl HTMLImageElement {
    pub fn current_src(&self) -> String {
        self.inner.get("currentSrc").as_::<String>()
    }
}
impl HTMLImageElement {
    pub fn referrer_policy(&self) -> String {
        self.inner.get("referrerPolicy").as_::<String>()
    }

    pub fn set_referrer_policy(&mut self, value: &str) {
        self.inner.set("referrerPolicy", value);
    }
}
impl HTMLImageElement {
    pub fn decoding(&self) -> String {
        self.inner.get("decoding").as_::<String>()
    }

    pub fn set_decoding(&mut self, value: &str) {
        self.inner.set("decoding", value);
    }
}
impl HTMLImageElement {
    pub fn loading(&self) -> String {
        self.inner.get("loading").as_::<String>()
    }

    pub fn set_loading(&mut self, value: &str) {
        self.inner.set("loading", value);
    }
}
impl HTMLImageElement {
    pub fn fetch_priority(&self) -> String {
        self.inner.get("fetchPriority").as_::<String>()
    }

    pub fn set_fetch_priority(&mut self, value: &str) {
        self.inner.set("fetchPriority", value);
    }
}
impl HTMLImageElement {
    pub fn decode(&self) -> Promise {
        self.inner.call("decode", &[]).as_::<Promise>()
    }
}
impl HTMLImageElement {
    pub fn x(&self) -> i32 {
        self.inner.get("x").as_::<i32>()
    }
}
impl HTMLImageElement {
    pub fn y(&self) -> i32 {
        self.inner.get("y").as_::<i32>()
    }
}
impl HTMLImageElement {
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }

    pub fn set_name(&mut self, value: &str) {
        self.inner.set("name", value);
    }
}
impl HTMLImageElement {
    pub fn lowsrc(&self) -> String {
        self.inner.get("lowsrc").as_::<String>()
    }

    pub fn set_lowsrc(&mut self, value: &str) {
        self.inner.set("lowsrc", value);
    }
}
impl HTMLImageElement {
    pub fn align(&self) -> String {
        self.inner.get("align").as_::<String>()
    }

    pub fn set_align(&mut self, value: &str) {
        self.inner.set("align", value);
    }
}
impl HTMLImageElement {
    pub fn hspace(&self) -> u32 {
        self.inner.get("hspace").as_::<u32>()
    }

    pub fn set_hspace(&mut self, value: u32) {
        self.inner.set("hspace", value);
    }
}
impl HTMLImageElement {
    pub fn vspace(&self) -> u32 {
        self.inner.get("vspace").as_::<u32>()
    }

    pub fn set_vspace(&mut self, value: u32) {
        self.inner.set("vspace", value);
    }
}
impl HTMLImageElement {
    pub fn long_desc(&self) -> String {
        self.inner.get("longDesc").as_::<String>()
    }

    pub fn set_long_desc(&mut self, value: &str) {
        self.inner.set("longDesc", value);
    }
}
impl HTMLImageElement {
    pub fn border(&self) -> String {
        self.inner.get("border").as_::<String>()
    }

    pub fn set_border(&mut self, value: &str) {
        self.inner.set("border", value);
    }
}
impl HTMLImageElement {
    pub fn attribution_src(&self) -> String {
        self.inner.get("attributionSrc").as_::<String>()
    }

    pub fn set_attribution_src(&mut self, value: &str) {
        self.inner.set("attributionSrc", value);
    }
}
impl HTMLImageElement {
    pub fn shared_storage_writable(&self) -> bool {
        self.inner.get("sharedStorageWritable").as_::<bool>()
    }

    pub fn set_shared_storage_writable(&mut self, value: bool) {
        self.inner.set("sharedStorageWritable", value);
    }
}
