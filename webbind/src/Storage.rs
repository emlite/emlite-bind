use super::*;




/// The Storage class.
/// [`Storage`](https://developer.mozilla.org/en-US/docs/Web/API/Storage)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Storage {
    inner: Any,
}

impl FromVal for Storage {
    fn from_val(v: &Any) -> Self {
        Storage { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for Storage {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Storage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Storage {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Storage {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<Storage> for Any {
    fn from(s: Storage) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Storage> for Any {
    fn from(s: &Storage) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Storage);


impl Storage {
    /// Getter of the `length` attribute.
    /// [`Storage.length`](https://developer.mozilla.org/en-US/docs/Web/API/Storage/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

}
impl Storage {
    /// The key method.
    /// [`Storage.key`](https://developer.mozilla.org/en-US/docs/Web/API/Storage/key)
    pub fn key(&self, index: u32) -> JsString {
        self.inner.call("key", &[index.into(), ]).as_::<JsString>()
    }
}
impl Storage {
    /// The getItem method.
    /// [`Storage.getItem`](https://developer.mozilla.org/en-US/docs/Web/API/Storage/getItem)
    pub fn get_item(&self, key: &JsString) -> JsString {
        self.inner.call("getItem", &[key.into(), ]).as_::<JsString>()
    }
}
impl Storage {
    /// The setItem method.
    /// [`Storage.setItem`](https://developer.mozilla.org/en-US/docs/Web/API/Storage/setItem)
    pub fn set_item(&self, key: &JsString, value: &JsString) -> Undefined {
        self.inner.call("setItem", &[key.into(), value.into(), ]).as_::<Undefined>()
    }
}
impl Storage {
    /// The removeItem method.
    /// [`Storage.removeItem`](https://developer.mozilla.org/en-US/docs/Web/API/Storage/removeItem)
    pub fn remove_item(&self, key: &JsString) -> Undefined {
        self.inner.call("removeItem", &[key.into(), ]).as_::<Undefined>()
    }
}
impl Storage {
    /// The clear method.
    /// [`Storage.clear`](https://developer.mozilla.org/en-US/docs/Web/API/Storage/clear)
    pub fn clear(&self, ) -> Undefined {
        self.inner.call("clear", &[]).as_::<Undefined>()
    }
}
