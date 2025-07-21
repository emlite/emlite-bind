use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoFrameMetadata {
    inner: Any,
}
impl FromVal for VideoFrameMetadata {
    fn from_val(v: &Any) -> Self {
        VideoFrameMetadata { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for VideoFrameMetadata {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VideoFrameMetadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for VideoFrameMetadata {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for VideoFrameMetadata {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<VideoFrameMetadata> for Any {
    fn from(s: VideoFrameMetadata) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&VideoFrameMetadata> for Any {
    fn from(s: &VideoFrameMetadata) -> Any {
        s.inner.clone()
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoFrameCopyToOptions {
    inner: Any,
}
impl FromVal for VideoFrameCopyToOptions {
    fn from_val(v: &Any) -> Self {
        VideoFrameCopyToOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for VideoFrameCopyToOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VideoFrameCopyToOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for VideoFrameCopyToOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for VideoFrameCopyToOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<VideoFrameCopyToOptions> for Any {
    fn from(s: VideoFrameCopyToOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&VideoFrameCopyToOptions> for Any {
    fn from(s: &VideoFrameCopyToOptions) -> Any {
        s.inner.clone()
    }
}

impl VideoFrameCopyToOptions {
    pub fn rect(&self) -> DOMRectInit {
        self.inner.get("rect").as_::<DOMRectInit>()
    }

    pub fn set_rect(&mut self, value: &DOMRectInit) {
        self.inner.set("rect", value);
    }
}
impl VideoFrameCopyToOptions {
    pub fn layout(&self) -> Sequence<PlaneLayout> {
        self.inner.get("layout").as_::<Sequence<PlaneLayout>>()
    }

    pub fn set_layout(&mut self, value: &Sequence<PlaneLayout>) {
        self.inner.set("layout", value);
    }
}
impl VideoFrameCopyToOptions {
    pub fn format(&self) -> VideoPixelFormat {
        self.inner.get("format").as_::<VideoPixelFormat>()
    }

    pub fn set_format(&mut self, value: &VideoPixelFormat) {
        self.inner.set("format", value);
    }
}
impl VideoFrameCopyToOptions {
    pub fn color_space(&self) -> PredefinedColorSpace {
        self.inner.get("colorSpace").as_::<PredefinedColorSpace>()
    }

    pub fn set_color_space(&mut self, value: &PredefinedColorSpace) {
        self.inner.set("colorSpace", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PlaneLayout {
    inner: Any,
}
impl FromVal for PlaneLayout {
    fn from_val(v: &Any) -> Self {
        PlaneLayout { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PlaneLayout {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PlaneLayout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PlaneLayout {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PlaneLayout {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PlaneLayout> for Any {
    fn from(s: PlaneLayout) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PlaneLayout> for Any {
    fn from(s: &PlaneLayout) -> Any {
        s.inner.clone()
    }
}

impl PlaneLayout {
    pub fn offset(&self) -> u32 {
        self.inner.get("offset").as_::<u32>()
    }

    pub fn set_offset(&mut self, value: u32) {
        self.inner.set("offset", value);
    }
}
impl PlaneLayout {
    pub fn stride(&self) -> u32 {
        self.inner.get("stride").as_::<u32>()
    }

    pub fn set_stride(&mut self, value: u32) {
        self.inner.set("stride", value);
    }
}
/// The VideoFrame class.
/// [`VideoFrame`](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoFrame {
    inner: Any,
}
impl FromVal for VideoFrame {
    fn from_val(v: &Any) -> Self {
        VideoFrame {
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
impl core::ops::Deref for VideoFrame {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VideoFrame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for VideoFrame {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for VideoFrame {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<VideoFrame> for Any {
    fn from(s: VideoFrame) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&VideoFrame> for Any {
    fn from(s: &VideoFrame) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(VideoFrame);

impl VideoFrame {
    /// The `new VideoFrame(..)` constructor, creating a new VideoFrame instance
    pub fn new(data: &Any, init: &Any) -> VideoFrame {
        Self {
            inner: Any::global("VideoFrame")
                .new(&[data.into(), init.into()])
                .as_::<Any>(),
        }
    }
}
impl VideoFrame {
    /// Getter of the `format` attribute.
    /// [`VideoFrame.format`](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/format)
    pub fn format(&self) -> VideoPixelFormat {
        self.inner.get("format").as_::<VideoPixelFormat>()
    }
}
impl VideoFrame {
    /// Getter of the `codedWidth` attribute.
    /// [`VideoFrame.codedWidth`](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/codedWidth)
    pub fn coded_width(&self) -> u32 {
        self.inner.get("codedWidth").as_::<u32>()
    }
}
impl VideoFrame {
    /// Getter of the `codedHeight` attribute.
    /// [`VideoFrame.codedHeight`](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/codedHeight)
    pub fn coded_height(&self) -> u32 {
        self.inner.get("codedHeight").as_::<u32>()
    }
}
impl VideoFrame {
    /// Getter of the `codedRect` attribute.
    /// [`VideoFrame.codedRect`](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/codedRect)
    pub fn coded_rect(&self) -> DOMRectReadOnly {
        self.inner.get("codedRect").as_::<DOMRectReadOnly>()
    }
}
impl VideoFrame {
    /// Getter of the `visibleRect` attribute.
    /// [`VideoFrame.visibleRect`](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/visibleRect)
    pub fn visible_rect(&self) -> DOMRectReadOnly {
        self.inner.get("visibleRect").as_::<DOMRectReadOnly>()
    }
}
impl VideoFrame {
    /// Getter of the `rotation` attribute.
    /// [`VideoFrame.rotation`](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/rotation)
    pub fn rotation(&self) -> f64 {
        self.inner.get("rotation").as_::<f64>()
    }
}
impl VideoFrame {
    /// Getter of the `flip` attribute.
    /// [`VideoFrame.flip`](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/flip)
    pub fn flip(&self) -> bool {
        self.inner.get("flip").as_::<bool>()
    }
}
impl VideoFrame {
    /// Getter of the `displayWidth` attribute.
    /// [`VideoFrame.displayWidth`](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/displayWidth)
    pub fn display_width(&self) -> u32 {
        self.inner.get("displayWidth").as_::<u32>()
    }
}
impl VideoFrame {
    /// Getter of the `displayHeight` attribute.
    /// [`VideoFrame.displayHeight`](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/displayHeight)
    pub fn display_height(&self) -> u32 {
        self.inner.get("displayHeight").as_::<u32>()
    }
}
impl VideoFrame {
    /// Getter of the `duration` attribute.
    /// [`VideoFrame.duration`](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/duration)
    pub fn duration(&self) -> u64 {
        self.inner.get("duration").as_::<u64>()
    }
}
impl VideoFrame {
    /// Getter of the `timestamp` attribute.
    /// [`VideoFrame.timestamp`](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/timestamp)
    pub fn timestamp(&self) -> i64 {
        self.inner.get("timestamp").as_::<i64>()
    }
}
impl VideoFrame {
    /// Getter of the `colorSpace` attribute.
    /// [`VideoFrame.colorSpace`](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/colorSpace)
    pub fn color_space(&self) -> VideoColorSpace {
        self.inner.get("colorSpace").as_::<VideoColorSpace>()
    }
}
impl VideoFrame {
    /// The metadata method.
    /// [`VideoFrame.metadata`](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/metadata)
    pub fn metadata(&self) -> VideoFrameMetadata {
        self.inner.call("metadata", &[]).as_::<VideoFrameMetadata>()
    }
}
impl VideoFrame {
    /// The allocationSize method.
    /// [`VideoFrame.allocationSize`](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/allocationSize)
    pub fn allocation_size0(&self) -> u32 {
        self.inner.call("allocationSize", &[]).as_::<u32>()
    }
    /// The allocationSize method.
    /// [`VideoFrame.allocationSize`](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/allocationSize)
    pub fn allocation_size1(&self, options: &VideoFrameCopyToOptions) -> u32 {
        self.inner
            .call("allocationSize", &[options.into()])
            .as_::<u32>()
    }
}
impl VideoFrame {
    /// The copyTo method.
    /// [`VideoFrame.copyTo`](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/copyTo)
    pub fn copy_to0(&self, destination: &Any) -> Promise<Sequence<PlaneLayout>> {
        self.inner
            .call("copyTo", &[destination.into()])
            .as_::<Promise<Sequence<PlaneLayout>>>()
    }
    /// The copyTo method.
    /// [`VideoFrame.copyTo`](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/copyTo)
    pub fn copy_to1(
        &self,
        destination: &Any,
        options: &VideoFrameCopyToOptions,
    ) -> Promise<Sequence<PlaneLayout>> {
        self.inner
            .call("copyTo", &[destination.into(), options.into()])
            .as_::<Promise<Sequence<PlaneLayout>>>()
    }
}
impl VideoFrame {
    /// The clone method.
    /// [`VideoFrame.clone`](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/clone)
    pub fn clone_(&self) -> VideoFrame {
        self.inner.call("clone", &[]).as_::<VideoFrame>()
    }
}
impl VideoFrame {
    /// The close method.
    /// [`VideoFrame.close`](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/close)
    pub fn close(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
