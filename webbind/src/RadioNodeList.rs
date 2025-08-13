use super::*;




/// The RadioNodeList class.
/// [`RadioNodeList`](https://developer.mozilla.org/en-US/docs/Web/API/RadioNodeList)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RadioNodeList {
    inner: NodeList,
}

impl FromVal for RadioNodeList {
    fn from_val(v: &Any) -> Self {
        RadioNodeList { inner: NodeList::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RadioNodeList {
    type Target = NodeList;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RadioNodeList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RadioNodeList {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RadioNodeList {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RadioNodeList> for Any {
    fn from(s: RadioNodeList) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RadioNodeList> for Any {
    fn from(s: &RadioNodeList) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(RadioNodeList);


impl RadioNodeList {
    /// Getter of the `value` attribute.
    /// [`RadioNodeList.value`](https://developer.mozilla.org/en-US/docs/Web/API/RadioNodeList/value)
    pub fn value(&self) -> JsString {
        self.inner.get("value").as_::<JsString>()
    }

    /// Setter of the `value` attribute.
    /// [`RadioNodeList.value`](https://developer.mozilla.org/en-US/docs/Web/API/RadioNodeList/value)
    pub fn set_value(&mut self, value: &JsString) {
        self.inner.set("value", value);
    }
}
