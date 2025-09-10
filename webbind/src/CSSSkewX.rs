use super::*;

/// The CSSSkewX class.
/// [`CSSSkewX`](https://developer.mozilla.org/en-US/docs/Web/API/CSSSkewX)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSSkewX {
    inner: CSSTransformComponent,
}

impl FromVal for CSSSkewX {
    fn from_val(v: &Any) -> Self {
        CSSSkewX {
            inner: CSSTransformComponent::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CSSSkewX {
    type Target = CSSTransformComponent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSSkewX {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSSkewX {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSSkewX {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CSSSkewX> for Any {
    fn from(s: CSSSkewX) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSSkewX> for Any {
    fn from(s: &CSSSkewX) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSSkewX);

impl CSSSkewX {
    /// The `new CSSSkewX(..)` constructor, creating a new CSSSkewX instance
    pub fn new(ax: &CSSNumericValue) -> CSSSkewX {
        Self {
            inner: Any::global("CSSSkewX")
                .new(&[ax.into()])
                .as_::<CSSTransformComponent>(),
        }
    }
}
impl CSSSkewX {
    /// Getter of the `ax` attribute.
    /// [`CSSSkewX.ax`](https://developer.mozilla.org/en-US/docs/Web/API/CSSSkewX/ax)
    pub fn ax(&self) -> CSSNumericValue {
        self.inner.get("ax").as_::<CSSNumericValue>()
    }

    /// Setter of the `ax` attribute.
    /// [`CSSSkewX.ax`](https://developer.mozilla.org/en-US/docs/Web/API/CSSSkewX/ax)
    pub fn set_ax(&mut self, value: &CSSNumericValue) {
        self.inner.set("ax", value);
    }
}
