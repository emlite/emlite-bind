use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PAExtendedHistogramContribution {
    inner: Any,
}
impl FromVal for PAExtendedHistogramContribution {
    fn from_val(v: &Any) -> Self {
        PAExtendedHistogramContribution { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PAExtendedHistogramContribution {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PAExtendedHistogramContribution {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PAExtendedHistogramContribution {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PAExtendedHistogramContribution {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PAExtendedHistogramContribution> for Any {
    fn from(s: PAExtendedHistogramContribution) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PAExtendedHistogramContribution> for Any {
    fn from(s: &PAExtendedHistogramContribution) -> Any {
        s.inner.clone()
    }
}

impl PAExtendedHistogramContribution {
    pub fn bucket(&self) -> Any {
        self.inner.get("bucket").as_::<Any>()
    }

    pub fn set_bucket(&mut self, value: &Any) {
        self.inner.set("bucket", value);
    }
}
impl PAExtendedHistogramContribution {
    pub fn value(&self) -> Any {
        self.inner.get("value").as_::<Any>()
    }

    pub fn set_value(&mut self, value: &Any) {
        self.inner.set("value", value);
    }
}
impl PAExtendedHistogramContribution {
    pub fn filtering_id(&self) -> i64 {
        self.inner.get("filteringId").as_::<i64>()
    }

    pub fn set_filtering_id(&mut self, value: i64) {
        self.inner.set("filteringId", value);
    }
}
