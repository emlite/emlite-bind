use super::*;

/// The ShadowAnimation class.
/// [`ShadowAnimation`](https://developer.mozilla.org/en-US/docs/Web/API/ShadowAnimation)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ShadowAnimation {
    inner: Animation,
}
impl FromVal for ShadowAnimation {
    fn from_val(v: &Any) -> Self {
        ShadowAnimation {
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
impl core::ops::Deref for ShadowAnimation {
    type Target = Animation;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ShadowAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ShadowAnimation {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ShadowAnimation {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ShadowAnimation> for Any {
    fn from(s: ShadowAnimation) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ShadowAnimation> for Any {
    fn from(s: &ShadowAnimation) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ShadowAnimation);

impl ShadowAnimation {
    /// The `new ShadowAnimation(..)` constructor, creating a new ShadowAnimation instance
    pub fn new(source: &Animation, new_target: &Any) -> ShadowAnimation {
        Self {
            inner: Any::global("ShadowAnimation")
                .new(&[source.into(), new_target.into()])
                .as_::<Animation>(),
        }
    }
}
impl ShadowAnimation {
    /// Getter of the `sourceAnimation` attribute.
    /// [`ShadowAnimation.sourceAnimation`](https://developer.mozilla.org/en-US/docs/Web/API/ShadowAnimation/sourceAnimation)
    pub fn source_animation(&self) -> Animation {
        self.inner.get("sourceAnimation").as_::<Animation>()
    }
}
