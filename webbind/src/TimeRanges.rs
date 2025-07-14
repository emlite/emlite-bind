use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TimeRanges {
    inner: emlite::Val,
}
impl FromVal for TimeRanges {
    fn from_val(v: &emlite::Val) -> Self {
        TimeRanges {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for TimeRanges {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TimeRanges {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<TimeRanges> for emlite::Val {
    fn from(s: TimeRanges) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl TimeRanges {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl TimeRanges {
    pub fn start(&self, index: u32) -> f64 {
        self.inner.call("start", &[index.into()]).as_::<f64>()
    }
}
impl TimeRanges {
    pub fn end(&self, index: u32) -> f64 {
        self.inner.call("end", &[index.into()]).as_::<f64>()
    }
}
