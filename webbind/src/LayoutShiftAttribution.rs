use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LayoutShiftAttribution {
    inner: emlite::Val,
}
impl FromVal for LayoutShiftAttribution {
    fn from_val(v: &emlite::Val) -> Self {
        LayoutShiftAttribution {
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
impl core::ops::Deref for LayoutShiftAttribution {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for LayoutShiftAttribution {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<LayoutShiftAttribution> for emlite::Val {
    fn from(s: LayoutShiftAttribution) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl LayoutShiftAttribution {
    pub fn node(&self) -> Node {
        self.inner.get("node").as_::<Node>()
    }
}
impl LayoutShiftAttribution {
    pub fn previous_rect(&self) -> DOMRectReadOnly {
        self.inner.get("previousRect").as_::<DOMRectReadOnly>()
    }
}
impl LayoutShiftAttribution {
    pub fn current_rect(&self) -> DOMRectReadOnly {
        self.inner.get("currentRect").as_::<DOMRectReadOnly>()
    }
}
