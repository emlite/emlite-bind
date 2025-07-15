use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for ChildBreakToken {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ChildBreakToken {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ChildBreakToken {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ChildBreakToken {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ChildBreakToken> for emlite::Val {
    fn from(s: ChildBreakToken) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&ChildBreakToken> for emlite::Val {
    fn from(s: &ChildBreakToken) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ChildBreakToken);

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
