use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceMark {
    inner: PerformanceEntry,
}
impl FromVal for PerformanceMark {
    fn from_val(v: &emlite::Val) -> Self {
        PerformanceMark {
            inner: PerformanceEntry::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PerformanceMark {
    type Target = PerformanceEntry;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PerformanceMark {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PerformanceMark {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PerformanceMark {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PerformanceMark> for emlite::Val {
    fn from(s: PerformanceMark) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(PerformanceMark);

impl PerformanceMark {
    pub fn new0(mark_name: jsbind::DOMString) -> PerformanceMark {
        Self {
            inner: emlite::Val::global("PerformanceMark")
                .new(&[mark_name.into()])
                .as_::<PerformanceEntry>(),
        }
    }

    pub fn new1(
        mark_name: jsbind::DOMString,
        mark_options: PerformanceMarkOptions,
    ) -> PerformanceMark {
        Self {
            inner: emlite::Val::global("PerformanceMark")
                .new(&[mark_name.into(), mark_options.into()])
                .as_::<PerformanceEntry>(),
        }
    }
}
impl PerformanceMark {
    pub fn detail(&self) -> jsbind::Any {
        self.inner.get("detail").as_::<jsbind::Any>()
    }
}
