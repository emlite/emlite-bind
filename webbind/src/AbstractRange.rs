use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for AbstractRange {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AbstractRange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AbstractRange> for emlite::Val {
    fn from(s: AbstractRange) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

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
