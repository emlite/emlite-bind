use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WorkletGroupEffect {
    inner: emlite::Val,
}
impl FromVal for WorkletGroupEffect {
    fn from_val(v: &emlite::Val) -> Self {
        WorkletGroupEffect { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WorkletGroupEffect {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WorkletGroupEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WorkletGroupEffect {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WorkletGroupEffect {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<WorkletGroupEffect> for emlite::Val {
    fn from(s: WorkletGroupEffect) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(WorkletGroupEffect);


impl WorkletGroupEffect {
    pub fn get_children(&self, ) -> Sequence<WorkletAnimationEffect> {
        self.inner.call("getChildren", &[]).as_::<Sequence<WorkletAnimationEffect>>()
    }

}
