use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WEBGL_multi_draw {
    inner: emlite::Val,
}
impl FromVal for WEBGL_multi_draw {
    fn from_val(v: &emlite::Val) -> Self {
        WEBGL_multi_draw {
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
impl core::ops::Deref for WEBGL_multi_draw {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WEBGL_multi_draw {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WEBGL_multi_draw {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WEBGL_multi_draw {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<WEBGL_multi_draw> for emlite::Val {
    fn from(s: WEBGL_multi_draw) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(WEBGL_multi_draw);

impl WEBGL_multi_draw {
    pub fn multi_draw_arrays_webgl(
        &self,
        mode: jsbind::Any,
        firsts_list: jsbind::Any,
        firsts_offset: u64,
        counts_list: jsbind::Any,
        counts_offset: u64,
        drawcount: jsbind::Any,
    ) -> jsbind::Undefined {
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
            .as_::<jsbind::Undefined>()
    }
}
impl WEBGL_multi_draw {
    pub fn multi_draw_elements_webgl(
        &self,
        mode: jsbind::Any,
        counts_list: jsbind::Any,
        counts_offset: u64,
        type_: jsbind::Any,
        offsets_list: jsbind::Any,
        offsets_offset: u64,
        drawcount: jsbind::Any,
    ) -> jsbind::Undefined {
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
            .as_::<jsbind::Undefined>()
    }
}
impl WEBGL_multi_draw {
    pub fn multi_draw_arrays_instanced_webgl(
        &self,
        mode: jsbind::Any,
        firsts_list: jsbind::Any,
        firsts_offset: u64,
        counts_list: jsbind::Any,
        counts_offset: u64,
        instance_counts_list: jsbind::Any,
        instance_counts_offset: u64,
        drawcount: jsbind::Any,
    ) -> jsbind::Undefined {
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
            .as_::<jsbind::Undefined>()
    }
}
impl WEBGL_multi_draw {
    pub fn multi_draw_elements_instanced_webgl(
        &self,
        mode: jsbind::Any,
        counts_list: jsbind::Any,
        counts_offset: u64,
        type_: jsbind::Any,
        offsets_list: jsbind::Any,
        offsets_offset: u64,
        instance_counts_list: jsbind::Any,
        instance_counts_offset: u64,
        drawcount: jsbind::Any,
    ) -> jsbind::Undefined {
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
            .as_::<jsbind::Undefined>()
    }
}
