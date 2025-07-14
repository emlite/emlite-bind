use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoFrameMetadata {
    inner: emlite::Val,
}
impl FromVal for VideoFrameMetadata {
    fn from_val(v: &emlite::Val) -> Self {
        VideoFrameMetadata { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for VideoFrameMetadata {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VideoFrameMetadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for VideoFrameMetadata {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for VideoFrameMetadata {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<VideoFrameMetadata> for emlite::Val {
    fn from(s: VideoFrameMetadata) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoFrameCopyToOptions {
    inner: emlite::Val,
}
impl FromVal for VideoFrameCopyToOptions {
    fn from_val(v: &emlite::Val) -> Self {
        VideoFrameCopyToOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for VideoFrameCopyToOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VideoFrameCopyToOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for VideoFrameCopyToOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for VideoFrameCopyToOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<VideoFrameCopyToOptions> for emlite::Val {
    fn from(s: VideoFrameCopyToOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl VideoFrameCopyToOptions {
    pub fn rect(&self) -> DOMRectInit {
        self.inner.get("rect").as_::<DOMRectInit>()
    }

    pub fn set_rect(&mut self, value: DOMRectInit) {
        self.inner.set("rect", value);
    }
}
impl VideoFrameCopyToOptions {
    pub fn layout(&self) -> jsbind::Sequence<PlaneLayout> {
        self.inner
            .get("layout")
            .as_::<jsbind::Sequence<PlaneLayout>>()
    }

    pub fn set_layout(&mut self, value: jsbind::Sequence<PlaneLayout>) {
        self.inner.set("layout", value);
    }
}
impl VideoFrameCopyToOptions {
    pub fn format(&self) -> VideoPixelFormat {
        self.inner.get("format").as_::<VideoPixelFormat>()
    }

    pub fn set_format(&mut self, value: VideoPixelFormat) {
        self.inner.set("format", value);
    }
}
impl VideoFrameCopyToOptions {
    pub fn color_space(&self) -> PredefinedColorSpace {
        self.inner.get("colorSpace").as_::<PredefinedColorSpace>()
    }

    pub fn set_color_space(&mut self, value: PredefinedColorSpace) {
        self.inner.set("colorSpace", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PlaneLayout {
    inner: emlite::Val,
}
impl FromVal for PlaneLayout {
    fn from_val(v: &emlite::Val) -> Self {
        PlaneLayout { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PlaneLayout {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PlaneLayout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PlaneLayout {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PlaneLayout {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PlaneLayout> for emlite::Val {
    fn from(s: PlaneLayout) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
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
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoFrame {
    inner: emlite::Val,
}
impl FromVal for VideoFrame {
    fn from_val(v: &emlite::Val) -> Self {
        VideoFrame {
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
impl core::ops::Deref for VideoFrame {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VideoFrame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for VideoFrame {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for VideoFrame {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<VideoFrame> for emlite::Val {
    fn from(s: VideoFrame) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(VideoFrame);

impl VideoFrame {
    pub fn new(data: jsbind::Any, init: jsbind::Any) -> VideoFrame {
        Self {
            inner: emlite::Val::global("VideoFrame")
                .new(&[data.into(), init.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl VideoFrame {
    pub fn format(&self) -> VideoPixelFormat {
        self.inner.get("format").as_::<VideoPixelFormat>()
    }
}
impl VideoFrame {
    pub fn coded_width(&self) -> u32 {
        self.inner.get("codedWidth").as_::<u32>()
    }
}
impl VideoFrame {
    pub fn coded_height(&self) -> u32 {
        self.inner.get("codedHeight").as_::<u32>()
    }
}
impl VideoFrame {
    pub fn coded_rect(&self) -> DOMRectReadOnly {
        self.inner.get("codedRect").as_::<DOMRectReadOnly>()
    }
}
impl VideoFrame {
    pub fn visible_rect(&self) -> DOMRectReadOnly {
        self.inner.get("visibleRect").as_::<DOMRectReadOnly>()
    }
}
impl VideoFrame {
    pub fn rotation(&self) -> f64 {
        self.inner.get("rotation").as_::<f64>()
    }
}
impl VideoFrame {
    pub fn flip(&self) -> bool {
        self.inner.get("flip").as_::<bool>()
    }
}
impl VideoFrame {
    pub fn display_width(&self) -> u32 {
        self.inner.get("displayWidth").as_::<u32>()
    }
}
impl VideoFrame {
    pub fn display_height(&self) -> u32 {
        self.inner.get("displayHeight").as_::<u32>()
    }
}
impl VideoFrame {
    pub fn duration(&self) -> u64 {
        self.inner.get("duration").as_::<u64>()
    }
}
impl VideoFrame {
    pub fn timestamp(&self) -> i64 {
        self.inner.get("timestamp").as_::<i64>()
    }
}
impl VideoFrame {
    pub fn color_space(&self) -> VideoColorSpace {
        self.inner.get("colorSpace").as_::<VideoColorSpace>()
    }
}
impl VideoFrame {
    pub fn metadata(&self) -> VideoFrameMetadata {
        self.inner.call("metadata", &[]).as_::<VideoFrameMetadata>()
    }
}
impl VideoFrame {
    pub fn allocation_size0(&self) -> u32 {
        self.inner.call("allocationSize", &[]).as_::<u32>()
    }

    pub fn allocation_size1(&self, options: VideoFrameCopyToOptions) -> u32 {
        self.inner
            .call("allocationSize", &[options.into()])
            .as_::<u32>()
    }
}
impl VideoFrame {
    pub fn copy_to0(&self, destination: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("copyTo", &[destination.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn copy_to1(
        &self,
        destination: jsbind::Any,
        options: VideoFrameCopyToOptions,
    ) -> jsbind::Promise {
        self.inner
            .call("copyTo", &[destination.into(), options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl VideoFrame {
    pub fn clone_(&self) -> VideoFrame {
        self.inner.call("clone", &[]).as_::<VideoFrame>()
    }
}
impl VideoFrame {
    pub fn close(&self) -> jsbind::Undefined {
        self.inner.call("close", &[]).as_::<jsbind::Undefined>()
    }
}
