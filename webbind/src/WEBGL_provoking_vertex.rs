use super::*;

/// The WEBGL_provoking_vertex class.
/// [`WEBGL_provoking_vertex`](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_provoking_vertex)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WEBGL_provoking_vertex {
    inner: Any,
}

impl FromVal for WEBGL_provoking_vertex {
    fn from_val(v: &Any) -> Self {
        WEBGL_provoking_vertex {
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

impl core::ops::Deref for WEBGL_provoking_vertex {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WEBGL_provoking_vertex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WEBGL_provoking_vertex {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WEBGL_provoking_vertex {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<WEBGL_provoking_vertex> for Any {
    fn from(s: WEBGL_provoking_vertex) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WEBGL_provoking_vertex> for Any {
    fn from(s: &WEBGL_provoking_vertex) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WEBGL_provoking_vertex);

impl WEBGL_provoking_vertex {
    /// The provokingVertexWEBGL method.
    /// [`WEBGL_provoking_vertex.provokingVertexWEBGL`](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_provoking_vertex/provokingVertexWEBGL)
    pub fn provoking_vertex_webgl(&self, provoke_mode: &Any) -> Undefined {
        self.inner
            .call("provokingVertexWEBGL", &[provoke_mode.into()])
            .as_::<Undefined>()
    }
}
