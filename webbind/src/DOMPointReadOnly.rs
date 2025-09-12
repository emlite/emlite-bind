use super::*;

/// The DOMPointReadOnly class.
/// [`DOMPointReadOnly`](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DOMPointReadOnly {
    inner: Any,
}

impl FromVal for DOMPointReadOnly {
    fn from_val(v: &Any) -> Self {
        DOMPointReadOnly {
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

impl core::ops::Deref for DOMPointReadOnly {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DOMPointReadOnly {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DOMPointReadOnly {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DOMPointReadOnly {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<DOMPointReadOnly> for Any {
    fn from(s: DOMPointReadOnly) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DOMPointReadOnly> for Any {
    fn from(s: &DOMPointReadOnly) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(DOMPointReadOnly);

impl DOMPointReadOnly {
    /// Getter of the `x` attribute.
    /// [`DOMPointReadOnly.x`](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/x)
    pub fn x(&self) -> f64 {
        self.inner.get("x").as_::<f64>()
    }
}
impl DOMPointReadOnly {
    /// Getter of the `y` attribute.
    /// [`DOMPointReadOnly.y`](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/y)
    pub fn y(&self) -> f64 {
        self.inner.get("y").as_::<f64>()
    }
}
impl DOMPointReadOnly {
    /// Getter of the `z` attribute.
    /// [`DOMPointReadOnly.z`](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/z)
    pub fn z(&self) -> f64 {
        self.inner.get("z").as_::<f64>()
    }
}
impl DOMPointReadOnly {
    /// Getter of the `w` attribute.
    /// [`DOMPointReadOnly.w`](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/w)
    pub fn w(&self) -> f64 {
        self.inner.get("w").as_::<f64>()
    }
}

impl DOMPointReadOnly {
    /// The `new DOMPointReadOnly(..)` constructor, creating a new DOMPointReadOnly instance
    pub fn new0() -> DOMPointReadOnly {
        Self {
            inner: Any::global("DOMPointReadOnly").new(&[]).as_::<Any>(),
        }
    }

    /// The `new DOMPointReadOnly(..)` constructor, creating a new DOMPointReadOnly instance
    pub fn new1(x: f64) -> DOMPointReadOnly {
        Self {
            inner: Any::global("DOMPointReadOnly")
                .new(&[x.into()])
                .as_::<Any>(),
        }
    }

    /// The `new DOMPointReadOnly(..)` constructor, creating a new DOMPointReadOnly instance
    pub fn new2(x: f64, y: f64) -> DOMPointReadOnly {
        Self {
            inner: Any::global("DOMPointReadOnly")
                .new(&[x.into(), y.into()])
                .as_::<Any>(),
        }
    }

    /// The `new DOMPointReadOnly(..)` constructor, creating a new DOMPointReadOnly instance
    pub fn new3(x: f64, y: f64, z: f64) -> DOMPointReadOnly {
        Self {
            inner: Any::global("DOMPointReadOnly")
                .new(&[x.into(), y.into(), z.into()])
                .as_::<Any>(),
        }
    }

    /// The `new DOMPointReadOnly(..)` constructor, creating a new DOMPointReadOnly instance
    pub fn new4(x: f64, y: f64, z: f64, w: f64) -> DOMPointReadOnly {
        Self {
            inner: Any::global("DOMPointReadOnly")
                .new(&[x.into(), y.into(), z.into(), w.into()])
                .as_::<Any>(),
        }
    }
}
impl DOMPointReadOnly {
    /// The fromPoint method.
    /// [`DOMPointReadOnly.fromPoint`](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/fromPoint)
    pub fn from_point0() -> DOMPointReadOnly {
        Any::global("DOMPointReadOnly")
            .call("fromPoint", &[])
            .as_::<DOMPointReadOnly>()
    }
    /// The fromPoint method.
    /// [`DOMPointReadOnly.fromPoint`](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/fromPoint)
    pub fn from_point1(other: &DOMPointInit) -> DOMPointReadOnly {
        Any::global("DOMPointReadOnly")
            .call("fromPoint", &[other.into()])
            .as_::<DOMPointReadOnly>()
    }
}
impl DOMPointReadOnly {
    /// The matrixTransform method.
    /// [`DOMPointReadOnly.matrixTransform`](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/matrixTransform)
    pub fn matrix_transform0(&self) -> DOMPoint {
        self.inner.call("matrixTransform", &[]).as_::<DOMPoint>()
    }
    /// The matrixTransform method.
    /// [`DOMPointReadOnly.matrixTransform`](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/matrixTransform)
    pub fn matrix_transform1(&self, matrix: &DOMMatrixInit) -> DOMPoint {
        self.inner
            .call("matrixTransform", &[matrix.into()])
            .as_::<DOMPoint>()
    }
}
impl DOMPointReadOnly {
    /// The toJSON method.
    /// [`DOMPointReadOnly.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/toJSON)
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
