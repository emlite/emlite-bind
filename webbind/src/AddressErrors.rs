use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AddressErrors {
    inner: Any,
}
impl FromVal for AddressErrors {
    fn from_val(v: &Any) -> Self {
        AddressErrors { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AddressErrors {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AddressErrors {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AddressErrors {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AddressErrors {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AddressErrors> for Any {
    fn from(s: AddressErrors) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AddressErrors> for Any {
    fn from(s: &AddressErrors) -> Any {
        s.inner.clone()
    }
}

impl AddressErrors {
    pub fn address_line(&self) -> JsString {
        self.inner.get("addressLine").as_::<JsString>()
    }

    pub fn set_address_line(&mut self, value: &JsString) {
        self.inner.set("addressLine", value);
    }
}
impl AddressErrors {
    pub fn city(&self) -> JsString {
        self.inner.get("city").as_::<JsString>()
    }

    pub fn set_city(&mut self, value: &JsString) {
        self.inner.set("city", value);
    }
}
impl AddressErrors {
    pub fn country(&self) -> JsString {
        self.inner.get("country").as_::<JsString>()
    }

    pub fn set_country(&mut self, value: &JsString) {
        self.inner.set("country", value);
    }
}
impl AddressErrors {
    pub fn dependent_locality(&self) -> JsString {
        self.inner.get("dependentLocality").as_::<JsString>()
    }

    pub fn set_dependent_locality(&mut self, value: &JsString) {
        self.inner.set("dependentLocality", value);
    }
}
impl AddressErrors {
    pub fn organization(&self) -> JsString {
        self.inner.get("organization").as_::<JsString>()
    }

    pub fn set_organization(&mut self, value: &JsString) {
        self.inner.set("organization", value);
    }
}
impl AddressErrors {
    pub fn phone(&self) -> JsString {
        self.inner.get("phone").as_::<JsString>()
    }

    pub fn set_phone(&mut self, value: &JsString) {
        self.inner.set("phone", value);
    }
}
impl AddressErrors {
    pub fn postal_code(&self) -> JsString {
        self.inner.get("postalCode").as_::<JsString>()
    }

    pub fn set_postal_code(&mut self, value: &JsString) {
        self.inner.set("postalCode", value);
    }
}
impl AddressErrors {
    pub fn recipient(&self) -> JsString {
        self.inner.get("recipient").as_::<JsString>()
    }

    pub fn set_recipient(&mut self, value: &JsString) {
        self.inner.set("recipient", value);
    }
}
impl AddressErrors {
    pub fn region(&self) -> JsString {
        self.inner.get("region").as_::<JsString>()
    }

    pub fn set_region(&mut self, value: &JsString) {
        self.inner.set("region", value);
    }
}
impl AddressErrors {
    pub fn sorting_code(&self) -> JsString {
        self.inner.get("sortingCode").as_::<JsString>()
    }

    pub fn set_sorting_code(&mut self, value: &JsString) {
        self.inner.set("sortingCode", value);
    }
}
