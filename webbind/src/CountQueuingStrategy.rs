use super::*;

/// The CountQueuingStrategy class.
/// [`CountQueuingStrategy`](https://developer.mozilla.org/en-US/docs/Web/API/CountQueuingStrategy)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CountQueuingStrategy {
    inner: Any,
}

impl FromVal for CountQueuingStrategy {
    fn from_val(v: &Any) -> Self {
        CountQueuingStrategy {
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

impl core::ops::Deref for CountQueuingStrategy {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CountQueuingStrategy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CountQueuingStrategy {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CountQueuingStrategy {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CountQueuingStrategy> for Any {
    fn from(s: CountQueuingStrategy) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CountQueuingStrategy> for Any {
    fn from(s: &CountQueuingStrategy) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CountQueuingStrategy);

impl CountQueuingStrategy {
    /// Getter of the `highWaterMark` attribute.
    /// [`CountQueuingStrategy.highWaterMark`](https://developer.mozilla.org/en-US/docs/Web/API/CountQueuingStrategy/highWaterMark)
    pub fn high_water_mark(&self) -> f64 {
        self.inner.get("highWaterMark").as_::<f64>()
    }
}
impl CountQueuingStrategy {
    /// Getter of the `size` attribute.
    /// [`CountQueuingStrategy.size`](https://developer.mozilla.org/en-US/docs/Web/API/CountQueuingStrategy/size)
    pub fn size(&self) -> Function {
        self.inner.get("size").as_::<Function>()
    }
}

impl CountQueuingStrategy {
    /// The `new CountQueuingStrategy(..)` constructor, creating a new CountQueuingStrategy instance
    pub fn new(init: &QueuingStrategyInit) -> CountQueuingStrategy {
        Self {
            inner: Any::global("CountQueuingStrategy")
                .new(&[init.into()])
                .as_::<Any>(),
        }
    }
}
