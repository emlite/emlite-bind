use super::*;

/// The StylePropertyMapReadOnly class.
/// [`StylePropertyMapReadOnly`](https://developer.mozilla.org/en-US/docs/Web/API/StylePropertyMapReadOnly)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StylePropertyMapReadOnly {
    inner: Any,
}
impl FromVal for StylePropertyMapReadOnly {
    fn from_val(v: &Any) -> Self {
        StylePropertyMapReadOnly {
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
impl core::ops::Deref for StylePropertyMapReadOnly {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for StylePropertyMapReadOnly {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for StylePropertyMapReadOnly {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for StylePropertyMapReadOnly {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<StylePropertyMapReadOnly> for Any {
    fn from(s: StylePropertyMapReadOnly) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&StylePropertyMapReadOnly> for Any {
    fn from(s: &StylePropertyMapReadOnly) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(StylePropertyMapReadOnly);

impl StylePropertyMapReadOnly {
    /// The get method.
    /// [`StylePropertyMapReadOnly.get`](https://developer.mozilla.org/en-US/docs/Web/API/StylePropertyMapReadOnly/get)
    pub fn get(&self, property: &str) -> Any {
        self.inner.call("get", &[property.into()]).as_::<Any>()
    }
}
impl StylePropertyMapReadOnly {
    /// The getAll method.
    /// [`StylePropertyMapReadOnly.getAll`](https://developer.mozilla.org/en-US/docs/Web/API/StylePropertyMapReadOnly/getAll)
    pub fn get_all(&self, property: &str) -> Sequence<CSSStyleValue> {
        self.inner
            .call("getAll", &[property.into()])
            .as_::<Sequence<CSSStyleValue>>()
    }
}
impl StylePropertyMapReadOnly {
    /// The has method.
    /// [`StylePropertyMapReadOnly.has`](https://developer.mozilla.org/en-US/docs/Web/API/StylePropertyMapReadOnly/has)
    pub fn has(&self, property: &str) -> bool {
        self.inner.call("has", &[property.into()]).as_::<bool>()
    }
}
impl StylePropertyMapReadOnly {
    /// Getter of the `size` attribute.
    /// [`StylePropertyMapReadOnly.size`](https://developer.mozilla.org/en-US/docs/Web/API/StylePropertyMapReadOnly/size)
    pub fn size(&self) -> u32 {
        self.inner.get("size").as_::<u32>()
    }
}
