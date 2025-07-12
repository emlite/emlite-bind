use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for HTMLImageElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HTMLImageElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLImageElement> for emlite::Val {
    fn from(s: HTMLImageElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

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
    pub fn alt(&self) -> jsbind::DOMString {
        self.inner.get("alt").as_::<jsbind::DOMString>()
    }

    pub fn set_alt(&mut self, value: jsbind::DOMString) {
        self.inner.set("alt", value);
    }
}
impl HTMLImageElement {
    pub fn src(&self) -> jsbind::USVString {
        self.inner.get("src").as_::<jsbind::USVString>()
    }

    pub fn set_src(&mut self, value: jsbind::USVString) {
        self.inner.set("src", value);
    }
}
impl HTMLImageElement {
    pub fn srcset(&self) -> jsbind::USVString {
        self.inner.get("srcset").as_::<jsbind::USVString>()
    }

    pub fn set_srcset(&mut self, value: jsbind::USVString) {
        self.inner.set("srcset", value);
    }
}
impl HTMLImageElement {
    pub fn sizes(&self) -> jsbind::DOMString {
        self.inner.get("sizes").as_::<jsbind::DOMString>()
    }

    pub fn set_sizes(&mut self, value: jsbind::DOMString) {
        self.inner.set("sizes", value);
    }
}
impl HTMLImageElement {
    pub fn cross_origin(&self) -> jsbind::DOMString {
        self.inner.get("crossOrigin").as_::<jsbind::DOMString>()
    }

    pub fn set_cross_origin(&mut self, value: jsbind::DOMString) {
        self.inner.set("crossOrigin", value);
    }
}
impl HTMLImageElement {
    pub fn use_map(&self) -> jsbind::DOMString {
        self.inner.get("useMap").as_::<jsbind::DOMString>()
    }

    pub fn set_use_map(&mut self, value: jsbind::DOMString) {
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
    pub fn current_src(&self) -> jsbind::USVString {
        self.inner.get("currentSrc").as_::<jsbind::USVString>()
    }
}
impl HTMLImageElement {
    pub fn referrer_policy(&self) -> jsbind::DOMString {
        self.inner.get("referrerPolicy").as_::<jsbind::DOMString>()
    }

    pub fn set_referrer_policy(&mut self, value: jsbind::DOMString) {
        self.inner.set("referrerPolicy", value);
    }
}
impl HTMLImageElement {
    pub fn decoding(&self) -> jsbind::DOMString {
        self.inner.get("decoding").as_::<jsbind::DOMString>()
    }

    pub fn set_decoding(&mut self, value: jsbind::DOMString) {
        self.inner.set("decoding", value);
    }
}
impl HTMLImageElement {
    pub fn loading(&self) -> jsbind::DOMString {
        self.inner.get("loading").as_::<jsbind::DOMString>()
    }

    pub fn set_loading(&mut self, value: jsbind::DOMString) {
        self.inner.set("loading", value);
    }
}
impl HTMLImageElement {
    pub fn fetch_priority(&self) -> jsbind::DOMString {
        self.inner.get("fetchPriority").as_::<jsbind::DOMString>()
    }

    pub fn set_fetch_priority(&mut self, value: jsbind::DOMString) {
        self.inner.set("fetchPriority", value);
    }
}
impl HTMLImageElement {
    pub fn decode(&self) -> jsbind::Promise {
        self.inner.call("decode", &[]).as_::<jsbind::Promise>()
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
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
    }

    pub fn set_name(&mut self, value: jsbind::DOMString) {
        self.inner.set("name", value);
    }
}
impl HTMLImageElement {
    pub fn lowsrc(&self) -> jsbind::USVString {
        self.inner.get("lowsrc").as_::<jsbind::USVString>()
    }

    pub fn set_lowsrc(&mut self, value: jsbind::USVString) {
        self.inner.set("lowsrc", value);
    }
}
impl HTMLImageElement {
    pub fn align(&self) -> jsbind::DOMString {
        self.inner.get("align").as_::<jsbind::DOMString>()
    }

    pub fn set_align(&mut self, value: jsbind::DOMString) {
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
    pub fn long_desc(&self) -> jsbind::USVString {
        self.inner.get("longDesc").as_::<jsbind::USVString>()
    }

    pub fn set_long_desc(&mut self, value: jsbind::USVString) {
        self.inner.set("longDesc", value);
    }
}
impl HTMLImageElement {
    pub fn border(&self) -> jsbind::DOMString {
        self.inner.get("border").as_::<jsbind::DOMString>()
    }

    pub fn set_border(&mut self, value: jsbind::DOMString) {
        self.inner.set("border", value);
    }
}
impl HTMLImageElement {
    pub fn attribution_src(&self) -> jsbind::USVString {
        self.inner.get("attributionSrc").as_::<jsbind::USVString>()
    }

    pub fn set_attribution_src(&mut self, value: jsbind::USVString) {
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
