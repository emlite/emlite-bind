use super::*;

/// The DOMRectList class.
/// [`DOMRectList`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectList)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DOMRectList {
    inner: Any,
}
impl FromVal for DOMRectList {
    fn from_val(v: &Any) -> Self {
        DOMRectList {
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
impl core::ops::Deref for DOMRectList {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DOMRectList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for DOMRectList {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for DOMRectList {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<DOMRectList> for Any {
    fn from(s: DOMRectList) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&DOMRectList> for Any {
    fn from(s: &DOMRectList) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(DOMRectList);

impl DOMRectList {
    /// Getter of the `length` attribute.
    /// [`DOMRectList.length`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectList/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl DOMRectList {
    /// The item method.
    /// [`DOMRectList.item`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectList/item)
    pub fn item(&self, index: u32) -> DOMRect {
        self.inner.call("item", &[index.into()]).as_::<DOMRect>()
    }
}
