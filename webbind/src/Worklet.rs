use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WorkletOptions {
    inner: Any,
}
impl FromVal for WorkletOptions {
    fn from_val(v: &Any) -> Self {
        WorkletOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WorkletOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WorkletOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for WorkletOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for WorkletOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<WorkletOptions> for Any {
    fn from(s: WorkletOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&WorkletOptions> for Any {
    fn from(s: &WorkletOptions) -> Any {
        s.inner.clone()
    }
}

impl WorkletOptions {
    pub fn credentials(&self) -> RequestCredentials {
        self.inner.get("credentials").as_::<RequestCredentials>()
    }

    pub fn set_credentials(&mut self, value: &RequestCredentials) {
        self.inner.set("credentials", value);
    }
}
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
    pub fn add_module0(&self, module_url: &USVString) -> Promise<Undefined> {
        self.inner
            .call("addModule", &[module_url.into()])
            .as_::<Promise<Undefined>>()
    }
    /// The addModule method.
    /// [`Worklet.addModule`](https://developer.mozilla.org/en-US/docs/Web/API/Worklet/addModule)
    pub fn add_module1(
        &self,
        module_url: &USVString,
        options: &WorkletOptions,
    ) -> Promise<Undefined> {
        self.inner
            .call("addModule", &[module_url.into(), options.into()])
            .as_::<Promise<Undefined>>()
    }
}
