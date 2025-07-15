use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLFencedFrameElement {
    inner: HTMLElement,
}
impl FromVal for HTMLFencedFrameElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLFencedFrameElement {
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
impl core::ops::Deref for HTMLFencedFrameElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLFencedFrameElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLFencedFrameElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLFencedFrameElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLFencedFrameElement> for emlite::Val {
    fn from(s: HTMLFencedFrameElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&HTMLFencedFrameElement> for emlite::Val {
    fn from(s: &HTMLFencedFrameElement) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLFencedFrameElement);

impl HTMLFencedFrameElement {
    pub fn new() -> HTMLFencedFrameElement {
        Self {
            inner: emlite::Val::global("HTMLFencedFrameElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLFencedFrameElement {
    pub fn config(&self) -> FencedFrameConfig {
        self.inner.get("config").as_::<FencedFrameConfig>()
    }

    pub fn set_config(&mut self, value: &FencedFrameConfig) {
        self.inner.set("config", value);
    }
}
impl HTMLFencedFrameElement {
    pub fn width(&self) -> String {
        self.inner.get("width").as_::<String>()
    }

    pub fn set_width(&mut self, value: &str) {
        self.inner.set("width", value);
    }
}
impl HTMLFencedFrameElement {
    pub fn height(&self) -> String {
        self.inner.get("height").as_::<String>()
    }

    pub fn set_height(&mut self, value: &str) {
        self.inner.set("height", value);
    }
}
impl HTMLFencedFrameElement {
    pub fn sandbox(&self) -> DOMTokenList {
        self.inner.get("sandbox").as_::<DOMTokenList>()
    }
}
impl HTMLFencedFrameElement {
    pub fn allow(&self) -> String {
        self.inner.get("allow").as_::<String>()
    }

    pub fn set_allow(&mut self, value: &str) {
        self.inner.set("allow", value);
    }
}
