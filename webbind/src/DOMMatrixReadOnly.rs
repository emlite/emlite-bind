use super::*;

/// The DOMMatrixReadOnly class.
/// [`DOMMatrixReadOnly`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DOMMatrixReadOnly {
    inner: Any,
}

impl FromVal for DOMMatrixReadOnly {
    fn from_val(v: &Any) -> Self {
        DOMMatrixReadOnly {
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

impl core::ops::Deref for DOMMatrixReadOnly {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DOMMatrixReadOnly {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DOMMatrixReadOnly {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DOMMatrixReadOnly {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<DOMMatrixReadOnly> for Any {
    fn from(s: DOMMatrixReadOnly) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DOMMatrixReadOnly> for Any {
    fn from(s: &DOMMatrixReadOnly) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(DOMMatrixReadOnly);

impl DOMMatrixReadOnly {
    /// The `new DOMMatrixReadOnly(..)` constructor, creating a new DOMMatrixReadOnly instance
    pub fn new0() -> DOMMatrixReadOnly {
        Self {
            inner: Any::global("DOMMatrixReadOnly").new(&[]).as_::<Any>(),
        }
    }

    /// The `new DOMMatrixReadOnly(..)` constructor, creating a new DOMMatrixReadOnly instance
    pub fn new1(init: &Any) -> DOMMatrixReadOnly {
        Self {
            inner: Any::global("DOMMatrixReadOnly")
                .new(&[init.into()])
                .as_::<Any>(),
        }
    }
}
impl DOMMatrixReadOnly {
    /// The fromMatrix method.
    /// [`DOMMatrixReadOnly.fromMatrix`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/fromMatrix)
    pub fn from_matrix0() -> DOMMatrixReadOnly {
        Any::global("DOMMatrixReadOnly")
            .call("fromMatrix", &[])
            .as_::<DOMMatrixReadOnly>()
    }
    /// The fromMatrix method.
    /// [`DOMMatrixReadOnly.fromMatrix`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/fromMatrix)
    pub fn from_matrix1(other: &DOMMatrixInit) -> DOMMatrixReadOnly {
        Any::global("DOMMatrixReadOnly")
            .call("fromMatrix", &[other.into()])
            .as_::<DOMMatrixReadOnly>()
    }
}
impl DOMMatrixReadOnly {
    /// The fromFloat32Array method.
    /// [`DOMMatrixReadOnly.fromFloat32Array`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/fromFloat32Array)
    pub fn from_float32_array(array32: &Float32Array) -> DOMMatrixReadOnly {
        Any::global("DOMMatrixReadOnly")
            .call("fromFloat32Array", &[array32.into()])
            .as_::<DOMMatrixReadOnly>()
    }
}
impl DOMMatrixReadOnly {
    /// The fromFloat64Array method.
    /// [`DOMMatrixReadOnly.fromFloat64Array`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/fromFloat64Array)
    pub fn from_float64_array(array64: &Float64Array) -> DOMMatrixReadOnly {
        Any::global("DOMMatrixReadOnly")
            .call("fromFloat64Array", &[array64.into()])
            .as_::<DOMMatrixReadOnly>()
    }
}
impl DOMMatrixReadOnly {
    /// Getter of the `a` attribute.
    /// [`DOMMatrixReadOnly.a`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/a)
    pub fn a(&self) -> f64 {
        self.inner.get("a").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    /// Getter of the `b` attribute.
    /// [`DOMMatrixReadOnly.b`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/b)
    pub fn b(&self) -> f64 {
        self.inner.get("b").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    /// Getter of the `c` attribute.
    /// [`DOMMatrixReadOnly.c`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/c)
    pub fn c(&self) -> f64 {
        self.inner.get("c").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    /// Getter of the `d` attribute.
    /// [`DOMMatrixReadOnly.d`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/d)
    pub fn d(&self) -> f64 {
        self.inner.get("d").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    /// Getter of the `e` attribute.
    /// [`DOMMatrixReadOnly.e`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/e)
    pub fn e(&self) -> f64 {
        self.inner.get("e").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    /// Getter of the `f` attribute.
    /// [`DOMMatrixReadOnly.f`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/f)
    pub fn f(&self) -> f64 {
        self.inner.get("f").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    /// Getter of the `m11` attribute.
    /// [`DOMMatrixReadOnly.m11`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m11)
    pub fn m11(&self) -> f64 {
        self.inner.get("m11").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    /// Getter of the `m12` attribute.
    /// [`DOMMatrixReadOnly.m12`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m12)
    pub fn m12(&self) -> f64 {
        self.inner.get("m12").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    /// Getter of the `m13` attribute.
    /// [`DOMMatrixReadOnly.m13`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m13)
    pub fn m13(&self) -> f64 {
        self.inner.get("m13").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    /// Getter of the `m14` attribute.
    /// [`DOMMatrixReadOnly.m14`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m14)
    pub fn m14(&self) -> f64 {
        self.inner.get("m14").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    /// Getter of the `m21` attribute.
    /// [`DOMMatrixReadOnly.m21`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m21)
    pub fn m21(&self) -> f64 {
        self.inner.get("m21").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    /// Getter of the `m22` attribute.
    /// [`DOMMatrixReadOnly.m22`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m22)
    pub fn m22(&self) -> f64 {
        self.inner.get("m22").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    /// Getter of the `m23` attribute.
    /// [`DOMMatrixReadOnly.m23`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m23)
    pub fn m23(&self) -> f64 {
        self.inner.get("m23").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    /// Getter of the `m24` attribute.
    /// [`DOMMatrixReadOnly.m24`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m24)
    pub fn m24(&self) -> f64 {
        self.inner.get("m24").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    /// Getter of the `m31` attribute.
    /// [`DOMMatrixReadOnly.m31`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m31)
    pub fn m31(&self) -> f64 {
        self.inner.get("m31").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    /// Getter of the `m32` attribute.
    /// [`DOMMatrixReadOnly.m32`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m32)
    pub fn m32(&self) -> f64 {
        self.inner.get("m32").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    /// Getter of the `m33` attribute.
    /// [`DOMMatrixReadOnly.m33`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m33)
    pub fn m33(&self) -> f64 {
        self.inner.get("m33").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    /// Getter of the `m34` attribute.
    /// [`DOMMatrixReadOnly.m34`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m34)
    pub fn m34(&self) -> f64 {
        self.inner.get("m34").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    /// Getter of the `m41` attribute.
    /// [`DOMMatrixReadOnly.m41`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m41)
    pub fn m41(&self) -> f64 {
        self.inner.get("m41").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    /// Getter of the `m42` attribute.
    /// [`DOMMatrixReadOnly.m42`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m42)
    pub fn m42(&self) -> f64 {
        self.inner.get("m42").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    /// Getter of the `m43` attribute.
    /// [`DOMMatrixReadOnly.m43`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m43)
    pub fn m43(&self) -> f64 {
        self.inner.get("m43").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    /// Getter of the `m44` attribute.
    /// [`DOMMatrixReadOnly.m44`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/m44)
    pub fn m44(&self) -> f64 {
        self.inner.get("m44").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    /// Getter of the `is2D` attribute.
    /// [`DOMMatrixReadOnly.is2D`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/is2D)
    pub fn is2_d(&self) -> bool {
        self.inner.get("is2D").as_::<bool>()
    }
}
impl DOMMatrixReadOnly {
    /// Getter of the `isIdentity` attribute.
    /// [`DOMMatrixReadOnly.isIdentity`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/isIdentity)
    pub fn is_identity(&self) -> bool {
        self.inner.get("isIdentity").as_::<bool>()
    }
}
impl DOMMatrixReadOnly {
    /// The translate method.
    /// [`DOMMatrixReadOnly.translate`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/translate)
    pub fn translate0(&self) -> DOMMatrix {
        self.inner.call("translate", &[]).as_::<DOMMatrix>()
    }
    /// The translate method.
    /// [`DOMMatrixReadOnly.translate`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/translate)
    pub fn translate1(&self, tx: f64) -> DOMMatrix {
        self.inner
            .call("translate", &[tx.into()])
            .as_::<DOMMatrix>()
    }
    /// The translate method.
    /// [`DOMMatrixReadOnly.translate`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/translate)
    pub fn translate2(&self, tx: f64, ty: f64) -> DOMMatrix {
        self.inner
            .call("translate", &[tx.into(), ty.into()])
            .as_::<DOMMatrix>()
    }
    /// The translate method.
    /// [`DOMMatrixReadOnly.translate`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/translate)
    pub fn translate3(&self, tx: f64, ty: f64, tz: f64) -> DOMMatrix {
        self.inner
            .call("translate", &[tx.into(), ty.into(), tz.into()])
            .as_::<DOMMatrix>()
    }
}
impl DOMMatrixReadOnly {
    /// The scale method.
    /// [`DOMMatrixReadOnly.scale`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scale)
    pub fn scale0(&self) -> DOMMatrix {
        self.inner.call("scale", &[]).as_::<DOMMatrix>()
    }
    /// The scale method.
    /// [`DOMMatrixReadOnly.scale`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scale)
    pub fn scale1(&self, scale_x: f64) -> DOMMatrix {
        self.inner
            .call("scale", &[scale_x.into()])
            .as_::<DOMMatrix>()
    }
    /// The scale method.
    /// [`DOMMatrixReadOnly.scale`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scale)
    pub fn scale2(&self, scale_x: f64, scale_y: f64) -> DOMMatrix {
        self.inner
            .call("scale", &[scale_x.into(), scale_y.into()])
            .as_::<DOMMatrix>()
    }
    /// The scale method.
    /// [`DOMMatrixReadOnly.scale`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scale)
    pub fn scale3(&self, scale_x: f64, scale_y: f64, scale_z: f64) -> DOMMatrix {
        self.inner
            .call("scale", &[scale_x.into(), scale_y.into(), scale_z.into()])
            .as_::<DOMMatrix>()
    }
    /// The scale method.
    /// [`DOMMatrixReadOnly.scale`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scale)
    pub fn scale4(&self, scale_x: f64, scale_y: f64, scale_z: f64, origin_x: f64) -> DOMMatrix {
        self.inner
            .call(
                "scale",
                &[
                    scale_x.into(),
                    scale_y.into(),
                    scale_z.into(),
                    origin_x.into(),
                ],
            )
            .as_::<DOMMatrix>()
    }
    /// The scale method.
    /// [`DOMMatrixReadOnly.scale`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scale)
    pub fn scale5(
        &self,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
        origin_x: f64,
        origin_y: f64,
    ) -> DOMMatrix {
        self.inner
            .call(
                "scale",
                &[
                    scale_x.into(),
                    scale_y.into(),
                    scale_z.into(),
                    origin_x.into(),
                    origin_y.into(),
                ],
            )
            .as_::<DOMMatrix>()
    }
    /// The scale method.
    /// [`DOMMatrixReadOnly.scale`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scale)
    pub fn scale6(
        &self,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
        origin_x: f64,
        origin_y: f64,
        origin_z: f64,
    ) -> DOMMatrix {
        self.inner
            .call(
                "scale",
                &[
                    scale_x.into(),
                    scale_y.into(),
                    scale_z.into(),
                    origin_x.into(),
                    origin_y.into(),
                    origin_z.into(),
                ],
            )
            .as_::<DOMMatrix>()
    }
}
impl DOMMatrixReadOnly {
    /// The scaleNonUniform method.
    /// [`DOMMatrixReadOnly.scaleNonUniform`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scaleNonUniform)
    pub fn scale_non_uniform0(&self) -> DOMMatrix {
        self.inner.call("scaleNonUniform", &[]).as_::<DOMMatrix>()
    }
    /// The scaleNonUniform method.
    /// [`DOMMatrixReadOnly.scaleNonUniform`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scaleNonUniform)
    pub fn scale_non_uniform1(&self, scale_x: f64) -> DOMMatrix {
        self.inner
            .call("scaleNonUniform", &[scale_x.into()])
            .as_::<DOMMatrix>()
    }
    /// The scaleNonUniform method.
    /// [`DOMMatrixReadOnly.scaleNonUniform`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scaleNonUniform)
    pub fn scale_non_uniform2(&self, scale_x: f64, scale_y: f64) -> DOMMatrix {
        self.inner
            .call("scaleNonUniform", &[scale_x.into(), scale_y.into()])
            .as_::<DOMMatrix>()
    }
}
impl DOMMatrixReadOnly {
    /// The scale3d method.
    /// [`DOMMatrixReadOnly.scale3d`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scale3d)
    pub fn scale3d0(&self) -> DOMMatrix {
        self.inner.call("scale3d", &[]).as_::<DOMMatrix>()
    }
    /// The scale3d method.
    /// [`DOMMatrixReadOnly.scale3d`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scale3d)
    pub fn scale3d1(&self, scale: f64) -> DOMMatrix {
        self.inner
            .call("scale3d", &[scale.into()])
            .as_::<DOMMatrix>()
    }
    /// The scale3d method.
    /// [`DOMMatrixReadOnly.scale3d`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scale3d)
    pub fn scale3d2(&self, scale: f64, origin_x: f64) -> DOMMatrix {
        self.inner
            .call("scale3d", &[scale.into(), origin_x.into()])
            .as_::<DOMMatrix>()
    }
    /// The scale3d method.
    /// [`DOMMatrixReadOnly.scale3d`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scale3d)
    pub fn scale3d3(&self, scale: f64, origin_x: f64, origin_y: f64) -> DOMMatrix {
        self.inner
            .call("scale3d", &[scale.into(), origin_x.into(), origin_y.into()])
            .as_::<DOMMatrix>()
    }
    /// The scale3d method.
    /// [`DOMMatrixReadOnly.scale3d`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/scale3d)
    pub fn scale3d4(&self, scale: f64, origin_x: f64, origin_y: f64, origin_z: f64) -> DOMMatrix {
        self.inner
            .call(
                "scale3d",
                &[
                    scale.into(),
                    origin_x.into(),
                    origin_y.into(),
                    origin_z.into(),
                ],
            )
            .as_::<DOMMatrix>()
    }
}
impl DOMMatrixReadOnly {
    /// The rotate method.
    /// [`DOMMatrixReadOnly.rotate`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/rotate)
    pub fn rotate0(&self) -> DOMMatrix {
        self.inner.call("rotate", &[]).as_::<DOMMatrix>()
    }
    /// The rotate method.
    /// [`DOMMatrixReadOnly.rotate`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/rotate)
    pub fn rotate1(&self, rot_x: f64) -> DOMMatrix {
        self.inner
            .call("rotate", &[rot_x.into()])
            .as_::<DOMMatrix>()
    }
    /// The rotate method.
    /// [`DOMMatrixReadOnly.rotate`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/rotate)
    pub fn rotate2(&self, rot_x: f64, rot_y: f64) -> DOMMatrix {
        self.inner
            .call("rotate", &[rot_x.into(), rot_y.into()])
            .as_::<DOMMatrix>()
    }
    /// The rotate method.
    /// [`DOMMatrixReadOnly.rotate`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/rotate)
    pub fn rotate3(&self, rot_x: f64, rot_y: f64, rot_z: f64) -> DOMMatrix {
        self.inner
            .call("rotate", &[rot_x.into(), rot_y.into(), rot_z.into()])
            .as_::<DOMMatrix>()
    }
}
impl DOMMatrixReadOnly {
    /// The rotateFromVector method.
    /// [`DOMMatrixReadOnly.rotateFromVector`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/rotateFromVector)
    pub fn rotate_from_vector0(&self) -> DOMMatrix {
        self.inner.call("rotateFromVector", &[]).as_::<DOMMatrix>()
    }
    /// The rotateFromVector method.
    /// [`DOMMatrixReadOnly.rotateFromVector`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/rotateFromVector)
    pub fn rotate_from_vector1(&self, x: f64) -> DOMMatrix {
        self.inner
            .call("rotateFromVector", &[x.into()])
            .as_::<DOMMatrix>()
    }
    /// The rotateFromVector method.
    /// [`DOMMatrixReadOnly.rotateFromVector`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/rotateFromVector)
    pub fn rotate_from_vector2(&self, x: f64, y: f64) -> DOMMatrix {
        self.inner
            .call("rotateFromVector", &[x.into(), y.into()])
            .as_::<DOMMatrix>()
    }
}
impl DOMMatrixReadOnly {
    /// The rotateAxisAngle method.
    /// [`DOMMatrixReadOnly.rotateAxisAngle`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/rotateAxisAngle)
    pub fn rotate_axis_angle0(&self) -> DOMMatrix {
        self.inner.call("rotateAxisAngle", &[]).as_::<DOMMatrix>()
    }
    /// The rotateAxisAngle method.
    /// [`DOMMatrixReadOnly.rotateAxisAngle`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/rotateAxisAngle)
    pub fn rotate_axis_angle1(&self, x: f64) -> DOMMatrix {
        self.inner
            .call("rotateAxisAngle", &[x.into()])
            .as_::<DOMMatrix>()
    }
    /// The rotateAxisAngle method.
    /// [`DOMMatrixReadOnly.rotateAxisAngle`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/rotateAxisAngle)
    pub fn rotate_axis_angle2(&self, x: f64, y: f64) -> DOMMatrix {
        self.inner
            .call("rotateAxisAngle", &[x.into(), y.into()])
            .as_::<DOMMatrix>()
    }
    /// The rotateAxisAngle method.
    /// [`DOMMatrixReadOnly.rotateAxisAngle`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/rotateAxisAngle)
    pub fn rotate_axis_angle3(&self, x: f64, y: f64, z: f64) -> DOMMatrix {
        self.inner
            .call("rotateAxisAngle", &[x.into(), y.into(), z.into()])
            .as_::<DOMMatrix>()
    }
    /// The rotateAxisAngle method.
    /// [`DOMMatrixReadOnly.rotateAxisAngle`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/rotateAxisAngle)
    pub fn rotate_axis_angle4(&self, x: f64, y: f64, z: f64, angle: f64) -> DOMMatrix {
        self.inner
            .call(
                "rotateAxisAngle",
                &[x.into(), y.into(), z.into(), angle.into()],
            )
            .as_::<DOMMatrix>()
    }
}
impl DOMMatrixReadOnly {
    /// The skewX method.
    /// [`DOMMatrixReadOnly.skewX`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/skewX)
    pub fn skew_x0(&self) -> DOMMatrix {
        self.inner.call("skewX", &[]).as_::<DOMMatrix>()
    }
    /// The skewX method.
    /// [`DOMMatrixReadOnly.skewX`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/skewX)
    pub fn skew_x1(&self, sx: f64) -> DOMMatrix {
        self.inner.call("skewX", &[sx.into()]).as_::<DOMMatrix>()
    }
}
impl DOMMatrixReadOnly {
    /// The skewY method.
    /// [`DOMMatrixReadOnly.skewY`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/skewY)
    pub fn skew_y0(&self) -> DOMMatrix {
        self.inner.call("skewY", &[]).as_::<DOMMatrix>()
    }
    /// The skewY method.
    /// [`DOMMatrixReadOnly.skewY`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/skewY)
    pub fn skew_y1(&self, sy: f64) -> DOMMatrix {
        self.inner.call("skewY", &[sy.into()]).as_::<DOMMatrix>()
    }
}
impl DOMMatrixReadOnly {
    /// The multiply method.
    /// [`DOMMatrixReadOnly.multiply`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/multiply)
    pub fn multiply0(&self) -> DOMMatrix {
        self.inner.call("multiply", &[]).as_::<DOMMatrix>()
    }
    /// The multiply method.
    /// [`DOMMatrixReadOnly.multiply`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/multiply)
    pub fn multiply1(&self, other: &DOMMatrixInit) -> DOMMatrix {
        self.inner
            .call("multiply", &[other.into()])
            .as_::<DOMMatrix>()
    }
}
impl DOMMatrixReadOnly {
    /// The flipX method.
    /// [`DOMMatrixReadOnly.flipX`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/flipX)
    pub fn flip_x(&self) -> DOMMatrix {
        self.inner.call("flipX", &[]).as_::<DOMMatrix>()
    }
}
impl DOMMatrixReadOnly {
    /// The flipY method.
    /// [`DOMMatrixReadOnly.flipY`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/flipY)
    pub fn flip_y(&self) -> DOMMatrix {
        self.inner.call("flipY", &[]).as_::<DOMMatrix>()
    }
}
impl DOMMatrixReadOnly {
    /// The inverse method.
    /// [`DOMMatrixReadOnly.inverse`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/inverse)
    pub fn inverse(&self) -> DOMMatrix {
        self.inner.call("inverse", &[]).as_::<DOMMatrix>()
    }
}
impl DOMMatrixReadOnly {
    /// The transformPoint method.
    /// [`DOMMatrixReadOnly.transformPoint`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/transformPoint)
    pub fn transform_point0(&self) -> DOMPoint {
        self.inner.call("transformPoint", &[]).as_::<DOMPoint>()
    }
    /// The transformPoint method.
    /// [`DOMMatrixReadOnly.transformPoint`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/transformPoint)
    pub fn transform_point1(&self, point: &DOMPointInit) -> DOMPoint {
        self.inner
            .call("transformPoint", &[point.into()])
            .as_::<DOMPoint>()
    }
}
impl DOMMatrixReadOnly {
    /// The toFloat32Array method.
    /// [`DOMMatrixReadOnly.toFloat32Array`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/toFloat32Array)
    pub fn to_float32_array(&self) -> Float32Array {
        self.inner.call("toFloat32Array", &[]).as_::<Float32Array>()
    }
}
impl DOMMatrixReadOnly {
    /// The toFloat64Array method.
    /// [`DOMMatrixReadOnly.toFloat64Array`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/toFloat64Array)
    pub fn to_float64_array(&self) -> Float64Array {
        self.inner.call("toFloat64Array", &[]).as_::<Float64Array>()
    }
}
impl DOMMatrixReadOnly {
    /// The toJSON method.
    /// [`DOMMatrixReadOnly.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrixReadOnly/toJSON)
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
