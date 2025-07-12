use super::*;

#[derive(Clone, Debug)]
pub struct PaymentManager {
    inner: emlite::Val,
}
impl FromVal for PaymentManager {
    fn from_val(v: &emlite::Val) -> Self {
        PaymentManager {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for PaymentManager {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PaymentManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PaymentManager> for emlite::Val {
    fn from(s: PaymentManager) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PaymentManager {
    pub fn user_hint(&self) -> jsbind::DOMString {
        self.inner.get("userHint").as_::<jsbind::DOMString>()
    }

    pub fn set_user_hint(&mut self, value: jsbind::DOMString) {
        self.inner.set("userHint", value);
    }
}
impl PaymentManager {
    pub fn enable_delegations(
        &self,
        delegations: jsbind::Sequence<PaymentDelegation>,
    ) -> jsbind::Promise {
        self.inner
            .call("enableDelegations", &[delegations.into()])
            .as_::<jsbind::Promise>()
    }
}
