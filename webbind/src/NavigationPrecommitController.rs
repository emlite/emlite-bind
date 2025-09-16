use super::*;

/// The NavigationPrecommitController class.
/// [`NavigationPrecommitController`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationPrecommitController)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigationPrecommitController {
    inner: Any,
}

impl FromVal for NavigationPrecommitController {
    fn from_val(v: &Any) -> Self {
        NavigationPrecommitController {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for NavigationPrecommitController {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for NavigationPrecommitController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for NavigationPrecommitController {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for NavigationPrecommitController {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<NavigationPrecommitController> for Any {
    fn from(s: NavigationPrecommitController) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&NavigationPrecommitController> for Any {
    fn from(s: &NavigationPrecommitController) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(NavigationPrecommitController);

impl NavigationPrecommitController {
    /// The redirect method.
    /// [`NavigationPrecommitController.redirect`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationPrecommitController/redirect)
    pub fn redirect(&self, url: &JsString) -> Undefined {
        self.inner
            .call("redirect", &[url.into()])
            .as_::<Undefined>()
    }
}
impl NavigationPrecommitController {
    /// The redirect method.
    /// [`NavigationPrecommitController.redirect`](https://developer.mozilla.org/en-US/docs/Web/API/NavigationPrecommitController/redirect)
    pub fn redirect_with_options(
        &self,
        url: &JsString,
        options: &NavigationNavigateOptions,
    ) -> Undefined {
        self.inner
            .call("redirect", &[url.into(), options.into()])
            .as_::<Undefined>()
    }
}
