use super::*;

/// The Plugin class.
/// [`Plugin`](https://developer.mozilla.org/en-US/docs/Web/API/Plugin)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Plugin {
    inner: Any,
}
impl FromVal for Plugin {
    fn from_val(v: &Any) -> Self {
        Plugin {
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
impl core::ops::Deref for Plugin {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Plugin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for Plugin {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Plugin {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Plugin> for Any {
    fn from(s: Plugin) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Plugin> for Any {
    fn from(s: &Plugin) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Plugin);

impl Plugin {
    /// Getter of the `name` attribute.
    /// [`Plugin.name`](https://developer.mozilla.org/en-US/docs/Web/API/Plugin/name)
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }
}
impl Plugin {
    /// Getter of the `description` attribute.
    /// [`Plugin.description`](https://developer.mozilla.org/en-US/docs/Web/API/Plugin/description)
    pub fn description(&self) -> DOMString {
        self.inner.get("description").as_::<DOMString>()
    }
}
impl Plugin {
    /// Getter of the `filename` attribute.
    /// [`Plugin.filename`](https://developer.mozilla.org/en-US/docs/Web/API/Plugin/filename)
    pub fn filename(&self) -> DOMString {
        self.inner.get("filename").as_::<DOMString>()
    }
}
impl Plugin {
    /// Getter of the `length` attribute.
    /// [`Plugin.length`](https://developer.mozilla.org/en-US/docs/Web/API/Plugin/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl Plugin {
    /// The item method.
    /// [`Plugin.item`](https://developer.mozilla.org/en-US/docs/Web/API/Plugin/item)
    pub fn item(&self, index: u32) -> MimeType {
        self.inner.call("item", &[index.into()]).as_::<MimeType>()
    }
}
impl Plugin {
    /// The namedItem method.
    /// [`Plugin.namedItem`](https://developer.mozilla.org/en-US/docs/Web/API/Plugin/namedItem)
    pub fn named_item(&self, name: &DOMString) -> MimeType {
        self.inner
            .call("namedItem", &[name.into()])
            .as_::<MimeType>()
    }
}
