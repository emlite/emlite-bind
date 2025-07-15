use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ContactInfo {
    inner: emlite::Val,
}
impl FromVal for ContactInfo {
    fn from_val(v: &emlite::Val) -> Self {
        ContactInfo { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ContactInfo {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ContactInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ContactInfo {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ContactInfo {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ContactInfo> for emlite::Val {
    fn from(s: ContactInfo) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ContactInfo {
    pub fn address(&self) -> Sequence<ContactAddress> {
        self.inner.get("address").as_::<Sequence<ContactAddress>>()
    }

    pub fn set_address(&mut self, value: Sequence<ContactAddress>) {
        self.inner.set("address", value);
    }
}
impl ContactInfo {
    pub fn email(&self) -> Sequence<DOMString> {
        self.inner.get("email").as_::<Sequence<DOMString>>()
    }

    pub fn set_email(&mut self, value: Sequence<DOMString>) {
        self.inner.set("email", value);
    }
}
impl ContactInfo {
    pub fn icon(&self) -> Sequence<Blob> {
        self.inner.get("icon").as_::<Sequence<Blob>>()
    }

    pub fn set_icon(&mut self, value: Sequence<Blob>) {
        self.inner.set("icon", value);
    }
}
impl ContactInfo {
    pub fn name(&self) -> Sequence<DOMString> {
        self.inner.get("name").as_::<Sequence<DOMString>>()
    }

    pub fn set_name(&mut self, value: Sequence<DOMString>) {
        self.inner.set("name", value);
    }
}
impl ContactInfo {
    pub fn tel(&self) -> Sequence<DOMString> {
        self.inner.get("tel").as_::<Sequence<DOMString>>()
    }

    pub fn set_tel(&mut self, value: Sequence<DOMString>) {
        self.inner.set("tel", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ContactsSelectOptions {
    inner: emlite::Val,
}
impl FromVal for ContactsSelectOptions {
    fn from_val(v: &emlite::Val) -> Self {
        ContactsSelectOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ContactsSelectOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ContactsSelectOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ContactsSelectOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ContactsSelectOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ContactsSelectOptions> for emlite::Val {
    fn from(s: ContactsSelectOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ContactsSelectOptions {
    pub fn multiple(&self) -> bool {
        self.inner.get("multiple").as_::<bool>()
    }

    pub fn set_multiple(&mut self, value: bool) {
        self.inner.set("multiple", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ContactsManager {
    inner: emlite::Val,
}
impl FromVal for ContactsManager {
    fn from_val(v: &emlite::Val) -> Self {
        ContactsManager {
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
impl core::ops::Deref for ContactsManager {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ContactsManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ContactsManager {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ContactsManager {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ContactsManager> for emlite::Val {
    fn from(s: ContactsManager) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(ContactsManager);

impl ContactsManager {
    pub fn get_properties(&self) -> Promise {
        self.inner.call("getProperties", &[]).as_::<Promise>()
    }
}
impl ContactsManager {
    pub fn select0(&self, properties: Sequence<ContactProperty>) -> Promise {
        self.inner
            .call("select", &[properties.into()])
            .as_::<Promise>()
    }

    pub fn select1(
        &self,
        properties: Sequence<ContactProperty>,
        options: ContactsSelectOptions,
    ) -> Promise {
        self.inner
            .call("select", &[properties.into(), options.into()])
            .as_::<Promise>()
    }
}
