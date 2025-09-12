use super::*;

/// The CSSParserFunction class.
/// [`CSSParserFunction`](https://developer.mozilla.org/en-US/docs/Web/API/CSSParserFunction)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSParserFunction {
    inner: CSSParserValue,
}

impl FromVal for CSSParserFunction {
    fn from_val(v: &Any) -> Self {
        CSSParserFunction {
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

impl core::ops::Deref for CSSParserFunction {
    type Target = CSSParserValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSParserFunction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSParserFunction {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSParserFunction {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CSSParserFunction> for Any {
    fn from(s: CSSParserFunction) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSParserFunction> for Any {
    fn from(s: &CSSParserFunction) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSParserFunction);

impl CSSParserFunction {
    /// Getter of the `name` attribute.
    /// [`CSSParserFunction.name`](https://developer.mozilla.org/en-US/docs/Web/API/CSSParserFunction/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }
}
impl CSSParserFunction {
    /// Getter of the `args` attribute.
    /// [`CSSParserFunction.args`](https://developer.mozilla.org/en-US/docs/Web/API/CSSParserFunction/args)
    pub fn args(&self) -> TypedArray<TypedArray<CSSParserValue>> {
        self.inner
            .get("args")
            .as_::<TypedArray<TypedArray<CSSParserValue>>>()
    }
}

impl CSSParserFunction {
    /// The `new CSSParserFunction(..)` constructor, creating a new CSSParserFunction instance
    pub fn new(
        name: &JsString,
        args: &TypedArray<TypedArray<CSSParserValue>>,
    ) -> CSSParserFunction {
        Self {
            inner: Any::global("CSSParserFunction")
                .new(&[name.into(), args.into()])
                .as_::<CSSParserValue>(),
        }
    }
}
