use super::*;

/// The TouchList class.
/// [`TouchList`](https://developer.mozilla.org/en-US/docs/Web/API/TouchList)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TouchList {
    inner: Any,
}
impl FromVal for TouchList {
    fn from_val(v: &Any) -> Self {
        TouchList {
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
impl core::ops::Deref for TouchList {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TouchList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for TouchList {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for TouchList {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<TouchList> for Any {
    fn from(s: TouchList) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&TouchList> for Any {
    fn from(s: &TouchList) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(TouchList);

impl TouchList {
    /// Getter of the `length` attribute.
    /// [`TouchList.length`](https://developer.mozilla.org/en-US/docs/Web/API/TouchList/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl TouchList {
    /// The item method.
    /// [`TouchList.item`](https://developer.mozilla.org/en-US/docs/Web/API/TouchList/item)
    pub fn item(&self, index: u32) -> Touch {
        self.inner.call("item", &[index.into()]).as_::<Touch>()
    }
}
