use super::*;

#[derive(Clone, Debug)]
pub struct RestrictionTarget {
    inner: emlite::Val,
}
impl FromVal for RestrictionTarget {
    fn from_val(v: &emlite::Val) -> Self {
        RestrictionTarget {
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
impl std::ops::Deref for RestrictionTarget {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RestrictionTarget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RestrictionTarget> for emlite::Val {
    fn from(s: RestrictionTarget) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RestrictionTarget {
    pub fn from_element(element: Element) -> jsbind::Promise {
        emlite::Val::global("restrictiontarget")
            .call("fromElement", &[element.into()])
            .as_::<jsbind::Promise>()
    }
}
