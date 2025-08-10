use super::*;

/// The ANGLE_instanced_arrays class.
/// [`ANGLE_instanced_arrays`](https://developer.mozilla.org/en-US/docs/Web/API/ANGLE_instanced_arrays)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ANGLE_instanced_arrays {
    inner: Any,
}

impl FromVal for ANGLE_instanced_arrays {
    fn from_val(v: &Any) -> Self {
        ANGLE_instanced_arrays {
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

impl core::ops::Deref for ANGLE_instanced_arrays {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ANGLE_instanced_arrays {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ANGLE_instanced_arrays {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ANGLE_instanced_arrays {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ANGLE_instanced_arrays> for Any {
    fn from(s: ANGLE_instanced_arrays) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ANGLE_instanced_arrays> for Any {
    fn from(s: &ANGLE_instanced_arrays) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(ANGLE_instanced_arrays);

impl ANGLE_instanced_arrays {
    /// The drawArraysInstancedANGLE method.
    /// [`ANGLE_instanced_arrays.drawArraysInstancedANGLE`](https://developer.mozilla.org/en-US/docs/Web/API/ANGLE_instanced_arrays/drawArraysInstancedANGLE)
    pub fn draw_arrays_instanced_angle(
        &self,
        mode: &Any,
        first: &Any,
        count: &Any,
        primcount: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "drawArraysInstancedANGLE",
                &[mode.into(), first.into(), count.into(), primcount.into()],
            )
            .as_::<Undefined>()
    }
}
impl ANGLE_instanced_arrays {
    /// The drawElementsInstancedANGLE method.
    /// [`ANGLE_instanced_arrays.drawElementsInstancedANGLE`](https://developer.mozilla.org/en-US/docs/Web/API/ANGLE_instanced_arrays/drawElementsInstancedANGLE)
    pub fn draw_elements_instanced_angle(
        &self,
        mode: &Any,
        count: &Any,
        type_: &Any,
        offset: &Any,
        primcount: &Any,
    ) -> Undefined {
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
            .as_::<Undefined>()
    }
}
impl ANGLE_instanced_arrays {
    /// The vertexAttribDivisorANGLE method.
    /// [`ANGLE_instanced_arrays.vertexAttribDivisorANGLE`](https://developer.mozilla.org/en-US/docs/Web/API/ANGLE_instanced_arrays/vertexAttribDivisorANGLE)
    pub fn vertex_attrib_divisor_angle(&self, index: &Any, divisor: &Any) -> Undefined {
        self.inner
            .call("vertexAttribDivisorANGLE", &[index.into(), divisor.into()])
            .as_::<Undefined>()
    }
}
