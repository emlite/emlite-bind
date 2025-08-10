use super::*;

/// The DOMStringList class.
/// [`DOMStringList`](https://developer.mozilla.org/en-US/docs/Web/API/DOMStringList)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DOMStringList {
    inner: Any,
}

impl FromVal for DOMStringList {
    fn from_val(v: &Any) -> Self {
        DOMStringList {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DOMStringList {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DOMStringList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DOMStringList {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DOMStringList {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<DOMStringList> for Any {
    fn from(s: DOMStringList) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DOMStringList> for Any {
    fn from(s: &DOMStringList) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(DOMStringList);

impl DOMStringList {
    /// Getter of the `length` attribute.
    /// [`DOMStringList.length`](https://developer.mozilla.org/en-US/docs/Web/API/DOMStringList/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl DOMStringList {
    /// The item method.
    /// [`DOMStringList.item`](https://developer.mozilla.org/en-US/docs/Web/API/DOMStringList/item)
    pub fn item(&self, index: u32) -> JsString {
        self.inner.call("item", &[index.into()]).as_::<JsString>()
    }
}
impl DOMStringList {
    /// The contains method.
    /// [`DOMStringList.contains`](https://developer.mozilla.org/en-US/docs/Web/API/DOMStringList/contains)
    pub fn contains(&self, string: &JsString) -> bool {
        self.inner.call("contains", &[string.into()]).as_::<bool>()
    }
}
