use super::*;




/// The ContactsManager class.
/// [`ContactsManager`](https://developer.mozilla.org/en-US/docs/Web/API/ContactsManager)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ContactsManager {
    inner: Any,
}

impl FromVal for ContactsManager {
    fn from_val(v: &Any) -> Self {
        ContactsManager { inner: Any::from_val(v) }
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
    pub fn get_properties(&self, ) -> Promise<TypedArray<ContactProperty>> {
        self.inner.call("getProperties", &[]).as_::<Promise<TypedArray<ContactProperty>>>()
    }
}
impl ContactsManager {
    /// The select method.
    /// [`ContactsManager.select`](https://developer.mozilla.org/en-US/docs/Web/API/ContactsManager/select)
    pub fn select0(&self, properties: &TypedArray<ContactProperty>) -> Promise<TypedArray<ContactInfo>> {
        self.inner.call("select", &[properties.into(), ]).as_::<Promise<TypedArray<ContactInfo>>>()
    }
    /// The select method.
    /// [`ContactsManager.select`](https://developer.mozilla.org/en-US/docs/Web/API/ContactsManager/select)
    pub fn select1(&self, properties: &TypedArray<ContactProperty>, options: &ContactsSelectOptions) -> Promise<TypedArray<ContactInfo>> {
        self.inner.call("select", &[properties.into(), options.into(), ]).as_::<Promise<TypedArray<ContactInfo>>>()
    }
}
