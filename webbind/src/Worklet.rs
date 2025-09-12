use super::*;

/// The Worklet class.
/// [`Worklet`](https://developer.mozilla.org/en-US/docs/Web/API/Worklet)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Worklet {
    inner: Any,
}

impl FromVal for Worklet {
    fn from_val(v: &Any) -> Self {
        Worklet {
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

impl core::ops::Deref for Worklet {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Worklet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Worklet {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Worklet {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<Worklet> for Any {
    fn from(s: Worklet) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Worklet> for Any {
    fn from(s: &Worklet) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Worklet);

impl Worklet {
    /// The addModule method.
    /// [`Worklet.addModule`](https://developer.mozilla.org/en-US/docs/Web/API/Worklet/addModule)
    pub fn add_module(&self, module_url: &JsString) -> Promise<Undefined> {
        self.inner
            .call("addModule", &[module_url.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl Worklet {
    /// The addModule method.
    /// [`Worklet.addModule`](https://developer.mozilla.org/en-US/docs/Web/API/Worklet/addModule)
    pub fn add_module_with_options(
        &self,
        module_url: &JsString,
        options: &WorkletOptions,
    ) -> Promise<Undefined> {
        self.inner
            .call("addModule", &[module_url.into(), options.into()])
            .as_::<Promise<Undefined>>()
    }
}
