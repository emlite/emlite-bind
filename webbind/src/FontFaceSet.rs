use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FontFaceSet {
    inner: EventTarget,
}
impl FromVal for FontFaceSet {
    fn from_val(v: &emlite::Val) -> Self {
        FontFaceSet {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FontFaceSet {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FontFaceSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<FontFaceSet> for emlite::Val {
    fn from(s: FontFaceSet) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FontFaceSet {
    pub fn add(&self, font: FontFace) -> FontFaceSet {
        self.inner.call("add", &[font.into()]).as_::<FontFaceSet>()
    }
}
impl FontFaceSet {
    pub fn delete(&self, font: FontFace) -> bool {
        self.inner.call("delete", &[font.into()]).as_::<bool>()
    }
}
impl FontFaceSet {
    pub fn clear(&self) -> jsbind::Undefined {
        self.inner.call("clear", &[]).as_::<jsbind::Undefined>()
    }
}
impl FontFaceSet {
    pub fn onloading(&self) -> jsbind::Any {
        self.inner.get("onloading").as_::<jsbind::Any>()
    }

    pub fn set_onloading(&mut self, value: jsbind::Any) {
        self.inner.set("onloading", value);
    }
}
impl FontFaceSet {
    pub fn onloadingdone(&self) -> jsbind::Any {
        self.inner.get("onloadingdone").as_::<jsbind::Any>()
    }

    pub fn set_onloadingdone(&mut self, value: jsbind::Any) {
        self.inner.set("onloadingdone", value);
    }
}
impl FontFaceSet {
    pub fn onloadingerror(&self) -> jsbind::Any {
        self.inner.get("onloadingerror").as_::<jsbind::Any>()
    }

    pub fn set_onloadingerror(&mut self, value: jsbind::Any) {
        self.inner.set("onloadingerror", value);
    }
}
impl FontFaceSet {
    pub fn load0(&self, font: jsbind::CSSOMString) -> jsbind::Promise {
        self.inner
            .call("load", &[font.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn load1(&self, font: jsbind::CSSOMString, text: jsbind::CSSOMString) -> jsbind::Promise {
        self.inner
            .call("load", &[font.into(), text.into()])
            .as_::<jsbind::Promise>()
    }
}
impl FontFaceSet {
    pub fn check0(&self, font: jsbind::CSSOMString) -> bool {
        self.inner.call("check", &[font.into()]).as_::<bool>()
    }

    pub fn check1(&self, font: jsbind::CSSOMString, text: jsbind::CSSOMString) -> bool {
        self.inner
            .call("check", &[font.into(), text.into()])
            .as_::<bool>()
    }
}
impl FontFaceSet {
    pub fn ready(&self) -> jsbind::Promise {
        self.inner.get("ready").as_::<jsbind::Promise>()
    }
}
impl FontFaceSet {
    pub fn status(&self) -> FontFaceSetLoadStatus {
        self.inner.get("status").as_::<FontFaceSetLoadStatus>()
    }
}
