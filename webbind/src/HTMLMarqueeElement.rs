use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLMarqueeElement {
    inner: HTMLElement,
}
impl FromVal for HTMLMarqueeElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLMarqueeElement { inner: HTMLElement::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HTMLMarqueeElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLMarqueeElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLMarqueeElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLMarqueeElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<HTMLMarqueeElement> for emlite::Val {
    fn from(s: HTMLMarqueeElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HTMLMarqueeElement);



impl HTMLMarqueeElement {
    pub fn new() -> HTMLMarqueeElement {
        Self {
            inner: emlite::Val::global("HTMLMarqueeElement").new(&[]).as_::<HTMLElement>(),
        }
    }

}
impl HTMLMarqueeElement {
    pub fn behavior(&self) -> DOMString {
        self.inner.get("behavior").as_::<DOMString>()
    }

    pub fn set_behavior(&mut self, value: DOMString) {
        self.inner.set("behavior", value);
    }

}
impl HTMLMarqueeElement {
    pub fn bg_color(&self) -> DOMString {
        self.inner.get("bgColor").as_::<DOMString>()
    }

    pub fn set_bg_color(&mut self, value: DOMString) {
        self.inner.set("bgColor", value);
    }

}
impl HTMLMarqueeElement {
    pub fn direction(&self) -> DOMString {
        self.inner.get("direction").as_::<DOMString>()
    }

    pub fn set_direction(&mut self, value: DOMString) {
        self.inner.set("direction", value);
    }

}
impl HTMLMarqueeElement {
    pub fn height(&self) -> DOMString {
        self.inner.get("height").as_::<DOMString>()
    }

    pub fn set_height(&mut self, value: DOMString) {
        self.inner.set("height", value);
    }

}
impl HTMLMarqueeElement {
    pub fn hspace(&self) -> u32 {
        self.inner.get("hspace").as_::<u32>()
    }

    pub fn set_hspace(&mut self, value: u32) {
        self.inner.set("hspace", value);
    }

}
impl HTMLMarqueeElement {
    pub fn loop_(&self) -> i32 {
        self.inner.get("loop").as_::<i32>()
    }

    pub fn set_loop_(&mut self, value: i32) {
        self.inner.set("loop", value);
    }

}
impl HTMLMarqueeElement {
    pub fn scroll_amount(&self) -> u32 {
        self.inner.get("scrollAmount").as_::<u32>()
    }

    pub fn set_scroll_amount(&mut self, value: u32) {
        self.inner.set("scrollAmount", value);
    }

}
impl HTMLMarqueeElement {
    pub fn scroll_delay(&self) -> u32 {
        self.inner.get("scrollDelay").as_::<u32>()
    }

    pub fn set_scroll_delay(&mut self, value: u32) {
        self.inner.set("scrollDelay", value);
    }

}
impl HTMLMarqueeElement {
    pub fn true_speed(&self) -> bool {
        self.inner.get("trueSpeed").as_::<bool>()
    }

    pub fn set_true_speed(&mut self, value: bool) {
        self.inner.set("trueSpeed", value);
    }

}
impl HTMLMarqueeElement {
    pub fn vspace(&self) -> u32 {
        self.inner.get("vspace").as_::<u32>()
    }

    pub fn set_vspace(&mut self, value: u32) {
        self.inner.set("vspace", value);
    }

}
impl HTMLMarqueeElement {
    pub fn width(&self) -> DOMString {
        self.inner.get("width").as_::<DOMString>()
    }

    pub fn set_width(&mut self, value: DOMString) {
        self.inner.set("width", value);
    }

}
impl HTMLMarqueeElement {
    pub fn start(&self, ) -> Undefined {
        self.inner.call("start", &[]).as_::<Undefined>()
    }

}
impl HTMLMarqueeElement {
    pub fn stop(&self, ) -> Undefined {
        self.inner.call("stop", &[]).as_::<Undefined>()
    }

}
