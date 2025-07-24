use super::*;

/// The PluginArray class.
/// [`PluginArray`](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PluginArray {
    inner: Any,
}
impl FromVal for PluginArray {
    fn from_val(v: &Any) -> Self {
        PluginArray {
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
impl core::ops::Deref for PluginArray {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PluginArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PluginArray {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PluginArray {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PluginArray> for Any {
    fn from(s: PluginArray) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PluginArray> for Any {
    fn from(s: &PluginArray) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PluginArray);

impl PluginArray {
    /// The refresh method.
    /// [`PluginArray.refresh`](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray/refresh)
    pub fn refresh(&self) -> Undefined {
        self.inner.call("refresh", &[]).as_::<Undefined>()
    }
}
impl PluginArray {
    /// Getter of the `length` attribute.
    /// [`PluginArray.length`](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl PluginArray {
    /// The item method.
    /// [`PluginArray.item`](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray/item)
    pub fn item(&self, index: u32) -> Plugin {
        self.inner.call("item", &[index.into()]).as_::<Plugin>()
    }
}
impl PluginArray {
    /// The namedItem method.
    /// [`PluginArray.namedItem`](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray/namedItem)
    pub fn named_item(&self, name: &DOMString) -> Plugin {
        self.inner.call("namedItem", &[name.into()]).as_::<Plugin>()
    }
}
