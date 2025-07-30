use super::*;

/// The PaymentManager class.
/// [`PaymentManager`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentManager)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PaymentManager {
    inner: Any,
}
impl FromVal for PaymentManager {
    fn from_val(v: &Any) -> Self {
        PaymentManager {
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
impl core::ops::Deref for PaymentManager {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PaymentManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PaymentManager {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PaymentManager {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PaymentManager> for Any {
    fn from(s: PaymentManager) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PaymentManager> for Any {
    fn from(s: &PaymentManager) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PaymentManager);

impl PaymentManager {
    /// Getter of the `userHint` attribute.
    /// [`PaymentManager.userHint`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentManager/userHint)
    pub fn user_hint(&self) -> JsString {
        self.inner.get("userHint").as_::<JsString>()
    }

    /// Setter of the `userHint` attribute.
    /// [`PaymentManager.userHint`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentManager/userHint)
    pub fn set_user_hint(&mut self, value: &JsString) {
        self.inner.set("userHint", value);
    }
}
impl PaymentManager {
    /// The enableDelegations method.
    /// [`PaymentManager.enableDelegations`](https://developer.mozilla.org/en-US/docs/Web/API/PaymentManager/enableDelegations)
    pub fn enable_delegations(
        &self,
        delegations: &TypedArray<PaymentDelegation>,
    ) -> Promise<Undefined> {
        self.inner
            .call("enableDelegations", &[delegations.into()])
            .as_::<Promise<Undefined>>()
    }
}
