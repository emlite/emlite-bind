use super::*;

#[derive(Clone, Debug)]
pub struct BreakToken {
    inner: emlite::Val,
}
impl FromVal for BreakToken {
    fn from_val(v: &emlite::Val) -> Self {
        BreakToken {
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
impl std::ops::Deref for BreakToken {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for BreakToken {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<BreakToken> for emlite::Val {
    fn from(s: BreakToken) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl BreakToken {
    pub fn child_break_tokens(&self) -> jsbind::FrozenArray<ChildBreakToken> {
        self.inner
            .get("childBreakTokens")
            .as_::<jsbind::FrozenArray<ChildBreakToken>>()
    }
}
impl BreakToken {
    pub fn data(&self) -> jsbind::Any {
        self.inner.get("data").as_::<jsbind::Any>()
    }
}
