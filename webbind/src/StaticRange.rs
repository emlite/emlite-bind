use super::*;

#[derive(Clone, Debug)]
pub struct StaticRange {
    inner: AbstractRange,
}
impl FromVal for StaticRange {
    fn from_val(v: &emlite::Val) -> Self {
        StaticRange {
            inner: AbstractRange::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for StaticRange {
    type Target = AbstractRange;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for StaticRange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<StaticRange> for emlite::Val {
    fn from(s: StaticRange) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl StaticRange {
    pub fn new(init: jsbind::Any) -> StaticRange {
        Self {
            inner: emlite::Val::global("StaticRange")
                .new(&[init.into()])
                .as_::<AbstractRange>(),
        }
    }
}
