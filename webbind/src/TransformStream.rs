use super::*;




/// The TransformStream class.
/// [`TransformStream`](https://developer.mozilla.org/en-US/docs/Web/API/TransformStream)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TransformStream {
    inner: Any,
}

impl FromVal for TransformStream {
    fn from_val(v: &Any) -> Self {
        TransformStream { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for TransformStream {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for TransformStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for TransformStream {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for TransformStream {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<TransformStream> for Any {
    fn from(s: TransformStream) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&TransformStream> for Any {
    fn from(s: &TransformStream) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(TransformStream);



impl TransformStream {
    /// The `new TransformStream(..)` constructor, creating a new TransformStream instance
    pub fn new0() -> TransformStream {
        Self {
            inner: Any::global("TransformStream").new(&[]).as_::<Any>(),
        }
    }

    /// The `new TransformStream(..)` constructor, creating a new TransformStream instance
    pub fn new1(transformer: &Object) -> TransformStream {
        Self {
            inner: Any::global("TransformStream").new(&[transformer.into()]).as_::<Any>(),
        }
    }

    /// The `new TransformStream(..)` constructor, creating a new TransformStream instance
    pub fn new2(transformer: &Object, writable_strategy: &QueuingStrategy) -> TransformStream {
        Self {
            inner: Any::global("TransformStream").new(&[transformer.into(), writable_strategy.into()]).as_::<Any>(),
        }
    }

    /// The `new TransformStream(..)` constructor, creating a new TransformStream instance
    pub fn new3(transformer: &Object, writable_strategy: &QueuingStrategy, readable_strategy: &QueuingStrategy) -> TransformStream {
        Self {
            inner: Any::global("TransformStream").new(&[transformer.into(), writable_strategy.into(), readable_strategy.into()]).as_::<Any>(),
        }
    }

}
impl TransformStream {
    /// Getter of the `readable` attribute.
    /// [`TransformStream.readable`](https://developer.mozilla.org/en-US/docs/Web/API/TransformStream/readable)
    pub fn readable(&self) -> ReadableStream {
        self.inner.get("readable").as_::<ReadableStream>()
    }

}
impl TransformStream {
    /// Getter of the `writable` attribute.
    /// [`TransformStream.writable`](https://developer.mozilla.org/en-US/docs/Web/API/TransformStream/writable)
    pub fn writable(&self) -> WritableStream {
        self.inner.get("writable").as_::<WritableStream>()
    }

}
