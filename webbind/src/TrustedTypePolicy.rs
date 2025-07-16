use super::*;

/// The TrustedTypePolicy class.
/// [`TrustedTypePolicy`](https://developer.mozilla.org/en-US/docs/Web/API/TrustedTypePolicy)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TrustedTypePolicy {
    inner: Any,
}
impl FromVal for TrustedTypePolicy {
    fn from_val(v: &Any) -> Self {
        TrustedTypePolicy {
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
impl core::ops::Deref for TrustedTypePolicy {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TrustedTypePolicy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for TrustedTypePolicy {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for TrustedTypePolicy {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<TrustedTypePolicy> for Any {
    fn from(s: TrustedTypePolicy) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&TrustedTypePolicy> for Any {
    fn from(s: &TrustedTypePolicy) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(TrustedTypePolicy);

impl TrustedTypePolicy {
    /// Getter of the `name` attribute.
    /// [`TrustedTypePolicy.name`](https://developer.mozilla.org/en-US/docs/Web/API/TrustedTypePolicy/name)
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }
}
impl TrustedTypePolicy {
    /// The createHTML method.
    /// [`TrustedTypePolicy.createHTML`](https://developer.mozilla.org/en-US/docs/Web/API/TrustedTypePolicy/createHTML)
    pub fn create_html(&self, input: &str, arguments: &Any) -> TrustedHTML {
        self.inner
            .call("createHTML", &[input.into(), arguments.into()])
            .as_::<TrustedHTML>()
    }
}
impl TrustedTypePolicy {
    /// The createScript method.
    /// [`TrustedTypePolicy.createScript`](https://developer.mozilla.org/en-US/docs/Web/API/TrustedTypePolicy/createScript)
    pub fn create_script(&self, input: &str, arguments: &Any) -> TrustedScript {
        self.inner
            .call("createScript", &[input.into(), arguments.into()])
            .as_::<TrustedScript>()
    }
}
impl TrustedTypePolicy {
    /// The createScriptURL method.
    /// [`TrustedTypePolicy.createScriptURL`](https://developer.mozilla.org/en-US/docs/Web/API/TrustedTypePolicy/createScriptURL)
    pub fn create_script_url(&self, input: &str, arguments: &Any) -> TrustedScriptURL {
        self.inner
            .call("createScriptURL", &[input.into(), arguments.into()])
            .as_::<TrustedScriptURL>()
    }
}
