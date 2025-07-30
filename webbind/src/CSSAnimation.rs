use super::*;

/// The CSSAnimation class.
/// [`CSSAnimation`](https://developer.mozilla.org/en-US/docs/Web/API/CSSAnimation)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSAnimation {
    inner: Animation,
}
impl FromVal for CSSAnimation {
    fn from_val(v: &Any) -> Self {
        CSSAnimation {
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
impl core::ops::Deref for CSSAnimation {
    type Target = Animation;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CSSAnimation {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CSSAnimation {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CSSAnimation> for Any {
    fn from(s: CSSAnimation) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CSSAnimation> for Any {
    fn from(s: &CSSAnimation) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSAnimation);

impl CSSAnimation {
    /// Getter of the `animationName` attribute.
    /// [`CSSAnimation.animationName`](https://developer.mozilla.org/en-US/docs/Web/API/CSSAnimation/animationName)
    pub fn animation_name(&self) -> JsString {
        self.inner.get("animationName").as_::<JsString>()
    }
}
