use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
impl From<WorkletAnimation> for emlite::Val {
    fn from(s: WorkletAnimation) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WorkletAnimation {
    pub fn new0(animator_name: jsbind::DOMString) -> WorkletAnimation {
        Self {
            inner: emlite::Val::global("WorkletAnimation")
                .new(&[animator_name.into()])
                .as_::<Animation>(),
        }
    }

    pub fn new1(animator_name: jsbind::DOMString, effects: jsbind::Any) -> WorkletAnimation {
        Self {
            inner: emlite::Val::global("WorkletAnimation")
                .new(&[animator_name.into(), effects.into()])
                .as_::<Animation>(),
        }
    }

    pub fn new2(
        animator_name: jsbind::DOMString,
        effects: jsbind::Any,
        timeline: AnimationTimeline,
    ) -> WorkletAnimation {
        Self {
            inner: emlite::Val::global("WorkletAnimation")
                .new(&[animator_name.into(), effects.into(), timeline.into()])
                .as_::<Animation>(),
        }
    }

    pub fn new3(
        animator_name: jsbind::DOMString,
        effects: jsbind::Any,
        timeline: AnimationTimeline,
        options: jsbind::Any,
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
    pub fn animator_name(&self) -> jsbind::DOMString {
        self.inner.get("animatorName").as_::<jsbind::DOMString>()
    }
}
