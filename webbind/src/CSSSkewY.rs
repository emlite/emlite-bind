use super::*;

/// The CSSSkewY class.
/// [`CSSSkewY`](https://developer.mozilla.org/en-US/docs/Web/API/CSSSkewY)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSSkewY {
    inner: CSSTransformComponent,
}

impl FromVal for CSSSkewY {
    fn from_val(v: &Any) -> Self {
        CSSSkewY {
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

impl core::ops::Deref for CSSSkewY {
    type Target = CSSTransformComponent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSSkewY {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSSkewY {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSSkewY {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CSSSkewY> for Any {
    fn from(s: CSSSkewY) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSSkewY> for Any {
    fn from(s: &CSSSkewY) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSSkewY);

impl CSSSkewY {
    /// Getter of the `ay` attribute.
    /// [`CSSSkewY.ay`](https://developer.mozilla.org/en-US/docs/Web/API/CSSSkewY/ay)
    pub fn ay(&self) -> CSSNumericValue {
        self.inner.get("ay").as_::<CSSNumericValue>()
    }

    /// Setter of the `ay` attribute.
    /// [`CSSSkewY.ay`](https://developer.mozilla.org/en-US/docs/Web/API/CSSSkewY/ay)
    pub fn set_ay(&mut self, value: &CSSNumericValue) {
        self.inner.set("ay", value);
    }
}

impl CSSSkewY {
    /// The `new CSSSkewY(..)` constructor, creating a new CSSSkewY instance
    pub fn new(ay: &CSSNumericValue) -> CSSSkewY {
        Self {
            inner: Any::global("CSSSkewY")
                .new(&[ay.into()])
                .as_::<CSSTransformComponent>(),
        }
    }
}
