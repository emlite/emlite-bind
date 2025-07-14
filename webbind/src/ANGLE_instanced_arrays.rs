use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ANGLE_instanced_arrays {
    inner: emlite::Val,
}
impl FromVal for ANGLE_instanced_arrays {
    fn from_val(v: &emlite::Val) -> Self {
        ANGLE_instanced_arrays {
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
impl core::ops::Deref for ANGLE_instanced_arrays {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ANGLE_instanced_arrays {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ANGLE_instanced_arrays {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ANGLE_instanced_arrays {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ANGLE_instanced_arrays> for emlite::Val {
    fn from(s: ANGLE_instanced_arrays) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(ANGLE_instanced_arrays);

impl ANGLE_instanced_arrays {
    pub fn draw_arrays_instanced_angle(
        &self,
        mode: jsbind::Any,
        first: jsbind::Any,
        count: jsbind::Any,
        primcount: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "drawArraysInstancedANGLE",
                &[mode.into(), first.into(), count.into(), primcount.into()],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl ANGLE_instanced_arrays {
    pub fn draw_elements_instanced_angle(
        &self,
        mode: jsbind::Any,
        count: jsbind::Any,
        type_: jsbind::Any,
        offset: jsbind::Any,
        primcount: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "drawElementsInstancedANGLE",
                &[
                    mode.into(),
                    count.into(),
                    type_.into(),
                    offset.into(),
                    primcount.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl ANGLE_instanced_arrays {
    pub fn vertex_attrib_divisor_angle(
        &self,
        index: jsbind::Any,
        divisor: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call("vertexAttribDivisorANGLE", &[index.into(), divisor.into()])
            .as_::<jsbind::Undefined>()
    }
}
