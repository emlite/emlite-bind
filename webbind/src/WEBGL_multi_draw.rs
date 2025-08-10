use super::*;

/// The WEBGL_multi_draw class.
/// [`WEBGL_multi_draw`](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WEBGL_multi_draw {
    inner: Any,
}

impl FromVal for WEBGL_multi_draw {
    fn from_val(v: &Any) -> Self {
        WEBGL_multi_draw {
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

impl core::ops::Deref for WEBGL_multi_draw {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WEBGL_multi_draw {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WEBGL_multi_draw {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WEBGL_multi_draw {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<WEBGL_multi_draw> for Any {
    fn from(s: WEBGL_multi_draw) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WEBGL_multi_draw> for Any {
    fn from(s: &WEBGL_multi_draw) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WEBGL_multi_draw);

impl WEBGL_multi_draw {
    /// The multiDrawArraysWEBGL method.
    /// [`WEBGL_multi_draw.multiDrawArraysWEBGL`](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysWEBGL)
    pub fn multi_draw_arrays_webgl(
        &self,
        mode: &Any,
        firsts_list: &Any,
        firsts_offset: u64,
        counts_list: &Any,
        counts_offset: u64,
        drawcount: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "multiDrawArraysWEBGL",
                &[
                    mode.into(),
                    firsts_list.into(),
                    firsts_offset.into(),
                    counts_list.into(),
                    counts_offset.into(),
                    drawcount.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WEBGL_multi_draw {
    /// The multiDrawElementsWEBGL method.
    /// [`WEBGL_multi_draw.multiDrawElementsWEBGL`](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsWEBGL)
    pub fn multi_draw_elements_webgl(
        &self,
        mode: &Any,
        counts_list: &Any,
        counts_offset: u64,
        type_: &Any,
        offsets_list: &Any,
        offsets_offset: u64,
        drawcount: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "multiDrawElementsWEBGL",
                &[
                    mode.into(),
                    counts_list.into(),
                    counts_offset.into(),
                    type_.into(),
                    offsets_list.into(),
                    offsets_offset.into(),
                    drawcount.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WEBGL_multi_draw {
    /// The multiDrawArraysInstancedWEBGL method.
    /// [`WEBGL_multi_draw.multiDrawArraysInstancedWEBGL`](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawArraysInstancedWEBGL)
    pub fn multi_draw_arrays_instanced_webgl(
        &self,
        mode: &Any,
        firsts_list: &Any,
        firsts_offset: u64,
        counts_list: &Any,
        counts_offset: u64,
        instance_counts_list: &Any,
        instance_counts_offset: u64,
        drawcount: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "multiDrawArraysInstancedWEBGL",
                &[
                    mode.into(),
                    firsts_list.into(),
                    firsts_offset.into(),
                    counts_list.into(),
                    counts_offset.into(),
                    instance_counts_list.into(),
                    instance_counts_offset.into(),
                    drawcount.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WEBGL_multi_draw {
    /// The multiDrawElementsInstancedWEBGL method.
    /// [`WEBGL_multi_draw.multiDrawElementsInstancedWEBGL`](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw/multiDrawElementsInstancedWEBGL)
    pub fn multi_draw_elements_instanced_webgl(
        &self,
        mode: &Any,
        counts_list: &Any,
        counts_offset: u64,
        type_: &Any,
        offsets_list: &Any,
        offsets_offset: u64,
        instance_counts_list: &Any,
        instance_counts_offset: u64,
        drawcount: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "multiDrawElementsInstancedWEBGL",
                &[
                    mode.into(),
                    counts_list.into(),
                    counts_offset.into(),
                    type_.into(),
                    offsets_list.into(),
                    offsets_offset.into(),
                    instance_counts_list.into(),
                    instance_counts_offset.into(),
                    drawcount.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
