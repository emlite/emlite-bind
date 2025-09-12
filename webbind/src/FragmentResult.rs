use super::*;

/// The FragmentResult class.
/// [`FragmentResult`](https://developer.mozilla.org/en-US/docs/Web/API/FragmentResult)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FragmentResult {
    inner: Any,
}

impl FromVal for FragmentResult {
    fn from_val(v: &Any) -> Self {
        FragmentResult {
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

impl core::ops::Deref for FragmentResult {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for FragmentResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for FragmentResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for FragmentResult {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<FragmentResult> for Any {
    fn from(s: FragmentResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&FragmentResult> for Any {
    fn from(s: &FragmentResult) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(FragmentResult);

impl FragmentResult {
    /// Getter of the `inlineSize` attribute.
    /// [`FragmentResult.inlineSize`](https://developer.mozilla.org/en-US/docs/Web/API/FragmentResult/inlineSize)
    pub fn inline_size(&self) -> f64 {
        self.inner.get("inlineSize").as_::<f64>()
    }
}
impl FragmentResult {
    /// Getter of the `blockSize` attribute.
    /// [`FragmentResult.blockSize`](https://developer.mozilla.org/en-US/docs/Web/API/FragmentResult/blockSize)
    pub fn block_size(&self) -> f64 {
        self.inner.get("blockSize").as_::<f64>()
    }
}

impl FragmentResult {
    /// The `new FragmentResult(..)` constructor, creating a new FragmentResult instance
    pub fn new() -> FragmentResult {
        Self {
            inner: Any::global("FragmentResult").new(&[]).as_::<Any>(),
        }
    }
}

impl FragmentResult {
    /// The `new FragmentResult(..)` constructor, creating a new FragmentResult instance
    pub fn new_with_options(options: &FragmentResultOptions) -> FragmentResult {
        Self {
            inner: Any::global("FragmentResult")
                .new(&[options.into()])
                .as_::<Any>(),
        }
    }
}
