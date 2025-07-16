use super::*;

/// The DOMParser class.
/// [`DOMParser`](https://developer.mozilla.org/en-US/docs/Web/API/DOMParser)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DOMParser {
    inner: Any,
}
impl FromVal for DOMParser {
    fn from_val(v: &Any) -> Self {
        DOMParser {
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
impl core::ops::Deref for DOMParser {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DOMParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for DOMParser {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for DOMParser {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<DOMParser> for Any {
    fn from(s: DOMParser) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&DOMParser> for Any {
    fn from(s: &DOMParser) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(DOMParser);

impl DOMParser {
    /// The `new DOMParser(..)` constructor, creating a new DOMParser instance
    pub fn new() -> DOMParser {
        Self {
            inner: Any::global("DOMParser").new(&[]).as_::<Any>(),
        }
    }
}
impl DOMParser {
    /// The parseFromString method.
    /// [`DOMParser.parseFromString`](https://developer.mozilla.org/en-US/docs/Web/API/DOMParser/parseFromString)
    pub fn parse_from_string(&self, string: &Any, type_: &DOMParserSupportedType) -> Document {
        self.inner
            .call("parseFromString", &[string.into(), type_.into()])
            .as_::<Document>()
    }
}
