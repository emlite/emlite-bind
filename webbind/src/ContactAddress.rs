use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for ContactAddress {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ContactAddress {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ContactAddress> for emlite::Val {
    fn from(s: ContactAddress) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ContactAddress {
    pub fn to_json(&self) -> jsbind::Object {
        self.inner.call("toJSON", &[]).as_::<jsbind::Object>()
    }
}
impl ContactAddress {
    pub fn city(&self) -> jsbind::DOMString {
        self.inner.get("city").as_::<jsbind::DOMString>()
    }
}
impl ContactAddress {
    pub fn country(&self) -> jsbind::DOMString {
        self.inner.get("country").as_::<jsbind::DOMString>()
    }
}
impl ContactAddress {
    pub fn dependent_locality(&self) -> jsbind::DOMString {
        self.inner
            .get("dependentLocality")
            .as_::<jsbind::DOMString>()
    }
}
impl ContactAddress {
    pub fn organization(&self) -> jsbind::DOMString {
        self.inner.get("organization").as_::<jsbind::DOMString>()
    }
}
impl ContactAddress {
    pub fn phone(&self) -> jsbind::DOMString {
        self.inner.get("phone").as_::<jsbind::DOMString>()
    }
}
impl ContactAddress {
    pub fn postal_code(&self) -> jsbind::DOMString {
        self.inner.get("postalCode").as_::<jsbind::DOMString>()
    }
}
impl ContactAddress {
    pub fn recipient(&self) -> jsbind::DOMString {
        self.inner.get("recipient").as_::<jsbind::DOMString>()
    }
}
impl ContactAddress {
    pub fn region(&self) -> jsbind::DOMString {
        self.inner.get("region").as_::<jsbind::DOMString>()
    }
}
impl ContactAddress {
    pub fn sorting_code(&self) -> jsbind::DOMString {
        self.inner.get("sortingCode").as_::<jsbind::DOMString>()
    }
}
impl ContactAddress {
    pub fn address_line(&self) -> jsbind::FrozenArray<jsbind::DOMString> {
        self.inner
            .get("addressLine")
            .as_::<jsbind::FrozenArray<jsbind::DOMString>>()
    }
}
