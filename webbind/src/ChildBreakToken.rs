use super::*;

#[derive(Clone, Debug)]
pub struct ChildBreakToken {
    inner: emlite::Val,
}
impl FromVal for ChildBreakToken {
    fn from_val(v: &emlite::Val) -> Self {
        ChildBreakToken {
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
impl std::ops::Deref for ChildBreakToken {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ChildBreakToken {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ChildBreakToken> for emlite::Val {
    fn from(s: ChildBreakToken) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ChildBreakToken {
    pub fn break_type(&self) -> BreakType {
        self.inner.get("breakType").as_::<BreakType>()
    }
}
impl ChildBreakToken {
    pub fn child(&self) -> LayoutChild {
        self.inner.get("child").as_::<LayoutChild>()
    }
}
