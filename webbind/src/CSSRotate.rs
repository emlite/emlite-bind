use super::*;

/// The CSSRotate class.
/// [`CSSRotate`](https://developer.mozilla.org/en-US/docs/Web/API/CSSRotate)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSRotate {
    inner: CSSTransformComponent,
}

impl FromVal for CSSRotate {
    fn from_val(v: &Any) -> Self {
        CSSRotate {
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

impl core::ops::Deref for CSSRotate {
    type Target = CSSTransformComponent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSRotate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSRotate {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSRotate {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CSSRotate> for Any {
    fn from(s: CSSRotate) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSRotate> for Any {
    fn from(s: &CSSRotate) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSRotate);

impl CSSRotate {
    /// Getter of the `x` attribute.
    /// [`CSSRotate.x`](https://developer.mozilla.org/en-US/docs/Web/API/CSSRotate/x)
    pub fn x(&self) -> Any {
        self.inner.get("x").as_::<Any>()
    }

    /// Setter of the `x` attribute.
    /// [`CSSRotate.x`](https://developer.mozilla.org/en-US/docs/Web/API/CSSRotate/x)
    pub fn set_x(&mut self, value: &Any) {
        self.inner.set("x", value);
    }
}
impl CSSRotate {
    /// Getter of the `y` attribute.
    /// [`CSSRotate.y`](https://developer.mozilla.org/en-US/docs/Web/API/CSSRotate/y)
    pub fn y(&self) -> Any {
        self.inner.get("y").as_::<Any>()
    }

    /// Setter of the `y` attribute.
    /// [`CSSRotate.y`](https://developer.mozilla.org/en-US/docs/Web/API/CSSRotate/y)
    pub fn set_y(&mut self, value: &Any) {
        self.inner.set("y", value);
    }
}
impl CSSRotate {
    /// Getter of the `z` attribute.
    /// [`CSSRotate.z`](https://developer.mozilla.org/en-US/docs/Web/API/CSSRotate/z)
    pub fn z(&self) -> Any {
        self.inner.get("z").as_::<Any>()
    }

    /// Setter of the `z` attribute.
    /// [`CSSRotate.z`](https://developer.mozilla.org/en-US/docs/Web/API/CSSRotate/z)
    pub fn set_z(&mut self, value: &Any) {
        self.inner.set("z", value);
    }
}
impl CSSRotate {
    /// Getter of the `angle` attribute.
    /// [`CSSRotate.angle`](https://developer.mozilla.org/en-US/docs/Web/API/CSSRotate/angle)
    pub fn angle(&self) -> CSSNumericValue {
        self.inner.get("angle").as_::<CSSNumericValue>()
    }

    /// Setter of the `angle` attribute.
    /// [`CSSRotate.angle`](https://developer.mozilla.org/en-US/docs/Web/API/CSSRotate/angle)
    pub fn set_angle(&mut self, value: &CSSNumericValue) {
        self.inner.set("angle", value);
    }
}

impl CSSRotate {
    /// The `new CSSRotate(..)` constructor, creating a new CSSRotate instance
    pub fn new(angle: &CSSNumericValue) -> CSSRotate {
        Self {
            inner: Any::global("CSSRotate")
                .new(&[angle.into()])
                .as_::<CSSTransformComponent>(),
        }
    }
}

impl CSSRotate {
    /// The `new CSSRotate(..)` constructor, creating a new CSSRotate instance
    pub fn new_with_x_and_y_and_z_and_angle(
        x: &Any,
        y: &Any,
        z: &Any,
        angle: &CSSNumericValue,
    ) -> CSSRotate {
        Self {
            inner: Any::global("CSSRotate")
                .new(&[x.into(), y.into(), z.into(), angle.into()])
                .as_::<CSSTransformComponent>(),
        }
    }
}
