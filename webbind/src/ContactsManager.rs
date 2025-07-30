use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ContactInfo {
    inner: Any,
}
impl FromVal for ContactInfo {
    fn from_val(v: &Any) -> Self {
        ContactInfo { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ContactInfo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ContactInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ContactInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ContactInfo {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ContactInfo> for Any {
    fn from(s: ContactInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ContactInfo> for Any {
    fn from(s: &ContactInfo) -> Any {
        s.inner.clone()
    }
}

impl ContactInfo {
    pub fn address(&self) -> TypedArray<ContactAddress> {
        self.inner
            .get("address")
            .as_::<TypedArray<ContactAddress>>()
    }

    pub fn set_address(&mut self, value: &TypedArray<ContactAddress>) {
        self.inner.set("address", value);
    }
}
impl ContactInfo {
    pub fn email(&self) -> TypedArray<JsString> {
        self.inner.get("email").as_::<TypedArray<JsString>>()
    }

    pub fn set_email(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("email", value);
    }
}
impl ContactInfo {
    pub fn icon(&self) -> TypedArray<Blob> {
        self.inner.get("icon").as_::<TypedArray<Blob>>()
    }

    pub fn set_icon(&mut self, value: &TypedArray<Blob>) {
        self.inner.set("icon", value);
    }
}
impl ContactInfo {
    pub fn name(&self) -> TypedArray<JsString> {
        self.inner.get("name").as_::<TypedArray<JsString>>()
    }

    pub fn set_name(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("name", value);
    }
}
impl ContactInfo {
    pub fn tel(&self) -> TypedArray<JsString> {
        self.inner.get("tel").as_::<TypedArray<JsString>>()
    }

    pub fn set_tel(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("tel", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ContactsSelectOptions {
    inner: Any,
}
impl FromVal for ContactsSelectOptions {
    fn from_val(v: &Any) -> Self {
        ContactsSelectOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ContactsSelectOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ContactsSelectOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ContactsSelectOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ContactsSelectOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ContactsSelectOptions> for Any {
    fn from(s: ContactsSelectOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ContactsSelectOptions> for Any {
    fn from(s: &ContactsSelectOptions) -> Any {
        s.inner.clone()
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
/// The ContactsManager class.
/// [`ContactsManager`](https://developer.mozilla.org/en-US/docs/Web/API/ContactsManager)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ContactsManager {
    inner: Any,
}
impl FromVal for ContactsManager {
    fn from_val(v: &Any) -> Self {
        ContactsManager {
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
impl core::ops::Deref for ContactsManager {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ContactsManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ContactsManager {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ContactsManager {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ContactsManager> for Any {
    fn from(s: ContactsManager) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ContactsManager> for Any {
    fn from(s: &ContactsManager) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ContactsManager);

impl ContactsManager {
    /// The getProperties method.
    /// [`ContactsManager.getProperties`](https://developer.mozilla.org/en-US/docs/Web/API/ContactsManager/getProperties)
    pub fn get_properties(&self) -> Promise<TypedArray<ContactProperty>> {
        self.inner
            .call("getProperties", &[])
            .as_::<Promise<TypedArray<ContactProperty>>>()
    }
}
impl ContactsManager {
    /// The select method.
    /// [`ContactsManager.select`](https://developer.mozilla.org/en-US/docs/Web/API/ContactsManager/select)
    pub fn select0(
        &self,
        properties: &TypedArray<ContactProperty>,
    ) -> Promise<TypedArray<ContactInfo>> {
        self.inner
            .call("select", &[properties.into()])
            .as_::<Promise<TypedArray<ContactInfo>>>()
    }
    /// The select method.
    /// [`ContactsManager.select`](https://developer.mozilla.org/en-US/docs/Web/API/ContactsManager/select)
    pub fn select1(
        &self,
        properties: &TypedArray<ContactProperty>,
        options: &ContactsSelectOptions,
    ) -> Promise<TypedArray<ContactInfo>> {
        self.inner
            .call("select", &[properties.into(), options.into()])
            .as_::<Promise<TypedArray<ContactInfo>>>()
    }
}
