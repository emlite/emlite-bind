use super::*;

/// The XMLSerializer class.
/// [`XMLSerializer`](https://developer.mozilla.org/en-US/docs/Web/API/XMLSerializer)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XMLSerializer {
    inner: Any,
}

impl FromVal for XMLSerializer {
    fn from_val(v: &Any) -> Self {
        XMLSerializer {
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

impl core::ops::Deref for XMLSerializer {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XMLSerializer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XMLSerializer {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XMLSerializer {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<XMLSerializer> for Any {
    fn from(s: XMLSerializer) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XMLSerializer> for Any {
    fn from(s: &XMLSerializer) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XMLSerializer);

impl XMLSerializer {
    /// The `new XMLSerializer(..)` constructor, creating a new XMLSerializer instance
    pub fn new() -> XMLSerializer {
        Self {
            inner: Any::global("XMLSerializer").new(&[]).as_::<Any>(),
        }
    }
}
impl XMLSerializer {
    /// The serializeToString method.
    /// [`XMLSerializer.serializeToString`](https://developer.mozilla.org/en-US/docs/Web/API/XMLSerializer/serializeToString)
    pub fn serialize_to_string(&self, root: &Node) -> JsString {
        self.inner
            .call("serializeToString", &[root.into()])
            .as_::<JsString>()
    }
}
