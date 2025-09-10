use super::*;

/// The HTMLAllCollection class.
/// [`HTMLAllCollection`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAllCollection)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLAllCollection {
    inner: Any,
}

impl FromVal for HTMLAllCollection {
    fn from_val(v: &Any) -> Self {
        HTMLAllCollection {
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

impl core::ops::Deref for HTMLAllCollection {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HTMLAllCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HTMLAllCollection {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HTMLAllCollection {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<HTMLAllCollection> for Any {
    fn from(s: HTMLAllCollection) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HTMLAllCollection> for Any {
    fn from(s: &HTMLAllCollection) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HTMLAllCollection);

impl HTMLAllCollection {
    /// Getter of the `length` attribute.
    /// [`HTMLAllCollection.length`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAllCollection/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl HTMLAllCollection {
    /// The namedItem method.
    /// [`HTMLAllCollection.namedItem`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAllCollection/namedItem)
    pub fn named_item(&self, name: &JsString) -> Any {
        self.inner.call("namedItem", &[name.into()]).as_::<Any>()
    }
}
impl HTMLAllCollection {
    /// The item method.
    /// [`HTMLAllCollection.item`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAllCollection/item)
    pub fn item0(&self) -> Any {
        self.inner.call("item", &[]).as_::<Any>()
    }
    /// The item method.
    /// [`HTMLAllCollection.item`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAllCollection/item)
    pub fn item1(&self, name_or_index: &JsString) -> Any {
        self.inner
            .call("item", &[name_or_index.into()])
            .as_::<Any>()
    }
}
