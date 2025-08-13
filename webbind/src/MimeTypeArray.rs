use super::*;




/// The MimeTypeArray class.
/// [`MimeTypeArray`](https://developer.mozilla.org/en-US/docs/Web/API/MimeTypeArray)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MimeTypeArray {
    inner: Any,
}

impl FromVal for MimeTypeArray {
    fn from_val(v: &Any) -> Self {
        MimeTypeArray { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MimeTypeArray {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MimeTypeArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MimeTypeArray {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MimeTypeArray {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MimeTypeArray> for Any {
    fn from(s: MimeTypeArray) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MimeTypeArray> for Any {
    fn from(s: &MimeTypeArray) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(MimeTypeArray);


impl MimeTypeArray {
    /// Getter of the `length` attribute.
    /// [`MimeTypeArray.length`](https://developer.mozilla.org/en-US/docs/Web/API/MimeTypeArray/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

}
impl MimeTypeArray {
    /// The item method.
    /// [`MimeTypeArray.item`](https://developer.mozilla.org/en-US/docs/Web/API/MimeTypeArray/item)
    pub fn item(&self, index: u32) -> MimeType {
        self.inner.call("item", &[index.into(), ]).as_::<MimeType>()
    }
}
impl MimeTypeArray {
    /// The namedItem method.
    /// [`MimeTypeArray.namedItem`](https://developer.mozilla.org/en-US/docs/Web/API/MimeTypeArray/namedItem)
    pub fn named_item(&self, name: &JsString) -> MimeType {
        self.inner.call("namedItem", &[name.into(), ]).as_::<MimeType>()
    }
}
