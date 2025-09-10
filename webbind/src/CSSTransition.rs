use super::*;

/// The CSSTransition class.
/// [`CSSTransition`](https://developer.mozilla.org/en-US/docs/Web/API/CSSTransition)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSTransition {
    inner: Animation,
}

impl FromVal for CSSTransition {
    fn from_val(v: &Any) -> Self {
        CSSTransition {
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

impl core::ops::Deref for CSSTransition {
    type Target = Animation;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSTransition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSTransition {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSTransition {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CSSTransition> for Any {
    fn from(s: CSSTransition) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSTransition> for Any {
    fn from(s: &CSSTransition) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSTransition);

impl CSSTransition {
    /// Getter of the `transitionProperty` attribute.
    /// [`CSSTransition.transitionProperty`](https://developer.mozilla.org/en-US/docs/Web/API/CSSTransition/transitionProperty)
    pub fn transition_property(&self) -> JsString {
        self.inner.get("transitionProperty").as_::<JsString>()
    }
}
