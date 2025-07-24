use super::*;

/// The DOMMatrix class.
/// [`DOMMatrix`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DOMMatrix {
    inner: DOMMatrixReadOnly,
}
impl FromVal for DOMMatrix {
    fn from_val(v: &Any) -> Self {
        DOMMatrix {
            inner: DOMMatrixReadOnly::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for DOMMatrix {
    type Target = DOMMatrixReadOnly;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DOMMatrix {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for DOMMatrix {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for DOMMatrix {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<DOMMatrix> for Any {
    fn from(s: DOMMatrix) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&DOMMatrix> for Any {
    fn from(s: &DOMMatrix) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(DOMMatrix);

impl DOMMatrix {
    /// The `new DOMMatrix(..)` constructor, creating a new DOMMatrix instance
    pub fn new0() -> DOMMatrix {
        Self {
            inner: Any::global("DOMMatrix").new(&[]).as_::<DOMMatrixReadOnly>(),
        }
    }

    /// The `new DOMMatrix(..)` constructor, creating a new DOMMatrix instance
    pub fn new1(init: &Any) -> DOMMatrix {
        Self {
            inner: Any::global("DOMMatrix")
                .new(&[init.into()])
                .as_::<DOMMatrixReadOnly>(),
        }
    }
}
impl DOMMatrix {
    /// The fromMatrix method.
    /// [`DOMMatrix.fromMatrix`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/fromMatrix)
    pub fn from_matrix0() -> DOMMatrix {
        Any::global("DOMMatrix")
            .call("fromMatrix", &[])
            .as_::<DOMMatrix>()
    }
    /// The fromMatrix method.
    /// [`DOMMatrix.fromMatrix`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/fromMatrix)
    pub fn from_matrix1(other: &DOMMatrixInit) -> DOMMatrix {
        Any::global("DOMMatrix")
            .call("fromMatrix", &[other.into()])
            .as_::<DOMMatrix>()
    }
}
impl DOMMatrix {
    /// The fromFloat32Array method.
    /// [`DOMMatrix.fromFloat32Array`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/fromFloat32Array)
    pub fn from_float32_array(array32: &Float32Array) -> DOMMatrix {
        Any::global("DOMMatrix")
            .call("fromFloat32Array", &[array32.into()])
            .as_::<DOMMatrix>()
    }
}
impl DOMMatrix {
    /// The fromFloat64Array method.
    /// [`DOMMatrix.fromFloat64Array`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/fromFloat64Array)
    pub fn from_float64_array(array64: &Float64Array) -> DOMMatrix {
        Any::global("DOMMatrix")
            .call("fromFloat64Array", &[array64.into()])
            .as_::<DOMMatrix>()
    }
}
impl DOMMatrix {
    /// Getter of the `a` attribute.
    /// [`DOMMatrix.a`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/a)
    pub fn a(&self) -> f64 {
        self.inner.get("a").as_::<f64>()
    }

    /// Setter of the `a` attribute.
    /// [`DOMMatrix.a`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/a)
    pub fn set_a(&mut self, value: f64) {
        self.inner.set("a", value);
    }
}
impl DOMMatrix {
    /// Getter of the `b` attribute.
    /// [`DOMMatrix.b`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/b)
    pub fn b(&self) -> f64 {
        self.inner.get("b").as_::<f64>()
    }

    /// Setter of the `b` attribute.
    /// [`DOMMatrix.b`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/b)
    pub fn set_b(&mut self, value: f64) {
        self.inner.set("b", value);
    }
}
impl DOMMatrix {
    /// Getter of the `c` attribute.
    /// [`DOMMatrix.c`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/c)
    pub fn c(&self) -> f64 {
        self.inner.get("c").as_::<f64>()
    }

    /// Setter of the `c` attribute.
    /// [`DOMMatrix.c`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/c)
    pub fn set_c(&mut self, value: f64) {
        self.inner.set("c", value);
    }
}
impl DOMMatrix {
    /// Getter of the `d` attribute.
    /// [`DOMMatrix.d`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/d)
    pub fn d(&self) -> f64 {
        self.inner.get("d").as_::<f64>()
    }

    /// Setter of the `d` attribute.
    /// [`DOMMatrix.d`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/d)
    pub fn set_d(&mut self, value: f64) {
        self.inner.set("d", value);
    }
}
impl DOMMatrix {
    /// Getter of the `e` attribute.
    /// [`DOMMatrix.e`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/e)
    pub fn e(&self) -> f64 {
        self.inner.get("e").as_::<f64>()
    }

    /// Setter of the `e` attribute.
    /// [`DOMMatrix.e`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/e)
    pub fn set_e(&mut self, value: f64) {
        self.inner.set("e", value);
    }
}
impl DOMMatrix {
    /// Getter of the `f` attribute.
    /// [`DOMMatrix.f`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/f)
    pub fn f(&self) -> f64 {
        self.inner.get("f").as_::<f64>()
    }

    /// Setter of the `f` attribute.
    /// [`DOMMatrix.f`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/f)
    pub fn set_f(&mut self, value: f64) {
        self.inner.set("f", value);
    }
}
impl DOMMatrix {
    /// Getter of the `m11` attribute.
    /// [`DOMMatrix.m11`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m11)
    pub fn m11(&self) -> f64 {
        self.inner.get("m11").as_::<f64>()
    }

    /// Setter of the `m11` attribute.
    /// [`DOMMatrix.m11`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m11)
    pub fn set_m11(&mut self, value: f64) {
        self.inner.set("m11", value);
    }
}
impl DOMMatrix {
    /// Getter of the `m12` attribute.
    /// [`DOMMatrix.m12`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m12)
    pub fn m12(&self) -> f64 {
        self.inner.get("m12").as_::<f64>()
    }

    /// Setter of the `m12` attribute.
    /// [`DOMMatrix.m12`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m12)
    pub fn set_m12(&mut self, value: f64) {
        self.inner.set("m12", value);
    }
}
impl DOMMatrix {
    /// Getter of the `m13` attribute.
    /// [`DOMMatrix.m13`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m13)
    pub fn m13(&self) -> f64 {
        self.inner.get("m13").as_::<f64>()
    }

    /// Setter of the `m13` attribute.
    /// [`DOMMatrix.m13`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m13)
    pub fn set_m13(&mut self, value: f64) {
        self.inner.set("m13", value);
    }
}
impl DOMMatrix {
    /// Getter of the `m14` attribute.
    /// [`DOMMatrix.m14`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m14)
    pub fn m14(&self) -> f64 {
        self.inner.get("m14").as_::<f64>()
    }

    /// Setter of the `m14` attribute.
    /// [`DOMMatrix.m14`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m14)
    pub fn set_m14(&mut self, value: f64) {
        self.inner.set("m14", value);
    }
}
impl DOMMatrix {
    /// Getter of the `m21` attribute.
    /// [`DOMMatrix.m21`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m21)
    pub fn m21(&self) -> f64 {
        self.inner.get("m21").as_::<f64>()
    }

    /// Setter of the `m21` attribute.
    /// [`DOMMatrix.m21`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m21)
    pub fn set_m21(&mut self, value: f64) {
        self.inner.set("m21", value);
    }
}
impl DOMMatrix {
    /// Getter of the `m22` attribute.
    /// [`DOMMatrix.m22`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m22)
    pub fn m22(&self) -> f64 {
        self.inner.get("m22").as_::<f64>()
    }

    /// Setter of the `m22` attribute.
    /// [`DOMMatrix.m22`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m22)
    pub fn set_m22(&mut self, value: f64) {
        self.inner.set("m22", value);
    }
}
impl DOMMatrix {
    /// Getter of the `m23` attribute.
    /// [`DOMMatrix.m23`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m23)
    pub fn m23(&self) -> f64 {
        self.inner.get("m23").as_::<f64>()
    }

    /// Setter of the `m23` attribute.
    /// [`DOMMatrix.m23`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m23)
    pub fn set_m23(&mut self, value: f64) {
        self.inner.set("m23", value);
    }
}
impl DOMMatrix {
    /// Getter of the `m24` attribute.
    /// [`DOMMatrix.m24`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m24)
    pub fn m24(&self) -> f64 {
        self.inner.get("m24").as_::<f64>()
    }

    /// Setter of the `m24` attribute.
    /// [`DOMMatrix.m24`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m24)
    pub fn set_m24(&mut self, value: f64) {
        self.inner.set("m24", value);
    }
}
impl DOMMatrix {
    /// Getter of the `m31` attribute.
    /// [`DOMMatrix.m31`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m31)
    pub fn m31(&self) -> f64 {
        self.inner.get("m31").as_::<f64>()
    }

    /// Setter of the `m31` attribute.
    /// [`DOMMatrix.m31`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m31)
    pub fn set_m31(&mut self, value: f64) {
        self.inner.set("m31", value);
    }
}
impl DOMMatrix {
    /// Getter of the `m32` attribute.
    /// [`DOMMatrix.m32`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m32)
    pub fn m32(&self) -> f64 {
        self.inner.get("m32").as_::<f64>()
    }

    /// Setter of the `m32` attribute.
    /// [`DOMMatrix.m32`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m32)
    pub fn set_m32(&mut self, value: f64) {
        self.inner.set("m32", value);
    }
}
impl DOMMatrix {
    /// Getter of the `m33` attribute.
    /// [`DOMMatrix.m33`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m33)
    pub fn m33(&self) -> f64 {
        self.inner.get("m33").as_::<f64>()
    }

    /// Setter of the `m33` attribute.
    /// [`DOMMatrix.m33`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m33)
    pub fn set_m33(&mut self, value: f64) {
        self.inner.set("m33", value);
    }
}
impl DOMMatrix {
    /// Getter of the `m34` attribute.
    /// [`DOMMatrix.m34`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m34)
    pub fn m34(&self) -> f64 {
        self.inner.get("m34").as_::<f64>()
    }

    /// Setter of the `m34` attribute.
    /// [`DOMMatrix.m34`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m34)
    pub fn set_m34(&mut self, value: f64) {
        self.inner.set("m34", value);
    }
}
impl DOMMatrix {
    /// Getter of the `m41` attribute.
    /// [`DOMMatrix.m41`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m41)
    pub fn m41(&self) -> f64 {
        self.inner.get("m41").as_::<f64>()
    }

    /// Setter of the `m41` attribute.
    /// [`DOMMatrix.m41`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m41)
    pub fn set_m41(&mut self, value: f64) {
        self.inner.set("m41", value);
    }
}
impl DOMMatrix {
    /// Getter of the `m42` attribute.
    /// [`DOMMatrix.m42`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m42)
    pub fn m42(&self) -> f64 {
        self.inner.get("m42").as_::<f64>()
    }

    /// Setter of the `m42` attribute.
    /// [`DOMMatrix.m42`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m42)
    pub fn set_m42(&mut self, value: f64) {
        self.inner.set("m42", value);
    }
}
impl DOMMatrix {
    /// Getter of the `m43` attribute.
    /// [`DOMMatrix.m43`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m43)
    pub fn m43(&self) -> f64 {
        self.inner.get("m43").as_::<f64>()
    }

    /// Setter of the `m43` attribute.
    /// [`DOMMatrix.m43`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m43)
    pub fn set_m43(&mut self, value: f64) {
        self.inner.set("m43", value);
    }
}
impl DOMMatrix {
    /// Getter of the `m44` attribute.
    /// [`DOMMatrix.m44`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m44)
    pub fn m44(&self) -> f64 {
        self.inner.get("m44").as_::<f64>()
    }

    /// Setter of the `m44` attribute.
    /// [`DOMMatrix.m44`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/m44)
    pub fn set_m44(&mut self, value: f64) {
        self.inner.set("m44", value);
    }
}
impl DOMMatrix {
    /// The multiplySelf method.
    /// [`DOMMatrix.multiplySelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/multiplySelf)
    pub fn multiply_self0(&self) -> DOMMatrix {
        self.inner.call("multiplySelf", &[]).as_::<DOMMatrix>()
    }
    /// The multiplySelf method.
    /// [`DOMMatrix.multiplySelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/multiplySelf)
    pub fn multiply_self1(&self, other: &DOMMatrixInit) -> DOMMatrix {
        self.inner
            .call("multiplySelf", &[other.into()])
            .as_::<DOMMatrix>()
    }
}
impl DOMMatrix {
    /// The preMultiplySelf method.
    /// [`DOMMatrix.preMultiplySelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/preMultiplySelf)
    pub fn pre_multiply_self0(&self) -> DOMMatrix {
        self.inner.call("preMultiplySelf", &[]).as_::<DOMMatrix>()
    }
    /// The preMultiplySelf method.
    /// [`DOMMatrix.preMultiplySelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/preMultiplySelf)
    pub fn pre_multiply_self1(&self, other: &DOMMatrixInit) -> DOMMatrix {
        self.inner
            .call("preMultiplySelf", &[other.into()])
            .as_::<DOMMatrix>()
    }
}
impl DOMMatrix {
    /// The translateSelf method.
    /// [`DOMMatrix.translateSelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/translateSelf)
    pub fn translate_self0(&self) -> DOMMatrix {
        self.inner.call("translateSelf", &[]).as_::<DOMMatrix>()
    }
    /// The translateSelf method.
    /// [`DOMMatrix.translateSelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/translateSelf)
    pub fn translate_self1(&self, tx: f64) -> DOMMatrix {
        self.inner
            .call("translateSelf", &[tx.into()])
            .as_::<DOMMatrix>()
    }
    /// The translateSelf method.
    /// [`DOMMatrix.translateSelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/translateSelf)
    pub fn translate_self2(&self, tx: f64, ty: f64) -> DOMMatrix {
        self.inner
            .call("translateSelf", &[tx.into(), ty.into()])
            .as_::<DOMMatrix>()
    }
    /// The translateSelf method.
    /// [`DOMMatrix.translateSelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/translateSelf)
    pub fn translate_self3(&self, tx: f64, ty: f64, tz: f64) -> DOMMatrix {
        self.inner
            .call("translateSelf", &[tx.into(), ty.into(), tz.into()])
            .as_::<DOMMatrix>()
    }
}
impl DOMMatrix {
    /// The scaleSelf method.
    /// [`DOMMatrix.scaleSelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scaleSelf)
    pub fn scale_self0(&self) -> DOMMatrix {
        self.inner.call("scaleSelf", &[]).as_::<DOMMatrix>()
    }
    /// The scaleSelf method.
    /// [`DOMMatrix.scaleSelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scaleSelf)
    pub fn scale_self1(&self, scale_x: f64) -> DOMMatrix {
        self.inner
            .call("scaleSelf", &[scale_x.into()])
            .as_::<DOMMatrix>()
    }
    /// The scaleSelf method.
    /// [`DOMMatrix.scaleSelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scaleSelf)
    pub fn scale_self2(&self, scale_x: f64, scale_y: f64) -> DOMMatrix {
        self.inner
            .call("scaleSelf", &[scale_x.into(), scale_y.into()])
            .as_::<DOMMatrix>()
    }
    /// The scaleSelf method.
    /// [`DOMMatrix.scaleSelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scaleSelf)
    pub fn scale_self3(&self, scale_x: f64, scale_y: f64, scale_z: f64) -> DOMMatrix {
        self.inner
            .call(
                "scaleSelf",
                &[scale_x.into(), scale_y.into(), scale_z.into()],
            )
            .as_::<DOMMatrix>()
    }
    /// The scaleSelf method.
    /// [`DOMMatrix.scaleSelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scaleSelf)
    pub fn scale_self4(
        &self,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
        origin_x: f64,
    ) -> DOMMatrix {
        self.inner
            .call(
                "scaleSelf",
                &[
                    scale_x.into(),
                    scale_y.into(),
                    scale_z.into(),
                    origin_x.into(),
                ],
            )
            .as_::<DOMMatrix>()
    }
    /// The scaleSelf method.
    /// [`DOMMatrix.scaleSelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scaleSelf)
    pub fn scale_self5(
        &self,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
        origin_x: f64,
        origin_y: f64,
    ) -> DOMMatrix {
        self.inner
            .call(
                "scaleSelf",
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
    /// The scaleSelf method.
    /// [`DOMMatrix.scaleSelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scaleSelf)
    pub fn scale_self6(
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
                "scaleSelf",
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
impl DOMMatrix {
    /// The scale3dSelf method.
    /// [`DOMMatrix.scale3dSelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scale3dSelf)
    pub fn scale3d_self0(&self) -> DOMMatrix {
        self.inner.call("scale3dSelf", &[]).as_::<DOMMatrix>()
    }
    /// The scale3dSelf method.
    /// [`DOMMatrix.scale3dSelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scale3dSelf)
    pub fn scale3d_self1(&self, scale: f64) -> DOMMatrix {
        self.inner
            .call("scale3dSelf", &[scale.into()])
            .as_::<DOMMatrix>()
    }
    /// The scale3dSelf method.
    /// [`DOMMatrix.scale3dSelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scale3dSelf)
    pub fn scale3d_self2(&self, scale: f64, origin_x: f64) -> DOMMatrix {
        self.inner
            .call("scale3dSelf", &[scale.into(), origin_x.into()])
            .as_::<DOMMatrix>()
    }
    /// The scale3dSelf method.
    /// [`DOMMatrix.scale3dSelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scale3dSelf)
    pub fn scale3d_self3(&self, scale: f64, origin_x: f64, origin_y: f64) -> DOMMatrix {
        self.inner
            .call(
                "scale3dSelf",
                &[scale.into(), origin_x.into(), origin_y.into()],
            )
            .as_::<DOMMatrix>()
    }
    /// The scale3dSelf method.
    /// [`DOMMatrix.scale3dSelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/scale3dSelf)
    pub fn scale3d_self4(
        &self,
        scale: f64,
        origin_x: f64,
        origin_y: f64,
        origin_z: f64,
    ) -> DOMMatrix {
        self.inner
            .call(
                "scale3dSelf",
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
impl DOMMatrix {
    /// The rotateSelf method.
    /// [`DOMMatrix.rotateSelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/rotateSelf)
    pub fn rotate_self0(&self) -> DOMMatrix {
        self.inner.call("rotateSelf", &[]).as_::<DOMMatrix>()
    }
    /// The rotateSelf method.
    /// [`DOMMatrix.rotateSelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/rotateSelf)
    pub fn rotate_self1(&self, rot_x: f64) -> DOMMatrix {
        self.inner
            .call("rotateSelf", &[rot_x.into()])
            .as_::<DOMMatrix>()
    }
    /// The rotateSelf method.
    /// [`DOMMatrix.rotateSelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/rotateSelf)
    pub fn rotate_self2(&self, rot_x: f64, rot_y: f64) -> DOMMatrix {
        self.inner
            .call("rotateSelf", &[rot_x.into(), rot_y.into()])
            .as_::<DOMMatrix>()
    }
    /// The rotateSelf method.
    /// [`DOMMatrix.rotateSelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/rotateSelf)
    pub fn rotate_self3(&self, rot_x: f64, rot_y: f64, rot_z: f64) -> DOMMatrix {
        self.inner
            .call("rotateSelf", &[rot_x.into(), rot_y.into(), rot_z.into()])
            .as_::<DOMMatrix>()
    }
}
impl DOMMatrix {
    /// The rotateFromVectorSelf method.
    /// [`DOMMatrix.rotateFromVectorSelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/rotateFromVectorSelf)
    pub fn rotate_from_vector_self0(&self) -> DOMMatrix {
        self.inner
            .call("rotateFromVectorSelf", &[])
            .as_::<DOMMatrix>()
    }
    /// The rotateFromVectorSelf method.
    /// [`DOMMatrix.rotateFromVectorSelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/rotateFromVectorSelf)
    pub fn rotate_from_vector_self1(&self, x: f64) -> DOMMatrix {
        self.inner
            .call("rotateFromVectorSelf", &[x.into()])
            .as_::<DOMMatrix>()
    }
    /// The rotateFromVectorSelf method.
    /// [`DOMMatrix.rotateFromVectorSelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/rotateFromVectorSelf)
    pub fn rotate_from_vector_self2(&self, x: f64, y: f64) -> DOMMatrix {
        self.inner
            .call("rotateFromVectorSelf", &[x.into(), y.into()])
            .as_::<DOMMatrix>()
    }
}
impl DOMMatrix {
    /// The rotateAxisAngleSelf method.
    /// [`DOMMatrix.rotateAxisAngleSelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/rotateAxisAngleSelf)
    pub fn rotate_axis_angle_self0(&self) -> DOMMatrix {
        self.inner
            .call("rotateAxisAngleSelf", &[])
            .as_::<DOMMatrix>()
    }
    /// The rotateAxisAngleSelf method.
    /// [`DOMMatrix.rotateAxisAngleSelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/rotateAxisAngleSelf)
    pub fn rotate_axis_angle_self1(&self, x: f64) -> DOMMatrix {
        self.inner
            .call("rotateAxisAngleSelf", &[x.into()])
            .as_::<DOMMatrix>()
    }
    /// The rotateAxisAngleSelf method.
    /// [`DOMMatrix.rotateAxisAngleSelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/rotateAxisAngleSelf)
    pub fn rotate_axis_angle_self2(&self, x: f64, y: f64) -> DOMMatrix {
        self.inner
            .call("rotateAxisAngleSelf", &[x.into(), y.into()])
            .as_::<DOMMatrix>()
    }
    /// The rotateAxisAngleSelf method.
    /// [`DOMMatrix.rotateAxisAngleSelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/rotateAxisAngleSelf)
    pub fn rotate_axis_angle_self3(&self, x: f64, y: f64, z: f64) -> DOMMatrix {
        self.inner
            .call("rotateAxisAngleSelf", &[x.into(), y.into(), z.into()])
            .as_::<DOMMatrix>()
    }
    /// The rotateAxisAngleSelf method.
    /// [`DOMMatrix.rotateAxisAngleSelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/rotateAxisAngleSelf)
    pub fn rotate_axis_angle_self4(&self, x: f64, y: f64, z: f64, angle: f64) -> DOMMatrix {
        self.inner
            .call(
                "rotateAxisAngleSelf",
                &[x.into(), y.into(), z.into(), angle.into()],
            )
            .as_::<DOMMatrix>()
    }
}
impl DOMMatrix {
    /// The skewXSelf method.
    /// [`DOMMatrix.skewXSelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/skewXSelf)
    pub fn skew_x_self0(&self) -> DOMMatrix {
        self.inner.call("skewXSelf", &[]).as_::<DOMMatrix>()
    }
    /// The skewXSelf method.
    /// [`DOMMatrix.skewXSelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/skewXSelf)
    pub fn skew_x_self1(&self, sx: f64) -> DOMMatrix {
        self.inner
            .call("skewXSelf", &[sx.into()])
            .as_::<DOMMatrix>()
    }
}
impl DOMMatrix {
    /// The skewYSelf method.
    /// [`DOMMatrix.skewYSelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/skewYSelf)
    pub fn skew_y_self0(&self) -> DOMMatrix {
        self.inner.call("skewYSelf", &[]).as_::<DOMMatrix>()
    }
    /// The skewYSelf method.
    /// [`DOMMatrix.skewYSelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/skewYSelf)
    pub fn skew_y_self1(&self, sy: f64) -> DOMMatrix {
        self.inner
            .call("skewYSelf", &[sy.into()])
            .as_::<DOMMatrix>()
    }
}
impl DOMMatrix {
    /// The invertSelf method.
    /// [`DOMMatrix.invertSelf`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/invertSelf)
    pub fn invert_self(&self) -> DOMMatrix {
        self.inner.call("invertSelf", &[]).as_::<DOMMatrix>()
    }
}
impl DOMMatrix {
    /// The setMatrixValue method.
    /// [`DOMMatrix.setMatrixValue`](https://developer.mozilla.org/en-US/docs/Web/API/DOMMatrix/setMatrixValue)
    pub fn set_matrix_value(&self, transform_list: &DOMString) -> DOMMatrix {
        self.inner
            .call("setMatrixValue", &[transform_list.into()])
            .as_::<DOMMatrix>()
    }
}
