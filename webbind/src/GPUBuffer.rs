use super::*;

/// The GPUBuffer class.
/// [`GPUBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/GPUBuffer)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUBuffer {
    inner: Any,
}
impl FromVal for GPUBuffer {
    fn from_val(v: &Any) -> Self {
        GPUBuffer {
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
impl core::ops::Deref for GPUBuffer {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUBuffer {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUBuffer {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUBuffer> for Any {
    fn from(s: GPUBuffer) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUBuffer> for Any {
    fn from(s: &GPUBuffer) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(GPUBuffer);

impl GPUBuffer {
    /// Getter of the `size` attribute.
    /// [`GPUBuffer.size`](https://developer.mozilla.org/en-US/docs/Web/API/GPUBuffer/size)
    pub fn size(&self) -> Any {
        self.inner.get("size").as_::<Any>()
    }
}
impl GPUBuffer {
    /// Getter of the `usage` attribute.
    /// [`GPUBuffer.usage`](https://developer.mozilla.org/en-US/docs/Web/API/GPUBuffer/usage)
    pub fn usage(&self) -> Any {
        self.inner.get("usage").as_::<Any>()
    }
}
impl GPUBuffer {
    /// Getter of the `mapState` attribute.
    /// [`GPUBuffer.mapState`](https://developer.mozilla.org/en-US/docs/Web/API/GPUBuffer/mapState)
    pub fn map_state(&self) -> GPUBufferMapState {
        self.inner.get("mapState").as_::<GPUBufferMapState>()
    }
}
impl GPUBuffer {
    /// The mapAsync method.
    /// [`GPUBuffer.mapAsync`](https://developer.mozilla.org/en-US/docs/Web/API/GPUBuffer/mapAsync)
    pub fn map_async0(&self, mode: &Any) -> Promise<Undefined> {
        self.inner
            .call("mapAsync", &[mode.into()])
            .as_::<Promise<Undefined>>()
    }
    /// The mapAsync method.
    /// [`GPUBuffer.mapAsync`](https://developer.mozilla.org/en-US/docs/Web/API/GPUBuffer/mapAsync)
    pub fn map_async1(&self, mode: &Any, offset: &Any) -> Promise<Undefined> {
        self.inner
            .call("mapAsync", &[mode.into(), offset.into()])
            .as_::<Promise<Undefined>>()
    }
    /// The mapAsync method.
    /// [`GPUBuffer.mapAsync`](https://developer.mozilla.org/en-US/docs/Web/API/GPUBuffer/mapAsync)
    pub fn map_async2(&self, mode: &Any, offset: &Any, size: &Any) -> Promise<Undefined> {
        self.inner
            .call("mapAsync", &[mode.into(), offset.into(), size.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl GPUBuffer {
    /// The getMappedRange method.
    /// [`GPUBuffer.getMappedRange`](https://developer.mozilla.org/en-US/docs/Web/API/GPUBuffer/getMappedRange)
    pub fn get_mapped_range0(&self) -> ArrayBuffer {
        self.inner.call("getMappedRange", &[]).as_::<ArrayBuffer>()
    }
    /// The getMappedRange method.
    /// [`GPUBuffer.getMappedRange`](https://developer.mozilla.org/en-US/docs/Web/API/GPUBuffer/getMappedRange)
    pub fn get_mapped_range1(&self, offset: &Any) -> ArrayBuffer {
        self.inner
            .call("getMappedRange", &[offset.into()])
            .as_::<ArrayBuffer>()
    }
    /// The getMappedRange method.
    /// [`GPUBuffer.getMappedRange`](https://developer.mozilla.org/en-US/docs/Web/API/GPUBuffer/getMappedRange)
    pub fn get_mapped_range2(&self, offset: &Any, size: &Any) -> ArrayBuffer {
        self.inner
            .call("getMappedRange", &[offset.into(), size.into()])
            .as_::<ArrayBuffer>()
    }
}
impl GPUBuffer {
    /// The unmap method.
    /// [`GPUBuffer.unmap`](https://developer.mozilla.org/en-US/docs/Web/API/GPUBuffer/unmap)
    pub fn unmap(&self) -> Undefined {
        self.inner.call("unmap", &[]).as_::<Undefined>()
    }
}
impl GPUBuffer {
    /// The destroy method.
    /// [`GPUBuffer.destroy`](https://developer.mozilla.org/en-US/docs/Web/API/GPUBuffer/destroy)
    pub fn destroy(&self) -> Undefined {
        self.inner.call("destroy", &[]).as_::<Undefined>()
    }
}
impl GPUBuffer {
    /// Getter of the `label` attribute.
    /// [`GPUBuffer.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUBuffer/label)
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }

    /// Setter of the `label` attribute.
    /// [`GPUBuffer.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUBuffer/label)
    pub fn set_label(&mut self, value: &JsString) {
        self.inner.set("label", value);
    }
}
