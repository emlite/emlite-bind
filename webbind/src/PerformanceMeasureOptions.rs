use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceMeasureOptions {
    inner: Any,
}
impl FromVal for PerformanceMeasureOptions {
    fn from_val(v: &Any) -> Self {
        PerformanceMeasureOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PerformanceMeasureOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PerformanceMeasureOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PerformanceMeasureOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PerformanceMeasureOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PerformanceMeasureOptions> for Any {
    fn from(s: PerformanceMeasureOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PerformanceMeasureOptions> for Any {
    fn from(s: &PerformanceMeasureOptions) -> Any {
        s.inner.clone()
    }
}

impl PerformanceMeasureOptions {
    pub fn detail(&self) -> Any {
        self.inner.get("detail").as_::<Any>()
    }

    pub fn set_detail(&mut self, value: &Any) {
        self.inner.set("detail", value);
    }
}
impl PerformanceMeasureOptions {
    pub fn start(&self) -> Any {
        self.inner.get("start").as_::<Any>()
    }

    pub fn set_start(&mut self, value: &Any) {
        self.inner.set("start", value);
    }
}
impl PerformanceMeasureOptions {
    pub fn duration(&self) -> Any {
        self.inner.get("duration").as_::<Any>()
    }

    pub fn set_duration(&mut self, value: &Any) {
        self.inner.set("duration", value);
    }
}
impl PerformanceMeasureOptions {
    pub fn end(&self) -> Any {
        self.inner.get("end").as_::<Any>()
    }

    pub fn set_end(&mut self, value: &Any) {
        self.inner.set("end", value);
    }
}
