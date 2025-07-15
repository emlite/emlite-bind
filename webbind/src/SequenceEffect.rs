use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SequenceEffect {
    inner: GroupEffect,
}
impl FromVal for SequenceEffect {
    fn from_val(v: &emlite::Val) -> Self {
        SequenceEffect {
            inner: GroupEffect::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SequenceEffect {
    type Target = GroupEffect;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SequenceEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SequenceEffect {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SequenceEffect {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SequenceEffect> for emlite::Val {
    fn from(s: SequenceEffect) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&SequenceEffect> for emlite::Val {
    fn from(s: &SequenceEffect) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SequenceEffect);

impl SequenceEffect {
    pub fn new0(children: Sequence<AnimationEffect>) -> SequenceEffect {
        Self {
            inner: emlite::Val::global("SequenceEffect")
                .new(&[children.into()])
                .as_::<GroupEffect>(),
        }
    }

    pub fn new1(children: Sequence<AnimationEffect>, timing: Any) -> SequenceEffect {
        Self {
            inner: emlite::Val::global("SequenceEffect")
                .new(&[children.into(), timing.into()])
                .as_::<GroupEffect>(),
        }
    }
}
impl SequenceEffect {
    pub fn clone_(&self) -> SequenceEffect {
        self.inner.call("clone", &[]).as_::<SequenceEffect>()
    }
}
