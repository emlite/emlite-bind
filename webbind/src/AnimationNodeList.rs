use super::*;




/// The AnimationNodeList class.
/// [`AnimationNodeList`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationNodeList)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AnimationNodeList {
    inner: Any,
}

impl FromVal for AnimationNodeList {
    fn from_val(v: &Any) -> Self {
        AnimationNodeList { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AnimationNodeList {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AnimationNodeList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AnimationNodeList {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AnimationNodeList {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<AnimationNodeList> for Any {
    fn from(s: AnimationNodeList) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AnimationNodeList> for Any {
    fn from(s: &AnimationNodeList) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(AnimationNodeList);


impl AnimationNodeList {
    /// Getter of the `length` attribute.
    /// [`AnimationNodeList.length`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationNodeList/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

}
impl AnimationNodeList {
    /// The item method.
    /// [`AnimationNodeList.item`](https://developer.mozilla.org/en-US/docs/Web/API/AnimationNodeList/item)
    pub fn item(&self, index: u32) -> AnimationEffect {
        self.inner.call("item", &[index.into(), ]).as_::<AnimationEffect>()
    }
}
