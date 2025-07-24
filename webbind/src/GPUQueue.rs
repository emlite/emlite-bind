use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUTexelCopyBufferLayout {
    inner: Any,
}
impl FromVal for GPUTexelCopyBufferLayout {
    fn from_val(v: &Any) -> Self {
        GPUTexelCopyBufferLayout { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUTexelCopyBufferLayout {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUTexelCopyBufferLayout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUTexelCopyBufferLayout {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUTexelCopyBufferLayout {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUTexelCopyBufferLayout> for Any {
    fn from(s: GPUTexelCopyBufferLayout) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUTexelCopyBufferLayout> for Any {
    fn from(s: &GPUTexelCopyBufferLayout) -> Any {
        s.inner.clone()
    }
}

impl GPUTexelCopyBufferLayout {
    pub fn offset(&self) -> Any {
        self.inner.get("offset").as_::<Any>()
    }

    pub fn set_offset(&mut self, value: &Any) {
        self.inner.set("offset", value);
    }
}
impl GPUTexelCopyBufferLayout {
    pub fn bytes_per_row(&self) -> Any {
        self.inner.get("bytesPerRow").as_::<Any>()
    }

    pub fn set_bytes_per_row(&mut self, value: &Any) {
        self.inner.set("bytesPerRow", value);
    }
}
impl GPUTexelCopyBufferLayout {
    pub fn rows_per_image(&self) -> Any {
        self.inner.get("rowsPerImage").as_::<Any>()
    }

    pub fn set_rows_per_image(&mut self, value: &Any) {
        self.inner.set("rowsPerImage", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUCopyExternalImageSourceInfo {
    inner: Any,
}
impl FromVal for GPUCopyExternalImageSourceInfo {
    fn from_val(v: &Any) -> Self {
        GPUCopyExternalImageSourceInfo { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUCopyExternalImageSourceInfo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUCopyExternalImageSourceInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUCopyExternalImageSourceInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUCopyExternalImageSourceInfo {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUCopyExternalImageSourceInfo> for Any {
    fn from(s: GPUCopyExternalImageSourceInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUCopyExternalImageSourceInfo> for Any {
    fn from(s: &GPUCopyExternalImageSourceInfo) -> Any {
        s.inner.clone()
    }
}

impl GPUCopyExternalImageSourceInfo {
    pub fn source(&self) -> Any {
        self.inner.get("source").as_::<Any>()
    }

    pub fn set_source(&mut self, value: &Any) {
        self.inner.set("source", value);
    }
}
impl GPUCopyExternalImageSourceInfo {
    pub fn origin(&self) -> Any {
        self.inner.get("origin").as_::<Any>()
    }

    pub fn set_origin(&mut self, value: &Any) {
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
    inner: Any,
}
impl FromVal for GPUCopyExternalImageDestInfo {
    fn from_val(v: &Any) -> Self {
        GPUCopyExternalImageDestInfo { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUCopyExternalImageDestInfo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUCopyExternalImageDestInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUCopyExternalImageDestInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUCopyExternalImageDestInfo {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUCopyExternalImageDestInfo> for Any {
    fn from(s: GPUCopyExternalImageDestInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUCopyExternalImageDestInfo> for Any {
    fn from(s: &GPUCopyExternalImageDestInfo) -> Any {
        s.inner.clone()
    }
}

impl GPUCopyExternalImageDestInfo {
    pub fn color_space(&self) -> PredefinedColorSpace {
        self.inner.get("colorSpace").as_::<PredefinedColorSpace>()
    }

    pub fn set_color_space(&mut self, value: &PredefinedColorSpace) {
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
/// The GPUQueue class.
/// [`GPUQueue`](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUQueue {
    inner: Any,
}
impl FromVal for GPUQueue {
    fn from_val(v: &Any) -> Self {
        GPUQueue {
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
impl core::ops::Deref for GPUQueue {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUQueue {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUQueue {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUQueue> for Any {
    fn from(s: GPUQueue) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUQueue> for Any {
    fn from(s: &GPUQueue) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(GPUQueue);

impl GPUQueue {
    /// The submit method.
    /// [`GPUQueue.submit`](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/submit)
    pub fn submit(&self, command_buffers: &Sequence<GPUCommandBuffer>) -> Undefined {
        self.inner
            .call("submit", &[command_buffers.into()])
            .as_::<Undefined>()
    }
}
impl GPUQueue {
    /// The onSubmittedWorkDone method.
    /// [`GPUQueue.onSubmittedWorkDone`](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/onSubmittedWorkDone)
    pub fn on_submitted_work_done(&self) -> Promise<Undefined> {
        self.inner
            .call("onSubmittedWorkDone", &[])
            .as_::<Promise<Undefined>>()
    }
}
impl GPUQueue {
    /// The writeBuffer method.
    /// [`GPUQueue.writeBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)
    pub fn write_buffer0(&self, buffer: &GPUBuffer, buffer_offset: &Any, data: &Any) -> Undefined {
        self.inner
            .call(
                "writeBuffer",
                &[buffer.into(), buffer_offset.into(), data.into()],
            )
            .as_::<Undefined>()
    }
    /// The writeBuffer method.
    /// [`GPUQueue.writeBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)
    pub fn write_buffer1(
        &self,
        buffer: &GPUBuffer,
        buffer_offset: &Any,
        data: &Any,
        data_offset: &Any,
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
    /// The writeBuffer method.
    /// [`GPUQueue.writeBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)
    pub fn write_buffer2(
        &self,
        buffer: &GPUBuffer,
        buffer_offset: &Any,
        data: &Any,
        data_offset: &Any,
        size: &Any,
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
    /// The writeTexture method.
    /// [`GPUQueue.writeTexture`](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeTexture)
    pub fn write_texture(
        &self,
        destination: &GPUTexelCopyTextureInfo,
        data: &Any,
        data_layout: &GPUTexelCopyBufferLayout,
        size: &Any,
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
    /// The copyExternalImageToTexture method.
    /// [`GPUQueue.copyExternalImageToTexture`](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/copyExternalImageToTexture)
    pub fn copy_external_image_to_texture(
        &self,
        source: &GPUCopyExternalImageSourceInfo,
        destination: &GPUCopyExternalImageDestInfo,
        copy_size: &Any,
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
    /// Getter of the `label` attribute.
    /// [`GPUQueue.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/label)
    pub fn label(&self) -> USVString {
        self.inner.get("label").as_::<USVString>()
    }

    /// Setter of the `label` attribute.
    /// [`GPUQueue.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/label)
    pub fn set_label(&mut self, value: &USVString) {
        self.inner.set("label", value);
    }
}
