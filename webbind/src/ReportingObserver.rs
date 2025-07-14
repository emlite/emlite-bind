use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ReportingObserver {
    inner: emlite::Val,
}
impl FromVal for ReportingObserver {
    fn from_val(v: &emlite::Val) -> Self {
        ReportingObserver {
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
impl core::ops::Deref for ReportingObserver {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ReportingObserver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ReportingObserver {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ReportingObserver {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ReportingObserver> for emlite::Val {
    fn from(s: ReportingObserver) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(ReportingObserver);

impl ReportingObserver {
    pub fn new0(callback: jsbind::Function) -> ReportingObserver {
        Self {
            inner: emlite::Val::global("ReportingObserver")
                .new(&[callback.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(callback: jsbind::Function, options: jsbind::Any) -> ReportingObserver {
        Self {
            inner: emlite::Val::global("ReportingObserver")
                .new(&[callback.into(), options.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl ReportingObserver {
    pub fn observe(&self) -> jsbind::Undefined {
        self.inner.call("observe", &[]).as_::<jsbind::Undefined>()
    }
}
impl ReportingObserver {
    pub fn disconnect(&self) -> jsbind::Undefined {
        self.inner
            .call("disconnect", &[])
            .as_::<jsbind::Undefined>()
    }
}
impl ReportingObserver {
    pub fn take_records(&self) -> jsbind::Any {
        self.inner.call("takeRecords", &[]).as_::<jsbind::Any>()
    }
}
