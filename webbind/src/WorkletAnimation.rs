use super::*;

/// The WorkletAnimation class.
/// [`WorkletAnimation`](https://developer.mozilla.org/en-US/docs/Web/API/WorkletAnimation)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WorkletAnimation {
    inner: Animation,
}
impl FromVal for WorkletAnimation {
    fn from_val(v: &Any) -> Self {
        WorkletAnimation {
            inner: Animation::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for WorkletAnimation {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for WorkletAnimation {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<WorkletAnimation> for Any {
    fn from(s: WorkletAnimation) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&WorkletAnimation> for Any {
    fn from(s: &WorkletAnimation) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(WorkletAnimation);

impl WorkletAnimation {
    /// The `new WorkletAnimation(..)` constructor, creating a new WorkletAnimation instance
    pub fn new0(animator_name: &str) -> WorkletAnimation {
        Self {
            inner: Any::global("WorkletAnimation")
                .new(&[animator_name.into()])
                .as_::<Animation>(),
        }
    }

    /// The `new WorkletAnimation(..)` constructor, creating a new WorkletAnimation instance
    pub fn new1(animator_name: &str, effects: &Any) -> WorkletAnimation {
        Self {
            inner: Any::global("WorkletAnimation")
                .new(&[animator_name.into(), effects.into()])
                .as_::<Animation>(),
        }
    }

    /// The `new WorkletAnimation(..)` constructor, creating a new WorkletAnimation instance
    pub fn new2(
        animator_name: &str,
        effects: &Any,
        timeline: &AnimationTimeline,
    ) -> WorkletAnimation {
        Self {
            inner: Any::global("WorkletAnimation")
                .new(&[animator_name.into(), effects.into(), timeline.into()])
                .as_::<Animation>(),
        }
    }

    /// The `new WorkletAnimation(..)` constructor, creating a new WorkletAnimation instance
    pub fn new3(
        animator_name: &str,
        effects: &Any,
        timeline: &AnimationTimeline,
        options: &Any,
    ) -> WorkletAnimation {
        Self {
            inner: Any::global("WorkletAnimation")
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
    /// Getter of the `animatorName` attribute.
    /// [`WorkletAnimation.animatorName`](https://developer.mozilla.org/en-US/docs/Web/API/WorkletAnimation/animatorName)
    pub fn animator_name(&self) -> String {
        self.inner.get("animatorName").as_::<String>()
    }
}
