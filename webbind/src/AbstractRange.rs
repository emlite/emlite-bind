use super::*;




/// The AbstractRange class.
/// [`AbstractRange`](https://developer.mozilla.org/en-US/docs/Web/API/AbstractRange)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AbstractRange {
    inner: Any,
}

impl FromVal for AbstractRange {
    fn from_val(v: &Any) -> Self {
        AbstractRange { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AbstractRange {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AbstractRange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AbstractRange {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AbstractRange {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<AbstractRange> for Any {
    fn from(s: AbstractRange) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AbstractRange> for Any {
    fn from(s: &AbstractRange) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(AbstractRange);


impl AbstractRange {
    /// Getter of the `startContainer` attribute.
    /// [`AbstractRange.startContainer`](https://developer.mozilla.org/en-US/docs/Web/API/AbstractRange/startContainer)
    pub fn start_container(&self) -> Node {
        self.inner.get("startContainer").as_::<Node>()
    }

}
impl AbstractRange {
    /// Getter of the `startOffset` attribute.
    /// [`AbstractRange.startOffset`](https://developer.mozilla.org/en-US/docs/Web/API/AbstractRange/startOffset)
    pub fn start_offset(&self) -> u32 {
        self.inner.get("startOffset").as_::<u32>()
    }

}
impl AbstractRange {
    /// Getter of the `endContainer` attribute.
    /// [`AbstractRange.endContainer`](https://developer.mozilla.org/en-US/docs/Web/API/AbstractRange/endContainer)
    pub fn end_container(&self) -> Node {
        self.inner.get("endContainer").as_::<Node>()
    }

}
impl AbstractRange {
    /// Getter of the `endOffset` attribute.
    /// [`AbstractRange.endOffset`](https://developer.mozilla.org/en-US/docs/Web/API/AbstractRange/endOffset)
    pub fn end_offset(&self) -> u32 {
        self.inner.get("endOffset").as_::<u32>()
    }

}
impl AbstractRange {
    /// Getter of the `collapsed` attribute.
    /// [`AbstractRange.collapsed`](https://developer.mozilla.org/en-US/docs/Web/API/AbstractRange/collapsed)
    pub fn collapsed(&self) -> bool {
        self.inner.get("collapsed").as_::<bool>()
    }

}
