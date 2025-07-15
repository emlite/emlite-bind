use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ContactAddress {
    inner: emlite::Val,
}
impl FromVal for ContactAddress {
    fn from_val(v: &emlite::Val) -> Self {
        ContactAddress {
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
impl core::ops::Deref for ContactAddress {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ContactAddress {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ContactAddress {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ContactAddress {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ContactAddress> for emlite::Val {
    fn from(s: ContactAddress) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(ContactAddress);

impl ContactAddress {
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
impl ContactAddress {
    pub fn city(&self) -> DOMString {
        self.inner.get("city").as_::<DOMString>()
    }
}
impl ContactAddress {
    pub fn country(&self) -> DOMString {
        self.inner.get("country").as_::<DOMString>()
    }
}
impl ContactAddress {
    pub fn dependent_locality(&self) -> DOMString {
        self.inner.get("dependentLocality").as_::<DOMString>()
    }
}
impl ContactAddress {
    pub fn organization(&self) -> DOMString {
        self.inner.get("organization").as_::<DOMString>()
    }
}
impl ContactAddress {
    pub fn phone(&self) -> DOMString {
        self.inner.get("phone").as_::<DOMString>()
    }
}
impl ContactAddress {
    pub fn postal_code(&self) -> DOMString {
        self.inner.get("postalCode").as_::<DOMString>()
    }
}
impl ContactAddress {
    pub fn recipient(&self) -> DOMString {
        self.inner.get("recipient").as_::<DOMString>()
    }
}
impl ContactAddress {
    pub fn region(&self) -> DOMString {
        self.inner.get("region").as_::<DOMString>()
    }
}
impl ContactAddress {
    pub fn sorting_code(&self) -> DOMString {
        self.inner.get("sortingCode").as_::<DOMString>()
    }
}
impl ContactAddress {
    pub fn address_line(&self) -> FrozenArray<DOMString> {
        self.inner
            .get("addressLine")
            .as_::<FrozenArray<DOMString>>()
    }
}
