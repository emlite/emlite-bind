use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUTexelCopyBufferLayout {
    inner: emlite::Val,
}
impl FromVal for GPUTexelCopyBufferLayout {
    fn from_val(v: &emlite::Val) -> Self {
        GPUTexelCopyBufferLayout { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUTexelCopyBufferLayout {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUTexelCopyBufferLayout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPUTexelCopyBufferLayout {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPUTexelCopyBufferLayout {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GPUTexelCopyBufferLayout> for emlite::Val {
    fn from(s: GPUTexelCopyBufferLayout) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&GPUTexelCopyBufferLayout> for emlite::Val {
    fn from(s: &GPUTexelCopyBufferLayout) -> emlite::Val {
        s.inner.clone()
    }
}

impl GPUTexelCopyBufferLayout {
    pub fn offset(&self) -> Any {
        self.inner.get("offset").as_::<Any>()
    }

    pub fn set_offset(&mut self, value: Any) {
        self.inner.set("offset", value);
    }
}
impl GPUTexelCopyBufferLayout {
    pub fn bytes_per_row(&self) -> Any {
        self.inner.get("bytesPerRow").as_::<Any>()
    }

    pub fn set_bytes_per_row(&mut self, value: Any) {
        self.inner.set("bytesPerRow", value);
    }
}
impl GPUTexelCopyBufferLayout {
    pub fn rows_per_image(&self) -> Any {
        self.inner.get("rowsPerImage").as_::<Any>()
    }

    pub fn set_rows_per_image(&mut self, value: Any) {
        self.inner.set("rowsPerImage", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUCopyExternalImageSourceInfo {
    inner: emlite::Val,
}
impl FromVal for GPUCopyExternalImageSourceInfo {
    fn from_val(v: &emlite::Val) -> Self {
        GPUCopyExternalImageSourceInfo { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUCopyExternalImageSourceInfo {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUCopyExternalImageSourceInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPUCopyExternalImageSourceInfo {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPUCopyExternalImageSourceInfo {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GPUCopyExternalImageSourceInfo> for emlite::Val {
    fn from(s: GPUCopyExternalImageSourceInfo) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&GPUCopyExternalImageSourceInfo> for emlite::Val {
    fn from(s: &GPUCopyExternalImageSourceInfo) -> emlite::Val {
        s.inner.clone()
    }
}

impl GPUCopyExternalImageSourceInfo {
    pub fn source(&self) -> Any {
        self.inner.get("source").as_::<Any>()
    }

    pub fn set_source(&mut self, value: Any) {
        self.inner.set("source", value);
    }
}
impl GPUCopyExternalImageSourceInfo {
    pub fn origin(&self) -> Any {
        self.inner.get("origin").as_::<Any>()
    }

    pub fn set_origin(&mut self, value: Any) {
        self.inner.set("origin", value);
    }
}
impl GPUCopyExternalImageSourceInfo {
    pub fn flip_y(&self) -> bool {
        self.inner.get("flipY").as_::<bool>()
    }

    pub fn set_flip_y(&mut self, value: bool) {
        self.inner.set("flipY", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUCopyExternalImageDestInfo {
    inner: emlite::Val,
}
impl FromVal for GPUCopyExternalImageDestInfo {
    fn from_val(v: &emlite::Val) -> Self {
        GPUCopyExternalImageDestInfo { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUCopyExternalImageDestInfo {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUCopyExternalImageDestInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPUCopyExternalImageDestInfo {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPUCopyExternalImageDestInfo {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GPUCopyExternalImageDestInfo> for emlite::Val {
    fn from(s: GPUCopyExternalImageDestInfo) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&GPUCopyExternalImageDestInfo> for emlite::Val {
    fn from(s: &GPUCopyExternalImageDestInfo) -> emlite::Val {
        s.inner.clone()
    }
}

impl GPUCopyExternalImageDestInfo {
    pub fn color_space(&self) -> PredefinedColorSpace {
        self.inner.get("colorSpace").as_::<PredefinedColorSpace>()
    }

    pub fn set_color_space(&mut self, value: PredefinedColorSpace) {
        self.inner.set("colorSpace", value);
    }
}
impl GPUCopyExternalImageDestInfo {
    pub fn premultiplied_alpha(&self) -> bool {
        self.inner.get("premultipliedAlpha").as_::<bool>()
    }

    pub fn set_premultiplied_alpha(&mut self, value: bool) {
        self.inner.set("premultipliedAlpha", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUQueue {
    inner: emlite::Val,
}
impl FromVal for GPUQueue {
    fn from_val(v: &emlite::Val) -> Self {
        GPUQueue {
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
impl core::ops::Deref for GPUQueue {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPUQueue {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPUQueue {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GPUQueue> for emlite::Val {
    fn from(s: GPUQueue) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&GPUQueue> for emlite::Val {
    fn from(s: &GPUQueue) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(GPUQueue);

impl GPUQueue {
    pub fn submit(&self, command_buffers: Sequence<GPUCommandBuffer>) -> Undefined {
        self.inner
            .call("submit", &[command_buffers.into()])
            .as_::<Undefined>()
    }
}
impl GPUQueue {
    pub fn on_submitted_work_done(&self) -> Promise {
        self.inner.call("onSubmittedWorkDone", &[]).as_::<Promise>()
    }
}
impl GPUQueue {
    pub fn write_buffer0(&self, buffer: GPUBuffer, buffer_offset: Any, data: Any) -> Undefined {
        self.inner
            .call(
                "writeBuffer",
                &[buffer.into(), buffer_offset.into(), data.into()],
            )
            .as_::<Undefined>()
    }

    pub fn write_buffer1(
        &self,
        buffer: GPUBuffer,
        buffer_offset: Any,
        data: Any,
        data_offset: Any,
    ) -> Undefined {
        self.inner
            .call(
                "writeBuffer",
                &[
                    buffer.into(),
                    buffer_offset.into(),
                    data.into(),
                    data_offset.into(),
                ],
            )
            .as_::<Undefined>()
    }

    pub fn write_buffer2(
        &self,
        buffer: GPUBuffer,
        buffer_offset: Any,
        data: Any,
        data_offset: Any,
        size: Any,
    ) -> Undefined {
        self.inner
            .call(
                "writeBuffer",
                &[
                    buffer.into(),
                    buffer_offset.into(),
                    data.into(),
                    data_offset.into(),
                    size.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl GPUQueue {
    pub fn write_texture(
        &self,
        destination: GPUTexelCopyTextureInfo,
        data: Any,
        data_layout: GPUTexelCopyBufferLayout,
        size: Any,
    ) -> Undefined {
        self.inner
            .call(
                "writeTexture",
                &[
                    destination.into(),
                    data.into(),
                    data_layout.into(),
                    size.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl GPUQueue {
    pub fn copy_external_image_to_texture(
        &self,
        source: GPUCopyExternalImageSourceInfo,
        destination: GPUCopyExternalImageDestInfo,
        copy_size: Any,
    ) -> Undefined {
        self.inner
            .call(
                "copyExternalImageToTexture",
                &[source.into(), destination.into(), copy_size.into()],
            )
            .as_::<Undefined>()
    }
}
impl GPUQueue {
    pub fn label(&self) -> USVString {
        self.inner.get("label").as_::<USVString>()
    }

    pub fn set_label(&mut self, value: USVString) {
        self.inner.set("label", value);
    }
}
