use super::*;

#[derive(Clone, Debug)]
pub struct DOMMatrixReadOnly {
    inner: emlite::Val,
}
impl FromVal for DOMMatrixReadOnly {
    fn from_val(v: &emlite::Val) -> Self {
        DOMMatrixReadOnly {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for DOMMatrixReadOnly {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for DOMMatrixReadOnly {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<DOMMatrixReadOnly> for emlite::Val {
    fn from(s: DOMMatrixReadOnly) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl DOMMatrixReadOnly {
    pub fn new0() -> DOMMatrixReadOnly {
        Self {
            inner: emlite::Val::global("DOMMatrixReadOnly")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(init: jsbind::Any) -> DOMMatrixReadOnly {
        Self {
            inner: emlite::Val::global("DOMMatrixReadOnly")
                .new(&[init.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl DOMMatrixReadOnly {
    pub fn from_matrix0() -> DOMMatrixReadOnly {
        emlite::Val::global("dommatrixreadonly")
            .call("fromMatrix", &[])
            .as_::<DOMMatrixReadOnly>()
    }

    pub fn from_matrix1(other: DOMMatrixInit) -> DOMMatrixReadOnly {
        emlite::Val::global("dommatrixreadonly")
            .call("fromMatrix", &[other.into()])
            .as_::<DOMMatrixReadOnly>()
    }
}
impl DOMMatrixReadOnly {
    pub fn from_float32_array(array32: jsbind::Float32Array) -> DOMMatrixReadOnly {
        emlite::Val::global("dommatrixreadonly")
            .call("fromFloat32Array", &[array32.into()])
            .as_::<DOMMatrixReadOnly>()
    }
}
impl DOMMatrixReadOnly {
    pub fn from_float64_array(array64: jsbind::Float64Array) -> DOMMatrixReadOnly {
        emlite::Val::global("dommatrixreadonly")
            .call("fromFloat64Array", &[array64.into()])
            .as_::<DOMMatrixReadOnly>()
    }
}
impl DOMMatrixReadOnly {
    pub fn a(&self) -> f64 {
        self.inner.get("a").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    pub fn b(&self) -> f64 {
        self.inner.get("b").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    pub fn c(&self) -> f64 {
        self.inner.get("c").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    pub fn d(&self) -> f64 {
        self.inner.get("d").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    pub fn e(&self) -> f64 {
        self.inner.get("e").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    pub fn f(&self) -> f64 {
        self.inner.get("f").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    pub fn m11(&self) -> f64 {
        self.inner.get("m11").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    pub fn m12(&self) -> f64 {
        self.inner.get("m12").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    pub fn m13(&self) -> f64 {
        self.inner.get("m13").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    pub fn m14(&self) -> f64 {
        self.inner.get("m14").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    pub fn m21(&self) -> f64 {
        self.inner.get("m21").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    pub fn m22(&self) -> f64 {
        self.inner.get("m22").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    pub fn m23(&self) -> f64 {
        self.inner.get("m23").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    pub fn m24(&self) -> f64 {
        self.inner.get("m24").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    pub fn m31(&self) -> f64 {
        self.inner.get("m31").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    pub fn m32(&self) -> f64 {
        self.inner.get("m32").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    pub fn m33(&self) -> f64 {
        self.inner.get("m33").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    pub fn m34(&self) -> f64 {
        self.inner.get("m34").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    pub fn m41(&self) -> f64 {
        self.inner.get("m41").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    pub fn m42(&self) -> f64 {
        self.inner.get("m42").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    pub fn m43(&self) -> f64 {
        self.inner.get("m43").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    pub fn m44(&self) -> f64 {
        self.inner.get("m44").as_::<f64>()
    }
}
impl DOMMatrixReadOnly {
    pub fn is2_d(&self) -> bool {
        self.inner.get("is2D").as_::<bool>()
    }
}
impl DOMMatrixReadOnly {
    pub fn is_identity(&self) -> bool {
        self.inner.get("isIdentity").as_::<bool>()
    }
}
impl DOMMatrixReadOnly {
    pub fn translate0(&self) -> DOMMatrix {
        self.inner.call("translate", &[]).as_::<DOMMatrix>()
    }

    pub fn translate1(&self, tx: f64) -> DOMMatrix {
        self.inner
            .call("translate", &[tx.into()])
            .as_::<DOMMatrix>()
    }

    pub fn translate2(&self, tx: f64, ty: f64) -> DOMMatrix {
        self.inner
            .call("translate", &[tx.into(), ty.into()])
            .as_::<DOMMatrix>()
    }

    pub fn translate3(&self, tx: f64, ty: f64, tz: f64) -> DOMMatrix {
        self.inner
            .call("translate", &[tx.into(), ty.into(), tz.into()])
            .as_::<DOMMatrix>()
    }
}
impl DOMMatrixReadOnly {
    pub fn scale0(&self) -> DOMMatrix {
        self.inner.call("scale", &[]).as_::<DOMMatrix>()
    }

    pub fn scale1(&self, scale_x: f64) -> DOMMatrix {
        self.inner
            .call("scale", &[scale_x.into()])
            .as_::<DOMMatrix>()
    }

    pub fn scale2(&self, scale_x: f64, scale_y: f64) -> DOMMatrix {
        self.inner
            .call("scale", &[scale_x.into(), scale_y.into()])
            .as_::<DOMMatrix>()
    }

    pub fn scale3(&self, scale_x: f64, scale_y: f64, scale_z: f64) -> DOMMatrix {
        self.inner
            .call("scale", &[scale_x.into(), scale_y.into(), scale_z.into()])
            .as_::<DOMMatrix>()
    }

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
    pub fn scale_non_uniform0(&self) -> DOMMatrix {
        self.inner.call("scaleNonUniform", &[]).as_::<DOMMatrix>()
    }

    pub fn scale_non_uniform1(&self, scale_x: f64) -> DOMMatrix {
        self.inner
            .call("scaleNonUniform", &[scale_x.into()])
            .as_::<DOMMatrix>()
    }

    pub fn scale_non_uniform2(&self, scale_x: f64, scale_y: f64) -> DOMMatrix {
        self.inner
            .call("scaleNonUniform", &[scale_x.into(), scale_y.into()])
            .as_::<DOMMatrix>()
    }
}
impl DOMMatrixReadOnly {
    pub fn scale3d0(&self) -> DOMMatrix {
        self.inner.call("scale3d", &[]).as_::<DOMMatrix>()
    }

    pub fn scale3d1(&self, scale: f64) -> DOMMatrix {
        self.inner
            .call("scale3d", &[scale.into()])
            .as_::<DOMMatrix>()
    }

    pub fn scale3d2(&self, scale: f64, origin_x: f64) -> DOMMatrix {
        self.inner
            .call("scale3d", &[scale.into(), origin_x.into()])
            .as_::<DOMMatrix>()
    }

    pub fn scale3d3(&self, scale: f64, origin_x: f64, origin_y: f64) -> DOMMatrix {
        self.inner
            .call("scale3d", &[scale.into(), origin_x.into(), origin_y.into()])
            .as_::<DOMMatrix>()
    }

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
    pub fn rotate0(&self) -> DOMMatrix {
        self.inner.call("rotate", &[]).as_::<DOMMatrix>()
    }

    pub fn rotate1(&self, rot_x: f64) -> DOMMatrix {
        self.inner
            .call("rotate", &[rot_x.into()])
            .as_::<DOMMatrix>()
    }

    pub fn rotate2(&self, rot_x: f64, rot_y: f64) -> DOMMatrix {
        self.inner
            .call("rotate", &[rot_x.into(), rot_y.into()])
            .as_::<DOMMatrix>()
    }

    pub fn rotate3(&self, rot_x: f64, rot_y: f64, rot_z: f64) -> DOMMatrix {
        self.inner
            .call("rotate", &[rot_x.into(), rot_y.into(), rot_z.into()])
            .as_::<DOMMatrix>()
    }
}
impl DOMMatrixReadOnly {
    pub fn rotate_from_vector0(&self) -> DOMMatrix {
        self.inner.call("rotateFromVector", &[]).as_::<DOMMatrix>()
    }

    pub fn rotate_from_vector1(&self, x: f64) -> DOMMatrix {
        self.inner
            .call("rotateFromVector", &[x.into()])
            .as_::<DOMMatrix>()
    }

    pub fn rotate_from_vector2(&self, x: f64, y: f64) -> DOMMatrix {
        self.inner
            .call("rotateFromVector", &[x.into(), y.into()])
            .as_::<DOMMatrix>()
    }
}
impl DOMMatrixReadOnly {
    pub fn rotate_axis_angle0(&self) -> DOMMatrix {
        self.inner.call("rotateAxisAngle", &[]).as_::<DOMMatrix>()
    }

    pub fn rotate_axis_angle1(&self, x: f64) -> DOMMatrix {
        self.inner
            .call("rotateAxisAngle", &[x.into()])
            .as_::<DOMMatrix>()
    }

    pub fn rotate_axis_angle2(&self, x: f64, y: f64) -> DOMMatrix {
        self.inner
            .call("rotateAxisAngle", &[x.into(), y.into()])
            .as_::<DOMMatrix>()
    }

    pub fn rotate_axis_angle3(&self, x: f64, y: f64, z: f64) -> DOMMatrix {
        self.inner
            .call("rotateAxisAngle", &[x.into(), y.into(), z.into()])
            .as_::<DOMMatrix>()
    }

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
    pub fn skew_x0(&self) -> DOMMatrix {
        self.inner.call("skewX", &[]).as_::<DOMMatrix>()
    }

    pub fn skew_x1(&self, sx: f64) -> DOMMatrix {
        self.inner.call("skewX", &[sx.into()]).as_::<DOMMatrix>()
    }
}
impl DOMMatrixReadOnly {
    pub fn skew_y0(&self) -> DOMMatrix {
        self.inner.call("skewY", &[]).as_::<DOMMatrix>()
    }

    pub fn skew_y1(&self, sy: f64) -> DOMMatrix {
        self.inner.call("skewY", &[sy.into()]).as_::<DOMMatrix>()
    }
}
impl DOMMatrixReadOnly {
    pub fn multiply0(&self) -> DOMMatrix {
        self.inner.call("multiply", &[]).as_::<DOMMatrix>()
    }

    pub fn multiply1(&self, other: DOMMatrixInit) -> DOMMatrix {
        self.inner
            .call("multiply", &[other.into()])
            .as_::<DOMMatrix>()
    }
}
impl DOMMatrixReadOnly {
    pub fn flip_x(&self) -> DOMMatrix {
        self.inner.call("flipX", &[]).as_::<DOMMatrix>()
    }
}
impl DOMMatrixReadOnly {
    pub fn flip_y(&self) -> DOMMatrix {
        self.inner.call("flipY", &[]).as_::<DOMMatrix>()
    }
}
impl DOMMatrixReadOnly {
    pub fn inverse(&self) -> DOMMatrix {
        self.inner.call("inverse", &[]).as_::<DOMMatrix>()
    }
}
impl DOMMatrixReadOnly {
    pub fn transform_point0(&self) -> DOMPoint {
        self.inner.call("transformPoint", &[]).as_::<DOMPoint>()
    }

    pub fn transform_point1(&self, point: DOMPointInit) -> DOMPoint {
        self.inner
            .call("transformPoint", &[point.into()])
            .as_::<DOMPoint>()
    }
}
impl DOMMatrixReadOnly {
    pub fn to_float32_array(&self) -> jsbind::Float32Array {
        self.inner
            .call("toFloat32Array", &[])
            .as_::<jsbind::Float32Array>()
    }
}
impl DOMMatrixReadOnly {
    pub fn to_float64_array(&self) -> jsbind::Float64Array {
        self.inner
            .call("toFloat64Array", &[])
            .as_::<jsbind::Float64Array>()
    }
}
impl DOMMatrixReadOnly {
    pub fn to_json(&self) -> jsbind::Object {
        self.inner.call("toJSON", &[]).as_::<jsbind::Object>()
    }
}
