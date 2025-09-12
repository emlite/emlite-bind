use super::*;

/// The DOMPoint class.
/// [`DOMPoint`](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DOMPoint {
    inner: DOMPointReadOnly,
}

impl FromVal for DOMPoint {
    fn from_val(v: &Any) -> Self {
        DOMPoint {
            inner: DOMPointReadOnly::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DOMPoint {
    type Target = DOMPointReadOnly;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DOMPoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DOMPoint {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DOMPoint {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<DOMPoint> for Any {
    fn from(s: DOMPoint) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DOMPoint> for Any {
    fn from(s: &DOMPoint) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(DOMPoint);

impl DOMPoint {
    /// Getter of the `x` attribute.
    /// [`DOMPoint.x`](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/x)
    pub fn x(&self) -> f64 {
        self.inner.get("x").as_::<f64>()
    }

    /// Setter of the `x` attribute.
    /// [`DOMPoint.x`](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/x)
    pub fn set_x(&mut self, value: f64) {
        self.inner.set("x", value);
    }
}
impl DOMPoint {
    /// Getter of the `y` attribute.
    /// [`DOMPoint.y`](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/y)
    pub fn y(&self) -> f64 {
        self.inner.get("y").as_::<f64>()
    }

    /// Setter of the `y` attribute.
    /// [`DOMPoint.y`](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/y)
    pub fn set_y(&mut self, value: f64) {
        self.inner.set("y", value);
    }
}
impl DOMPoint {
    /// Getter of the `z` attribute.
    /// [`DOMPoint.z`](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/z)
    pub fn z(&self) -> f64 {
        self.inner.get("z").as_::<f64>()
    }

    /// Setter of the `z` attribute.
    /// [`DOMPoint.z`](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/z)
    pub fn set_z(&mut self, value: f64) {
        self.inner.set("z", value);
    }
}
impl DOMPoint {
    /// Getter of the `w` attribute.
    /// [`DOMPoint.w`](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/w)
    pub fn w(&self) -> f64 {
        self.inner.get("w").as_::<f64>()
    }

    /// Setter of the `w` attribute.
    /// [`DOMPoint.w`](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/w)
    pub fn set_w(&mut self, value: f64) {
        self.inner.set("w", value);
    }
}

impl DOMPoint {
    /// The `new DOMPoint(..)` constructor, creating a new DOMPoint instance
    pub fn new() -> DOMPoint {
        Self {
            inner: Any::global("DOMPoint").new(&[]).as_::<DOMPointReadOnly>(),
        }
    }
}

impl DOMPoint {
    /// The `new DOMPoint(..)` constructor, creating a new DOMPoint instance
    pub fn new_with_x(x: f64) -> DOMPoint {
        Self {
            inner: Any::global("DOMPoint")
                .new(&[x.into()])
                .as_::<DOMPointReadOnly>(),
        }
    }
}

impl DOMPoint {
    /// The `new DOMPoint(..)` constructor, creating a new DOMPoint instance
    pub fn new_with_x_and_y(x: f64, y: f64) -> DOMPoint {
        Self {
            inner: Any::global("DOMPoint")
                .new(&[x.into(), y.into()])
                .as_::<DOMPointReadOnly>(),
        }
    }
}

impl DOMPoint {
    /// The `new DOMPoint(..)` constructor, creating a new DOMPoint instance
    pub fn new_with_x_and_y_and_z(x: f64, y: f64, z: f64) -> DOMPoint {
        Self {
            inner: Any::global("DOMPoint")
                .new(&[x.into(), y.into(), z.into()])
                .as_::<DOMPointReadOnly>(),
        }
    }
}

impl DOMPoint {
    /// The `new DOMPoint(..)` constructor, creating a new DOMPoint instance
    pub fn new_with_x_and_y_and_z_and_w(x: f64, y: f64, z: f64, w: f64) -> DOMPoint {
        Self {
            inner: Any::global("DOMPoint")
                .new(&[x.into(), y.into(), z.into(), w.into()])
                .as_::<DOMPointReadOnly>(),
        }
    }
}

impl DOMPoint {
    /// The fromPoint method.
    /// [`DOMPoint.fromPoint`](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/fromPoint)
    pub fn from_point() -> DOMPoint {
        Any::global("DOMPoint")
            .call("fromPoint", &[])
            .as_::<DOMPoint>()
    }
}
impl DOMPoint {
    /// The fromPoint method.
    /// [`DOMPoint.fromPoint`](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/fromPoint)
    pub fn from_point_with_other(other: &DOMPointInit) -> DOMPoint {
        Any::global("DOMPoint")
            .call("fromPoint", &[other.into()])
            .as_::<DOMPoint>()
    }
}
