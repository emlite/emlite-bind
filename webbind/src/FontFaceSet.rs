use super::*;

/// The FontFaceSet class.
/// [`FontFaceSet`](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FontFaceSet {
    inner: EventTarget,
}

impl FromVal for FontFaceSet {
    fn from_val(v: &Any) -> Self {
        FontFaceSet {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for FontFaceSet {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for FontFaceSet {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<FontFaceSet> for Any {
    fn from(s: FontFaceSet) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&FontFaceSet> for Any {
    fn from(s: &FontFaceSet) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(FontFaceSet);

impl FontFaceSet {
    /// The add method.
    /// [`FontFaceSet.add`](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/add)
    pub fn add(&self, font: &FontFace) -> FontFaceSet {
        self.inner.call("add", &[font.into()]).as_::<FontFaceSet>()
    }
}
impl FontFaceSet {
    /// The delete method.
    /// [`FontFaceSet.delete`](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/delete)
    pub fn delete(&self, font: &FontFace) -> bool {
        self.inner.call("delete", &[font.into()]).as_::<bool>()
    }
}
impl FontFaceSet {
    /// The clear method.
    /// [`FontFaceSet.clear`](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/clear)
    pub fn clear(&self) -> Undefined {
        self.inner.call("clear", &[]).as_::<Undefined>()
    }
}
impl FontFaceSet {
    /// Getter of the `onloading` attribute.
    /// [`FontFaceSet.onloading`](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/onloading)
    pub fn onloading(&self) -> Any {
        self.inner.get("onloading").as_::<Any>()
    }

    /// Setter of the `onloading` attribute.
    /// [`FontFaceSet.onloading`](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/onloading)
    pub fn set_onloading(&mut self, value: &Any) {
        self.inner.set("onloading", value);
    }
}
impl FontFaceSet {
    /// Getter of the `onloadingdone` attribute.
    /// [`FontFaceSet.onloadingdone`](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/onloadingdone)
    pub fn onloadingdone(&self) -> Any {
        self.inner.get("onloadingdone").as_::<Any>()
    }

    /// Setter of the `onloadingdone` attribute.
    /// [`FontFaceSet.onloadingdone`](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/onloadingdone)
    pub fn set_onloadingdone(&mut self, value: &Any) {
        self.inner.set("onloadingdone", value);
    }
}
impl FontFaceSet {
    /// Getter of the `onloadingerror` attribute.
    /// [`FontFaceSet.onloadingerror`](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/onloadingerror)
    pub fn onloadingerror(&self) -> Any {
        self.inner.get("onloadingerror").as_::<Any>()
    }

    /// Setter of the `onloadingerror` attribute.
    /// [`FontFaceSet.onloadingerror`](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/onloadingerror)
    pub fn set_onloadingerror(&mut self, value: &Any) {
        self.inner.set("onloadingerror", value);
    }
}
impl FontFaceSet {
    /// The load method.
    /// [`FontFaceSet.load`](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/load)
    pub fn load0(&self, font: &JsString) -> Promise<TypedArray<FontFace>> {
        self.inner
            .call("load", &[font.into()])
            .as_::<Promise<TypedArray<FontFace>>>()
    }
    /// The load method.
    /// [`FontFaceSet.load`](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/load)
    pub fn load1(&self, font: &JsString, text: &JsString) -> Promise<TypedArray<FontFace>> {
        self.inner
            .call("load", &[font.into(), text.into()])
            .as_::<Promise<TypedArray<FontFace>>>()
    }
}
impl FontFaceSet {
    /// The check method.
    /// [`FontFaceSet.check`](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/check)
    pub fn check0(&self, font: &JsString) -> bool {
        self.inner.call("check", &[font.into()]).as_::<bool>()
    }
    /// The check method.
    /// [`FontFaceSet.check`](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/check)
    pub fn check1(&self, font: &JsString, text: &JsString) -> bool {
        self.inner
            .call("check", &[font.into(), text.into()])
            .as_::<bool>()
    }
}
impl FontFaceSet {
    /// Getter of the `ready` attribute.
    /// [`FontFaceSet.ready`](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/ready)
    pub fn ready(&self) -> Promise<FontFaceSet> {
        self.inner.get("ready").as_::<Promise<FontFaceSet>>()
    }
}
impl FontFaceSet {
    /// Getter of the `status` attribute.
    /// [`FontFaceSet.status`](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/status)
    pub fn status(&self) -> FontFaceSetLoadStatus {
        self.inner.get("status").as_::<FontFaceSetLoadStatus>()
    }
}
