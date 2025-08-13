use super::*;




/// The PAHistogramContribution dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PAHistogramContribution {
    inner: Any,
}

impl FromVal for PAHistogramContribution {
    fn from_val(v: &Any) -> Self {
        PAHistogramContribution { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PAHistogramContribution {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PAHistogramContribution {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PAHistogramContribution {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PAHistogramContribution {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PAHistogramContribution> for Any {
    fn from(s: PAHistogramContribution) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PAHistogramContribution> for Any {
    fn from(s: &PAHistogramContribution) -> Any {
        s.inner.clone()
    }
}

impl PAHistogramContribution {
    /// Getter of the `bucket` attribute.
    pub fn bucket(&self) -> i64 {
        self.inner.get("bucket").as_::<i64>()
    }

    /// Setter of the `bucket` attribute.
    pub fn set_bucket(&mut self, value: i64) {
        self.inner.set("bucket", value);
    }
}
impl PAHistogramContribution {
    /// Getter of the `value` attribute.
    pub fn value(&self) -> i32 {
        self.inner.get("value").as_::<i32>()
    }

    /// Setter of the `value` attribute.
    pub fn set_value(&mut self, value: i32) {
        self.inner.set("value", value);
    }
}
impl PAHistogramContribution {
    /// Getter of the `filteringId` attribute.
    pub fn filtering_id(&self) -> i64 {
        self.inner.get("filteringId").as_::<i64>()
    }

    /// Setter of the `filteringId` attribute.
    pub fn set_filtering_id(&mut self, value: i64) {
        self.inner.set("filteringId", value);
    }
}
