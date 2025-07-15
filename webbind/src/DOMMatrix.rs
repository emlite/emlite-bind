use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DOMMatrix {
    inner: DOMMatrixReadOnly,
}
impl FromVal for DOMMatrix {
    fn from_val(v: &emlite::Val) -> Self {
        DOMMatrix { inner: DOMMatrixReadOnly::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for DOMMatrix {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DOMMatrix {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<DOMMatrix> for emlite::Val {
    fn from(s: DOMMatrix) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(DOMMatrix);



impl DOMMatrix {
    pub fn new0() -> DOMMatrix {
        Self {
            inner: emlite::Val::global("DOMMatrix").new(&[]).as_::<DOMMatrixReadOnly>(),
        }
    }

    pub fn new1(init: Any) -> DOMMatrix {
        Self {
            inner: emlite::Val::global("DOMMatrix").new(&[init.into()]).as_::<DOMMatrixReadOnly>(),
        }
    }

}
impl DOMMatrix {
    pub fn from_matrix0() -> DOMMatrix {
        emlite::Val::global("dommatrix").call("fromMatrix", &[]).as_::<DOMMatrix>()
    }

    pub fn from_matrix1(other: DOMMatrixInit) -> DOMMatrix {
        emlite::Val::global("dommatrix").call("fromMatrix", &[other.into(), ]).as_::<DOMMatrix>()
    }

}
impl DOMMatrix {
    pub fn from_float32_array(array32: Float32Array) -> DOMMatrix {
        emlite::Val::global("dommatrix").call("fromFloat32Array", &[array32.into(), ]).as_::<DOMMatrix>()
    }

}
impl DOMMatrix {
    pub fn from_float64_array(array64: Float64Array) -> DOMMatrix {
        emlite::Val::global("dommatrix").call("fromFloat64Array", &[array64.into(), ]).as_::<DOMMatrix>()
    }

}
impl DOMMatrix {
    pub fn a(&self) -> f64 {
        self.inner.get("a").as_::<f64>()
    }

    pub fn set_a(&mut self, value: f64) {
        self.inner.set("a", value);
    }

}
impl DOMMatrix {
    pub fn b(&self) -> f64 {
        self.inner.get("b").as_::<f64>()
    }

    pub fn set_b(&mut self, value: f64) {
        self.inner.set("b", value);
    }

}
impl DOMMatrix {
    pub fn c(&self) -> f64 {
        self.inner.get("c").as_::<f64>()
    }

    pub fn set_c(&mut self, value: f64) {
        self.inner.set("c", value);
    }

}
impl DOMMatrix {
    pub fn d(&self) -> f64 {
        self.inner.get("d").as_::<f64>()
    }

    pub fn set_d(&mut self, value: f64) {
        self.inner.set("d", value);
    }

}
impl DOMMatrix {
    pub fn e(&self) -> f64 {
        self.inner.get("e").as_::<f64>()
    }

    pub fn set_e(&mut self, value: f64) {
        self.inner.set("e", value);
    }

}
impl DOMMatrix {
    pub fn f(&self) -> f64 {
        self.inner.get("f").as_::<f64>()
    }

    pub fn set_f(&mut self, value: f64) {
        self.inner.set("f", value);
    }

}
impl DOMMatrix {
    pub fn m11(&self) -> f64 {
        self.inner.get("m11").as_::<f64>()
    }

    pub fn set_m11(&mut self, value: f64) {
        self.inner.set("m11", value);
    }

}
impl DOMMatrix {
    pub fn m12(&self) -> f64 {
        self.inner.get("m12").as_::<f64>()
    }

    pub fn set_m12(&mut self, value: f64) {
        self.inner.set("m12", value);
    }

}
impl DOMMatrix {
    pub fn m13(&self) -> f64 {
        self.inner.get("m13").as_::<f64>()
    }

    pub fn set_m13(&mut self, value: f64) {
        self.inner.set("m13", value);
    }

}
impl DOMMatrix {
    pub fn m14(&self) -> f64 {
        self.inner.get("m14").as_::<f64>()
    }

    pub fn set_m14(&mut self, value: f64) {
        self.inner.set("m14", value);
    }

}
impl DOMMatrix {
    pub fn m21(&self) -> f64 {
        self.inner.get("m21").as_::<f64>()
    }

    pub fn set_m21(&mut self, value: f64) {
        self.inner.set("m21", value);
    }

}
impl DOMMatrix {
    pub fn m22(&self) -> f64 {
        self.inner.get("m22").as_::<f64>()
    }

    pub fn set_m22(&mut self, value: f64) {
        self.inner.set("m22", value);
    }

}
impl DOMMatrix {
    pub fn m23(&self) -> f64 {
        self.inner.get("m23").as_::<f64>()
    }

    pub fn set_m23(&mut self, value: f64) {
        self.inner.set("m23", value);
    }

}
impl DOMMatrix {
    pub fn m24(&self) -> f64 {
        self.inner.get("m24").as_::<f64>()
    }

    pub fn set_m24(&mut self, value: f64) {
        self.inner.set("m24", value);
    }

}
impl DOMMatrix {
    pub fn m31(&self) -> f64 {
        self.inner.get("m31").as_::<f64>()
    }

    pub fn set_m31(&mut self, value: f64) {
        self.inner.set("m31", value);
    }

}
impl DOMMatrix {
    pub fn m32(&self) -> f64 {
        self.inner.get("m32").as_::<f64>()
    }

    pub fn set_m32(&mut self, value: f64) {
        self.inner.set("m32", value);
    }

}
impl DOMMatrix {
    pub fn m33(&self) -> f64 {
        self.inner.get("m33").as_::<f64>()
    }

    pub fn set_m33(&mut self, value: f64) {
        self.inner.set("m33", value);
    }

}
impl DOMMatrix {
    pub fn m34(&self) -> f64 {
        self.inner.get("m34").as_::<f64>()
    }

    pub fn set_m34(&mut self, value: f64) {
        self.inner.set("m34", value);
    }

}
impl DOMMatrix {
    pub fn m41(&self) -> f64 {
        self.inner.get("m41").as_::<f64>()
    }

    pub fn set_m41(&mut self, value: f64) {
        self.inner.set("m41", value);
    }

}
impl DOMMatrix {
    pub fn m42(&self) -> f64 {
        self.inner.get("m42").as_::<f64>()
    }

    pub fn set_m42(&mut self, value: f64) {
        self.inner.set("m42", value);
    }

}
impl DOMMatrix {
    pub fn m43(&self) -> f64 {
        self.inner.get("m43").as_::<f64>()
    }

    pub fn set_m43(&mut self, value: f64) {
        self.inner.set("m43", value);
    }

}
impl DOMMatrix {
    pub fn m44(&self) -> f64 {
        self.inner.get("m44").as_::<f64>()
    }

    pub fn set_m44(&mut self, value: f64) {
        self.inner.set("m44", value);
    }

}
impl DOMMatrix {
    pub fn multiply_self0(&self, ) -> DOMMatrix {
        self.inner.call("multiplySelf", &[]).as_::<DOMMatrix>()
    }

    pub fn multiply_self1(&self, other: DOMMatrixInit) -> DOMMatrix {
        self.inner.call("multiplySelf", &[other.into(), ]).as_::<DOMMatrix>()
    }

}
impl DOMMatrix {
    pub fn pre_multiply_self0(&self, ) -> DOMMatrix {
        self.inner.call("preMultiplySelf", &[]).as_::<DOMMatrix>()
    }

    pub fn pre_multiply_self1(&self, other: DOMMatrixInit) -> DOMMatrix {
        self.inner.call("preMultiplySelf", &[other.into(), ]).as_::<DOMMatrix>()
    }

}
impl DOMMatrix {
    pub fn translate_self0(&self, ) -> DOMMatrix {
        self.inner.call("translateSelf", &[]).as_::<DOMMatrix>()
    }

    pub fn translate_self1(&self, tx: f64) -> DOMMatrix {
        self.inner.call("translateSelf", &[tx.into(), ]).as_::<DOMMatrix>()
    }

    pub fn translate_self2(&self, tx: f64, ty: f64) -> DOMMatrix {
        self.inner.call("translateSelf", &[tx.into(), ty.into(), ]).as_::<DOMMatrix>()
    }

    pub fn translate_self3(&self, tx: f64, ty: f64, tz: f64) -> DOMMatrix {
        self.inner.call("translateSelf", &[tx.into(), ty.into(), tz.into(), ]).as_::<DOMMatrix>()
    }

}
impl DOMMatrix {
    pub fn scale_self0(&self, ) -> DOMMatrix {
        self.inner.call("scaleSelf", &[]).as_::<DOMMatrix>()
    }

    pub fn scale_self1(&self, scale_x: f64) -> DOMMatrix {
        self.inner.call("scaleSelf", &[scale_x.into(), ]).as_::<DOMMatrix>()
    }

    pub fn scale_self2(&self, scale_x: f64, scale_y: f64) -> DOMMatrix {
        self.inner.call("scaleSelf", &[scale_x.into(), scale_y.into(), ]).as_::<DOMMatrix>()
    }

    pub fn scale_self3(&self, scale_x: f64, scale_y: f64, scale_z: f64) -> DOMMatrix {
        self.inner.call("scaleSelf", &[scale_x.into(), scale_y.into(), scale_z.into(), ]).as_::<DOMMatrix>()
    }

    pub fn scale_self4(&self, scale_x: f64, scale_y: f64, scale_z: f64, origin_x: f64) -> DOMMatrix {
        self.inner.call("scaleSelf", &[scale_x.into(), scale_y.into(), scale_z.into(), origin_x.into(), ]).as_::<DOMMatrix>()
    }

    pub fn scale_self5(&self, scale_x: f64, scale_y: f64, scale_z: f64, origin_x: f64, origin_y: f64) -> DOMMatrix {
        self.inner.call("scaleSelf", &[scale_x.into(), scale_y.into(), scale_z.into(), origin_x.into(), origin_y.into(), ]).as_::<DOMMatrix>()
    }

    pub fn scale_self6(&self, scale_x: f64, scale_y: f64, scale_z: f64, origin_x: f64, origin_y: f64, origin_z: f64) -> DOMMatrix {
        self.inner.call("scaleSelf", &[scale_x.into(), scale_y.into(), scale_z.into(), origin_x.into(), origin_y.into(), origin_z.into(), ]).as_::<DOMMatrix>()
    }

}
impl DOMMatrix {
    pub fn scale3d_self0(&self, ) -> DOMMatrix {
        self.inner.call("scale3dSelf", &[]).as_::<DOMMatrix>()
    }

    pub fn scale3d_self1(&self, scale: f64) -> DOMMatrix {
        self.inner.call("scale3dSelf", &[scale.into(), ]).as_::<DOMMatrix>()
    }

    pub fn scale3d_self2(&self, scale: f64, origin_x: f64) -> DOMMatrix {
        self.inner.call("scale3dSelf", &[scale.into(), origin_x.into(), ]).as_::<DOMMatrix>()
    }

    pub fn scale3d_self3(&self, scale: f64, origin_x: f64, origin_y: f64) -> DOMMatrix {
        self.inner.call("scale3dSelf", &[scale.into(), origin_x.into(), origin_y.into(), ]).as_::<DOMMatrix>()
    }

    pub fn scale3d_self4(&self, scale: f64, origin_x: f64, origin_y: f64, origin_z: f64) -> DOMMatrix {
        self.inner.call("scale3dSelf", &[scale.into(), origin_x.into(), origin_y.into(), origin_z.into(), ]).as_::<DOMMatrix>()
    }

}
impl DOMMatrix {
    pub fn rotate_self0(&self, ) -> DOMMatrix {
        self.inner.call("rotateSelf", &[]).as_::<DOMMatrix>()
    }

    pub fn rotate_self1(&self, rot_x: f64) -> DOMMatrix {
        self.inner.call("rotateSelf", &[rot_x.into(), ]).as_::<DOMMatrix>()
    }

    pub fn rotate_self2(&self, rot_x: f64, rot_y: f64) -> DOMMatrix {
        self.inner.call("rotateSelf", &[rot_x.into(), rot_y.into(), ]).as_::<DOMMatrix>()
    }

    pub fn rotate_self3(&self, rot_x: f64, rot_y: f64, rot_z: f64) -> DOMMatrix {
        self.inner.call("rotateSelf", &[rot_x.into(), rot_y.into(), rot_z.into(), ]).as_::<DOMMatrix>()
    }

}
impl DOMMatrix {
    pub fn rotate_from_vector_self0(&self, ) -> DOMMatrix {
        self.inner.call("rotateFromVectorSelf", &[]).as_::<DOMMatrix>()
    }

    pub fn rotate_from_vector_self1(&self, x: f64) -> DOMMatrix {
        self.inner.call("rotateFromVectorSelf", &[x.into(), ]).as_::<DOMMatrix>()
    }

    pub fn rotate_from_vector_self2(&self, x: f64, y: f64) -> DOMMatrix {
        self.inner.call("rotateFromVectorSelf", &[x.into(), y.into(), ]).as_::<DOMMatrix>()
    }

}
impl DOMMatrix {
    pub fn rotate_axis_angle_self0(&self, ) -> DOMMatrix {
        self.inner.call("rotateAxisAngleSelf", &[]).as_::<DOMMatrix>()
    }

    pub fn rotate_axis_angle_self1(&self, x: f64) -> DOMMatrix {
        self.inner.call("rotateAxisAngleSelf", &[x.into(), ]).as_::<DOMMatrix>()
    }

    pub fn rotate_axis_angle_self2(&self, x: f64, y: f64) -> DOMMatrix {
        self.inner.call("rotateAxisAngleSelf", &[x.into(), y.into(), ]).as_::<DOMMatrix>()
    }

    pub fn rotate_axis_angle_self3(&self, x: f64, y: f64, z: f64) -> DOMMatrix {
        self.inner.call("rotateAxisAngleSelf", &[x.into(), y.into(), z.into(), ]).as_::<DOMMatrix>()
    }

    pub fn rotate_axis_angle_self4(&self, x: f64, y: f64, z: f64, angle: f64) -> DOMMatrix {
        self.inner.call("rotateAxisAngleSelf", &[x.into(), y.into(), z.into(), angle.into(), ]).as_::<DOMMatrix>()
    }

}
impl DOMMatrix {
    pub fn skew_x_self0(&self, ) -> DOMMatrix {
        self.inner.call("skewXSelf", &[]).as_::<DOMMatrix>()
    }

    pub fn skew_x_self1(&self, sx: f64) -> DOMMatrix {
        self.inner.call("skewXSelf", &[sx.into(), ]).as_::<DOMMatrix>()
    }

}
impl DOMMatrix {
    pub fn skew_y_self0(&self, ) -> DOMMatrix {
        self.inner.call("skewYSelf", &[]).as_::<DOMMatrix>()
    }

    pub fn skew_y_self1(&self, sy: f64) -> DOMMatrix {
        self.inner.call("skewYSelf", &[sy.into(), ]).as_::<DOMMatrix>()
    }

}
impl DOMMatrix {
    pub fn invert_self(&self, ) -> DOMMatrix {
        self.inner.call("invertSelf", &[]).as_::<DOMMatrix>()
    }

}
impl DOMMatrix {
    pub fn set_matrix_value(&self, transform_list: DOMString) -> DOMMatrix {
        self.inner.call("setMatrixValue", &[transform_list.into(), ]).as_::<DOMMatrix>()
    }

}
