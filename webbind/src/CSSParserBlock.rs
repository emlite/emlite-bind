use super::*;

/// The CSSParserBlock class.
/// [`CSSParserBlock`](https://developer.mozilla.org/en-US/docs/Web/API/CSSParserBlock)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSParserBlock {
    inner: CSSParserValue,
}
impl FromVal for CSSParserBlock {
    fn from_val(v: &Any) -> Self {
        CSSParserBlock {
            inner: CSSParserValue::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSParserBlock {
    type Target = CSSParserValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSParserBlock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CSSParserBlock {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CSSParserBlock {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CSSParserBlock> for Any {
    fn from(s: CSSParserBlock) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CSSParserBlock> for Any {
    fn from(s: &CSSParserBlock) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSParserBlock);

impl CSSParserBlock {
    /// The `new CSSParserBlock(..)` constructor, creating a new CSSParserBlock instance
    pub fn new(name: &str, body: &Sequence<CSSParserValue>) -> CSSParserBlock {
        Self {
            inner: Any::global("CSSParserBlock")
                .new(&[name.into(), body.into()])
                .as_::<CSSParserValue>(),
        }
    }
}
impl CSSParserBlock {
    /// Getter of the `name` attribute.
    /// [`CSSParserBlock.name`](https://developer.mozilla.org/en-US/docs/Web/API/CSSParserBlock/name)
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }
}
impl CSSParserBlock {
    /// Getter of the `body` attribute.
    /// [`CSSParserBlock.body`](https://developer.mozilla.org/en-US/docs/Web/API/CSSParserBlock/body)
    pub fn body(&self) -> FrozenArray<CSSParserValue> {
        self.inner.get("body").as_::<FrozenArray<CSSParserValue>>()
    }
}
