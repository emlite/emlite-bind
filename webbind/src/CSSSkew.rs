use super::*;

/// The CSSSkew class.
/// [`CSSSkew`](https://developer.mozilla.org/en-US/docs/Web/API/CSSSkew)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSSkew {
    inner: CSSTransformComponent,
}

impl FromVal for CSSSkew {
    fn from_val(v: &Any) -> Self {
        CSSSkew {
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

impl core::ops::Deref for CSSSkew {
    type Target = CSSTransformComponent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSSkew {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSSkew {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSSkew {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CSSSkew> for Any {
    fn from(s: CSSSkew) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSSkew> for Any {
    fn from(s: &CSSSkew) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSSkew);

impl CSSSkew {
    /// Getter of the `ax` attribute.
    /// [`CSSSkew.ax`](https://developer.mozilla.org/en-US/docs/Web/API/CSSSkew/ax)
    pub fn ax(&self) -> CSSNumericValue {
        self.inner.get("ax").as_::<CSSNumericValue>()
    }

    /// Setter of the `ax` attribute.
    /// [`CSSSkew.ax`](https://developer.mozilla.org/en-US/docs/Web/API/CSSSkew/ax)
    pub fn set_ax(&mut self, value: &CSSNumericValue) {
        self.inner.set("ax", value);
    }
}
impl CSSSkew {
    /// Getter of the `ay` attribute.
    /// [`CSSSkew.ay`](https://developer.mozilla.org/en-US/docs/Web/API/CSSSkew/ay)
    pub fn ay(&self) -> CSSNumericValue {
        self.inner.get("ay").as_::<CSSNumericValue>()
    }

    /// Setter of the `ay` attribute.
    /// [`CSSSkew.ay`](https://developer.mozilla.org/en-US/docs/Web/API/CSSSkew/ay)
    pub fn set_ay(&mut self, value: &CSSNumericValue) {
        self.inner.set("ay", value);
    }
}

impl CSSSkew {
    /// The `new CSSSkew(..)` constructor, creating a new CSSSkew instance
    pub fn new(ax: &CSSNumericValue, ay: &CSSNumericValue) -> CSSSkew {
        Self {
            inner: Any::global("CSSSkew")
                .new(&[ax.into(), ay.into()])
                .as_::<CSSTransformComponent>(),
        }
    }
}
