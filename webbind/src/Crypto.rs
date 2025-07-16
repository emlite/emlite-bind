use super::*;

/// The Crypto class.
/// [`Crypto`](https://developer.mozilla.org/en-US/docs/Web/API/Crypto)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Crypto {
    inner: Any,
}
impl FromVal for Crypto {
    fn from_val(v: &Any) -> Self {
        Crypto {
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
impl core::ops::Deref for Crypto {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Crypto {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for Crypto {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Crypto {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Crypto> for Any {
    fn from(s: Crypto) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Crypto> for Any {
    fn from(s: &Crypto) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Crypto);

impl Crypto {
    /// Getter of the `subtle` attribute.
    /// [`Crypto.subtle`](https://developer.mozilla.org/en-US/docs/Web/API/Crypto/subtle)
    pub fn subtle(&self) -> SubtleCrypto {
        self.inner.get("subtle").as_::<SubtleCrypto>()
    }
}
impl Crypto {
    /// The getRandomValues method.
    /// [`Crypto.getRandomValues`](https://developer.mozilla.org/en-US/docs/Web/API/Crypto/getRandomValues)
    pub fn get_random_values(&self, array: &Any) -> Any {
        self.inner
            .call("getRandomValues", &[array.into()])
            .as_::<Any>()
    }
}
impl Crypto {
    /// The randomUUID method.
    /// [`Crypto.randomUUID`](https://developer.mozilla.org/en-US/docs/Web/API/Crypto/randomUUID)
    pub fn random_uuid(&self) -> String {
        self.inner.call("randomUUID", &[]).as_::<String>()
    }
}
