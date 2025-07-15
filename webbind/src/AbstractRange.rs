use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AbstractRange {
    inner: emlite::Val,
}
impl FromVal for AbstractRange {
    fn from_val(v: &emlite::Val) -> Self {
        AbstractRange {
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
impl core::ops::Deref for AbstractRange {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AbstractRange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for AbstractRange {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for AbstractRange {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<AbstractRange> for emlite::Val {
    fn from(s: AbstractRange) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&AbstractRange> for emlite::Val {
    fn from(s: &AbstractRange) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(AbstractRange);

impl AbstractRange {
    pub fn start_container(&self) -> Node {
        self.inner.get("startContainer").as_::<Node>()
    }
}
impl AbstractRange {
    pub fn start_offset(&self) -> u32 {
        self.inner.get("startOffset").as_::<u32>()
    }
}
impl AbstractRange {
    pub fn end_container(&self) -> Node {
        self.inner.get("endContainer").as_::<Node>()
    }
}
impl AbstractRange {
    pub fn end_offset(&self) -> u32 {
        self.inner.get("endOffset").as_::<u32>()
    }
}
impl AbstractRange {
    pub fn collapsed(&self) -> bool {
        self.inner.get("collapsed").as_::<bool>()
    }
}
