use super::*;




/// The CSSPageDescriptors class.
/// [`CSSPageDescriptors`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPageDescriptors)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSPageDescriptors {
    inner: CSSStyleDeclaration,
}

impl FromVal for CSSPageDescriptors {
    fn from_val(v: &Any) -> Self {
        CSSPageDescriptors { inner: CSSStyleDeclaration::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CSSPageDescriptors {
    type Target = CSSStyleDeclaration;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSPageDescriptors {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSPageDescriptors {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSPageDescriptors {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CSSPageDescriptors> for Any {
    fn from(s: CSSPageDescriptors) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSPageDescriptors> for Any {
    fn from(s: &CSSPageDescriptors) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSPageDescriptors);


impl CSSPageDescriptors {
    /// Getter of the `margin` attribute.
    /// [`CSSPageDescriptors.margin`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPageDescriptors/margin)
    pub fn margin(&self) -> JsString {
        self.inner.get("margin").as_::<JsString>()
    }

    /// Setter of the `margin` attribute.
    /// [`CSSPageDescriptors.margin`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPageDescriptors/margin)
    pub fn set_margin(&mut self, value: &JsString) {
        self.inner.set("margin", value);
    }
}
impl CSSPageDescriptors {
    /// Getter of the `marginTop` attribute.
    /// [`CSSPageDescriptors.marginTop`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPageDescriptors/marginTop)
    pub fn margin_top(&self) -> JsString {
        self.inner.get("marginTop").as_::<JsString>()
    }

    /// Setter of the `marginTop` attribute.
    /// [`CSSPageDescriptors.marginTop`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPageDescriptors/marginTop)
    pub fn set_margin_top(&mut self, value: &JsString) {
        self.inner.set("marginTop", value);
    }
}
impl CSSPageDescriptors {
    /// Getter of the `marginRight` attribute.
    /// [`CSSPageDescriptors.marginRight`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPageDescriptors/marginRight)
    pub fn margin_right(&self) -> JsString {
        self.inner.get("marginRight").as_::<JsString>()
    }

    /// Setter of the `marginRight` attribute.
    /// [`CSSPageDescriptors.marginRight`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPageDescriptors/marginRight)
    pub fn set_margin_right(&mut self, value: &JsString) {
        self.inner.set("marginRight", value);
    }
}
impl CSSPageDescriptors {
    /// Getter of the `marginBottom` attribute.
    /// [`CSSPageDescriptors.marginBottom`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPageDescriptors/marginBottom)
    pub fn margin_bottom(&self) -> JsString {
        self.inner.get("marginBottom").as_::<JsString>()
    }

    /// Setter of the `marginBottom` attribute.
    /// [`CSSPageDescriptors.marginBottom`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPageDescriptors/marginBottom)
    pub fn set_margin_bottom(&mut self, value: &JsString) {
        self.inner.set("marginBottom", value);
    }
}
impl CSSPageDescriptors {
    /// Getter of the `marginLeft` attribute.
    /// [`CSSPageDescriptors.marginLeft`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPageDescriptors/marginLeft)
    pub fn margin_left(&self) -> JsString {
        self.inner.get("marginLeft").as_::<JsString>()
    }

    /// Setter of the `marginLeft` attribute.
    /// [`CSSPageDescriptors.marginLeft`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPageDescriptors/marginLeft)
    pub fn set_margin_left(&mut self, value: &JsString) {
        self.inner.set("marginLeft", value);
    }
}
impl CSSPageDescriptors {
    /// Getter of the `size` attribute.
    /// [`CSSPageDescriptors.size`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPageDescriptors/size)
    pub fn size(&self) -> JsString {
        self.inner.get("size").as_::<JsString>()
    }

    /// Setter of the `size` attribute.
    /// [`CSSPageDescriptors.size`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPageDescriptors/size)
    pub fn set_size(&mut self, value: &JsString) {
        self.inner.set("size", value);
    }
}
impl CSSPageDescriptors {
    /// Getter of the `pageOrientation` attribute.
    /// [`CSSPageDescriptors.pageOrientation`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPageDescriptors/pageOrientation)
    pub fn page_orientation(&self) -> JsString {
        self.inner.get("pageOrientation").as_::<JsString>()
    }

    /// Setter of the `pageOrientation` attribute.
    /// [`CSSPageDescriptors.pageOrientation`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPageDescriptors/pageOrientation)
    pub fn set_page_orientation(&mut self, value: &JsString) {
        self.inner.set("pageOrientation", value);
    }
}
impl CSSPageDescriptors {
    /// Getter of the `marks` attribute.
    /// [`CSSPageDescriptors.marks`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPageDescriptors/marks)
    pub fn marks(&self) -> JsString {
        self.inner.get("marks").as_::<JsString>()
    }

    /// Setter of the `marks` attribute.
    /// [`CSSPageDescriptors.marks`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPageDescriptors/marks)
    pub fn set_marks(&mut self, value: &JsString) {
        self.inner.set("marks", value);
    }
}
impl CSSPageDescriptors {
    /// Getter of the `bleed` attribute.
    /// [`CSSPageDescriptors.bleed`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPageDescriptors/bleed)
    pub fn bleed(&self) -> JsString {
        self.inner.get("bleed").as_::<JsString>()
    }

    /// Setter of the `bleed` attribute.
    /// [`CSSPageDescriptors.bleed`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPageDescriptors/bleed)
    pub fn set_bleed(&mut self, value: &JsString) {
        self.inner.set("bleed", value);
    }
}
