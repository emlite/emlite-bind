use super::*;

/// The ContactAddress class.
/// [`ContactAddress`](https://developer.mozilla.org/en-US/docs/Web/API/ContactAddress)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ContactAddress {
    inner: Any,
}
impl FromVal for ContactAddress {
    fn from_val(v: &Any) -> Self {
        ContactAddress {
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
impl core::ops::Deref for ContactAddress {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ContactAddress {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ContactAddress {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ContactAddress {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ContactAddress> for Any {
    fn from(s: ContactAddress) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ContactAddress> for Any {
    fn from(s: &ContactAddress) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ContactAddress);

impl ContactAddress {
    /// The toJSON method.
    /// [`ContactAddress.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/ContactAddress/toJSON)
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
impl ContactAddress {
    /// Getter of the `city` attribute.
    /// [`ContactAddress.city`](https://developer.mozilla.org/en-US/docs/Web/API/ContactAddress/city)
    pub fn city(&self) -> DOMString {
        self.inner.get("city").as_::<DOMString>()
    }
}
impl ContactAddress {
    /// Getter of the `country` attribute.
    /// [`ContactAddress.country`](https://developer.mozilla.org/en-US/docs/Web/API/ContactAddress/country)
    pub fn country(&self) -> DOMString {
        self.inner.get("country").as_::<DOMString>()
    }
}
impl ContactAddress {
    /// Getter of the `dependentLocality` attribute.
    /// [`ContactAddress.dependentLocality`](https://developer.mozilla.org/en-US/docs/Web/API/ContactAddress/dependentLocality)
    pub fn dependent_locality(&self) -> DOMString {
        self.inner.get("dependentLocality").as_::<DOMString>()
    }
}
impl ContactAddress {
    /// Getter of the `organization` attribute.
    /// [`ContactAddress.organization`](https://developer.mozilla.org/en-US/docs/Web/API/ContactAddress/organization)
    pub fn organization(&self) -> DOMString {
        self.inner.get("organization").as_::<DOMString>()
    }
}
impl ContactAddress {
    /// Getter of the `phone` attribute.
    /// [`ContactAddress.phone`](https://developer.mozilla.org/en-US/docs/Web/API/ContactAddress/phone)
    pub fn phone(&self) -> DOMString {
        self.inner.get("phone").as_::<DOMString>()
    }
}
impl ContactAddress {
    /// Getter of the `postalCode` attribute.
    /// [`ContactAddress.postalCode`](https://developer.mozilla.org/en-US/docs/Web/API/ContactAddress/postalCode)
    pub fn postal_code(&self) -> DOMString {
        self.inner.get("postalCode").as_::<DOMString>()
    }
}
impl ContactAddress {
    /// Getter of the `recipient` attribute.
    /// [`ContactAddress.recipient`](https://developer.mozilla.org/en-US/docs/Web/API/ContactAddress/recipient)
    pub fn recipient(&self) -> DOMString {
        self.inner.get("recipient").as_::<DOMString>()
    }
}
impl ContactAddress {
    /// Getter of the `region` attribute.
    /// [`ContactAddress.region`](https://developer.mozilla.org/en-US/docs/Web/API/ContactAddress/region)
    pub fn region(&self) -> DOMString {
        self.inner.get("region").as_::<DOMString>()
    }
}
impl ContactAddress {
    /// Getter of the `sortingCode` attribute.
    /// [`ContactAddress.sortingCode`](https://developer.mozilla.org/en-US/docs/Web/API/ContactAddress/sortingCode)
    pub fn sorting_code(&self) -> DOMString {
        self.inner.get("sortingCode").as_::<DOMString>()
    }
}
impl ContactAddress {
    /// Getter of the `addressLine` attribute.
    /// [`ContactAddress.addressLine`](https://developer.mozilla.org/en-US/docs/Web/API/ContactAddress/addressLine)
    pub fn address_line(&self) -> FrozenArray<DOMString> {
        self.inner
            .get("addressLine")
            .as_::<FrozenArray<DOMString>>()
    }
}
