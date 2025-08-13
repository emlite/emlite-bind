use super::*;




/// The StyleSheetList class.
/// [`StyleSheetList`](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheetList)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StyleSheetList {
    inner: Any,
}

impl FromVal for StyleSheetList {
    fn from_val(v: &Any) -> Self {
        StyleSheetList { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for StyleSheetList {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for StyleSheetList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for StyleSheetList {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for StyleSheetList {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<StyleSheetList> for Any {
    fn from(s: StyleSheetList) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&StyleSheetList> for Any {
    fn from(s: &StyleSheetList) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(StyleSheetList);


impl StyleSheetList {
    /// The item method.
    /// [`StyleSheetList.item`](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheetList/item)
    pub fn item(&self, index: u32) -> CSSStyleSheet {
        self.inner.call("item", &[index.into(), ]).as_::<CSSStyleSheet>()
    }
}
impl StyleSheetList {
    /// Getter of the `length` attribute.
    /// [`StyleSheetList.length`](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheetList/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

}
