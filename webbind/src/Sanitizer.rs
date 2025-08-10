use super::*;

/// The Sanitizer class.
/// [`Sanitizer`](https://developer.mozilla.org/en-US/docs/Web/API/Sanitizer)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Sanitizer {
    inner: Any,
}

impl FromVal for Sanitizer {
    fn from_val(v: &Any) -> Self {
        Sanitizer {
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

impl core::ops::Deref for Sanitizer {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Sanitizer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Sanitizer {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Sanitizer {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<Sanitizer> for Any {
    fn from(s: Sanitizer) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Sanitizer> for Any {
    fn from(s: &Sanitizer) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Sanitizer);

impl Sanitizer {
    /// The `new Sanitizer(..)` constructor, creating a new Sanitizer instance
    pub fn new0() -> Sanitizer {
        Self {
            inner: Any::global("Sanitizer").new(&[]).as_::<Any>(),
        }
    }

    /// The `new Sanitizer(..)` constructor, creating a new Sanitizer instance
    pub fn new1(configuration: &Any) -> Sanitizer {
        Self {
            inner: Any::global("Sanitizer")
                .new(&[configuration.into()])
                .as_::<Any>(),
        }
    }
}
impl Sanitizer {
    /// The get method.
    /// [`Sanitizer.get`](https://developer.mozilla.org/en-US/docs/Web/API/Sanitizer/get)
    pub fn get(&self) -> SanitizerConfig {
        self.inner.call("get", &[]).as_::<SanitizerConfig>()
    }
}
impl Sanitizer {
    /// The allowElement method.
    /// [`Sanitizer.allowElement`](https://developer.mozilla.org/en-US/docs/Web/API/Sanitizer/allowElement)
    pub fn allow_element(&self, element: &Any) -> Undefined {
        self.inner
            .call("allowElement", &[element.into()])
            .as_::<Undefined>()
    }
}
impl Sanitizer {
    /// The removeElement method.
    /// [`Sanitizer.removeElement`](https://developer.mozilla.org/en-US/docs/Web/API/Sanitizer/removeElement)
    pub fn remove_element(&self, element: &Any) -> Undefined {
        self.inner
            .call("removeElement", &[element.into()])
            .as_::<Undefined>()
    }
}
impl Sanitizer {
    /// The replaceElementWithChildren method.
    /// [`Sanitizer.replaceElementWithChildren`](https://developer.mozilla.org/en-US/docs/Web/API/Sanitizer/replaceElementWithChildren)
    pub fn replace_element_with_children(&self, element: &Any) -> Undefined {
        self.inner
            .call("replaceElementWithChildren", &[element.into()])
            .as_::<Undefined>()
    }
}
impl Sanitizer {
    /// The allowAttribute method.
    /// [`Sanitizer.allowAttribute`](https://developer.mozilla.org/en-US/docs/Web/API/Sanitizer/allowAttribute)
    pub fn allow_attribute(&self, attribute: &Any) -> Undefined {
        self.inner
            .call("allowAttribute", &[attribute.into()])
            .as_::<Undefined>()
    }
}
impl Sanitizer {
    /// The removeAttribute method.
    /// [`Sanitizer.removeAttribute`](https://developer.mozilla.org/en-US/docs/Web/API/Sanitizer/removeAttribute)
    pub fn remove_attribute(&self, attribute: &Any) -> Undefined {
        self.inner
            .call("removeAttribute", &[attribute.into()])
            .as_::<Undefined>()
    }
}
impl Sanitizer {
    /// The setComments method.
    /// [`Sanitizer.setComments`](https://developer.mozilla.org/en-US/docs/Web/API/Sanitizer/setComments)
    pub fn set_comments(&self, allow: bool) -> Undefined {
        self.inner
            .call("setComments", &[allow.into()])
            .as_::<Undefined>()
    }
}
impl Sanitizer {
    /// The setDataAttributes method.
    /// [`Sanitizer.setDataAttributes`](https://developer.mozilla.org/en-US/docs/Web/API/Sanitizer/setDataAttributes)
    pub fn set_data_attributes(&self, allow: bool) -> Undefined {
        self.inner
            .call("setDataAttributes", &[allow.into()])
            .as_::<Undefined>()
    }
}
impl Sanitizer {
    /// The removeUnsafe method.
    /// [`Sanitizer.removeUnsafe`](https://developer.mozilla.org/en-US/docs/Web/API/Sanitizer/removeUnsafe)
    pub fn remove_unsafe(&self) -> Undefined {
        self.inner.call("removeUnsafe", &[]).as_::<Undefined>()
    }
}
