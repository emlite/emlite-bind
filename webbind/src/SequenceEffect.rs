use super::*;

/// The SequenceEffect class.
/// [`SequenceEffect`](https://developer.mozilla.org/en-US/docs/Web/API/SequenceEffect)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SequenceEffect {
    inner: GroupEffect,
}
impl FromVal for SequenceEffect {
    fn from_val(v: &Any) -> Self {
        SequenceEffect {
            inner: GroupEffect::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for SequenceEffect {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SequenceEffect {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SequenceEffect> for Any {
    fn from(s: SequenceEffect) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SequenceEffect> for Any {
    fn from(s: &SequenceEffect) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SequenceEffect);

impl SequenceEffect {
    /// The `new SequenceEffect(..)` constructor, creating a new SequenceEffect instance
    pub fn new0(children: &TypedArray<AnimationEffect>) -> SequenceEffect {
        Self {
            inner: Any::global("SequenceEffect")
                .new(&[children.into()])
                .as_::<GroupEffect>(),
        }
    }

    /// The `new SequenceEffect(..)` constructor, creating a new SequenceEffect instance
    pub fn new1(children: &TypedArray<AnimationEffect>, timing: &Any) -> SequenceEffect {
        Self {
            inner: Any::global("SequenceEffect")
                .new(&[children.into(), timing.into()])
                .as_::<GroupEffect>(),
        }
    }
}
impl SequenceEffect {
    /// The clone method.
    /// [`SequenceEffect.clone`](https://developer.mozilla.org/en-US/docs/Web/API/SequenceEffect/clone)
    pub fn clone_(&self) -> SequenceEffect {
        self.inner.call("clone", &[]).as_::<SequenceEffect>()
    }
}
