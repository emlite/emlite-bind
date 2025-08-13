use super::*;




/// The StaticRange class.
/// [`StaticRange`](https://developer.mozilla.org/en-US/docs/Web/API/StaticRange)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StaticRange {
    inner: AbstractRange,
}

impl FromVal for StaticRange {
    fn from_val(v: &Any) -> Self {
        StaticRange { inner: AbstractRange::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for StaticRange {
    type Target = AbstractRange;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for StaticRange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for StaticRange {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for StaticRange {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<StaticRange> for Any {
    fn from(s: StaticRange) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&StaticRange> for Any {
    fn from(s: &StaticRange) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(StaticRange);



impl StaticRange {
    /// The `new StaticRange(..)` constructor, creating a new StaticRange instance
    pub fn new(init: &StaticRangeInit) -> StaticRange {
        Self {
            inner: Any::global("StaticRange").new(&[init.into()]).as_::<AbstractRange>(),
        }
    }

}
