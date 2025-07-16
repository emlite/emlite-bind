use super::*;

/// The ByteLengthQueuingStrategy class.
/// [`ByteLengthQueuingStrategy`](https://developer.mozilla.org/en-US/docs/Web/API/ByteLengthQueuingStrategy)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ByteLengthQueuingStrategy {
    inner: Any,
}
impl FromVal for ByteLengthQueuingStrategy {
    fn from_val(v: &Any) -> Self {
        ByteLengthQueuingStrategy {
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
impl core::ops::Deref for ByteLengthQueuingStrategy {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ByteLengthQueuingStrategy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ByteLengthQueuingStrategy {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ByteLengthQueuingStrategy {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ByteLengthQueuingStrategy> for Any {
    fn from(s: ByteLengthQueuingStrategy) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ByteLengthQueuingStrategy> for Any {
    fn from(s: &ByteLengthQueuingStrategy) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ByteLengthQueuingStrategy);

impl ByteLengthQueuingStrategy {
    /// The `new ByteLengthQueuingStrategy(..)` constructor, creating a new ByteLengthQueuingStrategy instance
    pub fn new(init: &Any) -> ByteLengthQueuingStrategy {
        Self {
            inner: Any::global("ByteLengthQueuingStrategy")
                .new(&[init.into()])
                .as_::<Any>(),
        }
    }
}
impl ByteLengthQueuingStrategy {
    /// Getter of the `highWaterMark` attribute.
    /// [`ByteLengthQueuingStrategy.highWaterMark`](https://developer.mozilla.org/en-US/docs/Web/API/ByteLengthQueuingStrategy/highWaterMark)
    pub fn high_water_mark(&self) -> f64 {
        self.inner.get("highWaterMark").as_::<f64>()
    }
}
impl ByteLengthQueuingStrategy {
    /// Getter of the `size` attribute.
    /// [`ByteLengthQueuingStrategy.size`](https://developer.mozilla.org/en-US/docs/Web/API/ByteLengthQueuingStrategy/size)
    pub fn size(&self) -> Function {
        self.inner.get("size").as_::<Function>()
    }
}
