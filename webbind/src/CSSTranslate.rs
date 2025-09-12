use super::*;

/// The CSSTranslate class.
/// [`CSSTranslate`](https://developer.mozilla.org/en-US/docs/Web/API/CSSTranslate)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSTranslate {
    inner: CSSTransformComponent,
}

impl FromVal for CSSTranslate {
    fn from_val(v: &Any) -> Self {
        CSSTranslate {
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

impl core::ops::Deref for CSSTranslate {
    type Target = CSSTransformComponent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSTranslate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSTranslate {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSTranslate {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CSSTranslate> for Any {
    fn from(s: CSSTranslate) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSTranslate> for Any {
    fn from(s: &CSSTranslate) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSTranslate);

impl CSSTranslate {
    /// Getter of the `x` attribute.
    /// [`CSSTranslate.x`](https://developer.mozilla.org/en-US/docs/Web/API/CSSTranslate/x)
    pub fn x(&self) -> CSSNumericValue {
        self.inner.get("x").as_::<CSSNumericValue>()
    }

    /// Setter of the `x` attribute.
    /// [`CSSTranslate.x`](https://developer.mozilla.org/en-US/docs/Web/API/CSSTranslate/x)
    pub fn set_x(&mut self, value: &CSSNumericValue) {
        self.inner.set("x", value);
    }
}
impl CSSTranslate {
    /// Getter of the `y` attribute.
    /// [`CSSTranslate.y`](https://developer.mozilla.org/en-US/docs/Web/API/CSSTranslate/y)
    pub fn y(&self) -> CSSNumericValue {
        self.inner.get("y").as_::<CSSNumericValue>()
    }

    /// Setter of the `y` attribute.
    /// [`CSSTranslate.y`](https://developer.mozilla.org/en-US/docs/Web/API/CSSTranslate/y)
    pub fn set_y(&mut self, value: &CSSNumericValue) {
        self.inner.set("y", value);
    }
}
impl CSSTranslate {
    /// Getter of the `z` attribute.
    /// [`CSSTranslate.z`](https://developer.mozilla.org/en-US/docs/Web/API/CSSTranslate/z)
    pub fn z(&self) -> CSSNumericValue {
        self.inner.get("z").as_::<CSSNumericValue>()
    }

    /// Setter of the `z` attribute.
    /// [`CSSTranslate.z`](https://developer.mozilla.org/en-US/docs/Web/API/CSSTranslate/z)
    pub fn set_z(&mut self, value: &CSSNumericValue) {
        self.inner.set("z", value);
    }
}

impl CSSTranslate {
    /// The `new CSSTranslate(..)` constructor, creating a new CSSTranslate instance
    pub fn new0(x: &CSSNumericValue, y: &CSSNumericValue) -> CSSTranslate {
        Self {
            inner: Any::global("CSSTranslate")
                .new(&[x.into(), y.into()])
                .as_::<CSSTransformComponent>(),
        }
    }

    /// The `new CSSTranslate(..)` constructor, creating a new CSSTranslate instance
    pub fn new1(x: &CSSNumericValue, y: &CSSNumericValue, z: &CSSNumericValue) -> CSSTranslate {
        Self {
            inner: Any::global("CSSTranslate")
                .new(&[x.into(), y.into(), z.into()])
                .as_::<CSSTransformComponent>(),
        }
    }
}
