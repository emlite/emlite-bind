use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WEBGL_draw_instanced_base_vertex_base_instance {
    inner: emlite::Val,
}
impl FromVal for WEBGL_draw_instanced_base_vertex_base_instance {
    fn from_val(v: &emlite::Val) -> Self {
        WEBGL_draw_instanced_base_vertex_base_instance {
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
impl core::ops::Deref for WEBGL_draw_instanced_base_vertex_base_instance {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WEBGL_draw_instanced_base_vertex_base_instance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WEBGL_draw_instanced_base_vertex_base_instance {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WEBGL_draw_instanced_base_vertex_base_instance {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<WEBGL_draw_instanced_base_vertex_base_instance> for emlite::Val {
    fn from(s: WEBGL_draw_instanced_base_vertex_base_instance) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(WEBGL_draw_instanced_base_vertex_base_instance);

impl WEBGL_draw_instanced_base_vertex_base_instance {
    pub fn draw_arrays_instanced_base_instance_webgl(
        &self,
        mode: jsbind::Any,
        first: jsbind::Any,
        count: jsbind::Any,
        instance_count: jsbind::Any,
        base_instance: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "drawArraysInstancedBaseInstanceWEBGL",
                &[
                    mode.into(),
                    first.into(),
                    count.into(),
                    instance_count.into(),
                    base_instance.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl WEBGL_draw_instanced_base_vertex_base_instance {
    pub fn draw_elements_instanced_base_vertex_base_instance_webgl(
        &self,
        mode: jsbind::Any,
        count: jsbind::Any,
        type_: jsbind::Any,
        offset: jsbind::Any,
        instance_count: jsbind::Any,
        base_vertex: jsbind::Any,
        base_instance: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "drawElementsInstancedBaseVertexBaseInstanceWEBGL",
                &[
                    mode.into(),
                    count.into(),
                    type_.into(),
                    offset.into(),
                    instance_count.into(),
                    base_vertex.into(),
                    base_instance.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }
}
