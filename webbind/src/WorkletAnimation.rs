use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WorkletAnimation {
    inner: Animation,
}
impl FromVal for WorkletAnimation {
    fn from_val(v: &emlite::Val) -> Self {
        WorkletAnimation {
            inner: Animation::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WorkletAnimation {
    type Target = Animation;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WorkletAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WorkletAnimation {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WorkletAnimation {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<WorkletAnimation> for emlite::Val {
    fn from(s: WorkletAnimation) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&WorkletAnimation> for emlite::Val {
    fn from(s: &WorkletAnimation) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(WorkletAnimation);

impl WorkletAnimation {
    pub fn new0(animator_name: &str) -> WorkletAnimation {
        Self {
            inner: emlite::Val::global("WorkletAnimation")
                .new(&[animator_name.into()])
                .as_::<Animation>(),
        }
    }

    pub fn new1(animator_name: &str, effects: &Any) -> WorkletAnimation {
        Self {
            inner: emlite::Val::global("WorkletAnimation")
                .new(&[animator_name.into(), effects.into()])
                .as_::<Animation>(),
        }
    }

    pub fn new2(
        animator_name: &str,
        effects: &Any,
        timeline: &AnimationTimeline,
    ) -> WorkletAnimation {
        Self {
            inner: emlite::Val::global("WorkletAnimation")
                .new(&[animator_name.into(), effects.into(), timeline.into()])
                .as_::<Animation>(),
        }
    }

    pub fn new3(
        animator_name: &str,
        effects: &Any,
        timeline: &AnimationTimeline,
        options: &Any,
    ) -> WorkletAnimation {
        Self {
            inner: emlite::Val::global("WorkletAnimation")
                .new(&[
                    animator_name.into(),
                    effects.into(),
                    timeline.into(),
                    options.into(),
                ])
                .as_::<Animation>(),
        }
    }
}
impl WorkletAnimation {
    pub fn animator_name(&self) -> String {
        self.inner.get("animatorName").as_::<String>()
    }
}
