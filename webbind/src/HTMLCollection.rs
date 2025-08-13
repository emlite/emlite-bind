use super::*;




/// The HTMLCollection class.
/// [`HTMLCollection`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCollection)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLCollection {
    inner: Any,
}

impl FromVal for HTMLCollection {
    fn from_val(v: &Any) -> Self {
        HTMLCollection { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HTMLCollection {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HTMLCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HTMLCollection {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLCollection {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<HTMLCollection> for Any {
    fn from(s: HTMLCollection) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLCollection> for Any {
    fn from(s: &HTMLCollection) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLCollection);


impl HTMLCollection {
    /// Getter of the `length` attribute.
    /// [`HTMLCollection.length`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCollection/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

}
impl HTMLCollection {
    /// The item method.
    /// [`HTMLCollection.item`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCollection/item)
    pub fn item(&self, index: u32) -> Element {
        self.inner.call("item", &[index.into(), ]).as_::<Element>()
    }
}
impl HTMLCollection {
    /// The namedItem method.
    /// [`HTMLCollection.namedItem`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCollection/namedItem)
    pub fn named_item(&self, name: &JsString) -> Element {
        self.inner.call("namedItem", &[name.into(), ]).as_::<Element>()
    }
}
