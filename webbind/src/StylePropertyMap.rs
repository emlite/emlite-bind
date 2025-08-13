use super::*;




/// The StylePropertyMap class.
/// [`StylePropertyMap`](https://developer.mozilla.org/en-US/docs/Web/API/StylePropertyMap)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StylePropertyMap {
    inner: StylePropertyMapReadOnly,
}

impl FromVal for StylePropertyMap {
    fn from_val(v: &Any) -> Self {
        StylePropertyMap { inner: StylePropertyMapReadOnly::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for StylePropertyMap {
    type Target = StylePropertyMapReadOnly;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for StylePropertyMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for StylePropertyMap {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for StylePropertyMap {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<StylePropertyMap> for Any {
    fn from(s: StylePropertyMap) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&StylePropertyMap> for Any {
    fn from(s: &StylePropertyMap) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(StylePropertyMap);


impl StylePropertyMap {
    /// The set method.
    /// [`StylePropertyMap.set`](https://developer.mozilla.org/en-US/docs/Web/API/StylePropertyMap/set)
    pub fn set(&self, property: &JsString, values: &Any) -> Undefined {
        self.inner.call("set", &[property.into(), values.into(), ]).as_::<Undefined>()
    }
}
impl StylePropertyMap {
    /// The append method.
    /// [`StylePropertyMap.append`](https://developer.mozilla.org/en-US/docs/Web/API/StylePropertyMap/append)
    pub fn append(&self, property: &JsString, values: &Any) -> Undefined {
        self.inner.call("append", &[property.into(), values.into(), ]).as_::<Undefined>()
    }
}
impl StylePropertyMap {
    /// The delete method.
    /// [`StylePropertyMap.delete`](https://developer.mozilla.org/en-US/docs/Web/API/StylePropertyMap/delete)
    pub fn delete(&self, property: &JsString) -> Undefined {
        self.inner.call("delete", &[property.into(), ]).as_::<Undefined>()
    }
}
impl StylePropertyMap {
    /// The clear method.
    /// [`StylePropertyMap.clear`](https://developer.mozilla.org/en-US/docs/Web/API/StylePropertyMap/clear)
    pub fn clear(&self, ) -> Undefined {
        self.inner.call("clear", &[]).as_::<Undefined>()
    }
}
