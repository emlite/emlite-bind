use super::*;

/// The VTTCue class.
/// [`VTTCue`](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VTTCue {
    inner: TextTrackCue,
}
impl FromVal for VTTCue {
    fn from_val(v: &Any) -> Self {
        VTTCue {
            inner: TextTrackCue::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for VTTCue {
    type Target = TextTrackCue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VTTCue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for VTTCue {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for VTTCue {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<VTTCue> for Any {
    fn from(s: VTTCue) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&VTTCue> for Any {
    fn from(s: &VTTCue) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(VTTCue);

impl VTTCue {
    /// The `new VTTCue(..)` constructor, creating a new VTTCue instance
    pub fn new(start_time: f64, end_time: f64, text: &str) -> VTTCue {
        Self {
            inner: Any::global("VTTCue")
                .new(&[start_time.into(), end_time.into(), text.into()])
                .as_::<TextTrackCue>(),
        }
    }
}
impl VTTCue {
    /// Getter of the `region` attribute.
    /// [`VTTCue.region`](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/region)
    pub fn region(&self) -> VTTRegion {
        self.inner.get("region").as_::<VTTRegion>()
    }

    /// Setter of the `region` attribute.
    /// [`VTTCue.region`](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/region)
    pub fn set_region(&mut self, value: &VTTRegion) {
        self.inner.set("region", value);
    }
}
impl VTTCue {
    /// Getter of the `vertical` attribute.
    /// [`VTTCue.vertical`](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/vertical)
    pub fn vertical(&self) -> DirectionSetting {
        self.inner.get("vertical").as_::<DirectionSetting>()
    }

    /// Setter of the `vertical` attribute.
    /// [`VTTCue.vertical`](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/vertical)
    pub fn set_vertical(&mut self, value: &DirectionSetting) {
        self.inner.set("vertical", value);
    }
}
impl VTTCue {
    /// Getter of the `snapToLines` attribute.
    /// [`VTTCue.snapToLines`](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/snapToLines)
    pub fn snap_to_lines(&self) -> bool {
        self.inner.get("snapToLines").as_::<bool>()
    }

    /// Setter of the `snapToLines` attribute.
    /// [`VTTCue.snapToLines`](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/snapToLines)
    pub fn set_snap_to_lines(&mut self, value: bool) {
        self.inner.set("snapToLines", value);
    }
}
impl VTTCue {
    /// Getter of the `line` attribute.
    /// [`VTTCue.line`](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/line)
    pub fn line(&self) -> Any {
        self.inner.get("line").as_::<Any>()
    }

    /// Setter of the `line` attribute.
    /// [`VTTCue.line`](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/line)
    pub fn set_line(&mut self, value: &Any) {
        self.inner.set("line", value);
    }
}
impl VTTCue {
    /// Getter of the `lineAlign` attribute.
    /// [`VTTCue.lineAlign`](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/lineAlign)
    pub fn line_align(&self) -> LineAlignSetting {
        self.inner.get("lineAlign").as_::<LineAlignSetting>()
    }

    /// Setter of the `lineAlign` attribute.
    /// [`VTTCue.lineAlign`](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/lineAlign)
    pub fn set_line_align(&mut self, value: &LineAlignSetting) {
        self.inner.set("lineAlign", value);
    }
}
impl VTTCue {
    /// Getter of the `position` attribute.
    /// [`VTTCue.position`](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/position)
    pub fn position(&self) -> Any {
        self.inner.get("position").as_::<Any>()
    }

    /// Setter of the `position` attribute.
    /// [`VTTCue.position`](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/position)
    pub fn set_position(&mut self, value: &Any) {
        self.inner.set("position", value);
    }
}
impl VTTCue {
    /// Getter of the `positionAlign` attribute.
    /// [`VTTCue.positionAlign`](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/positionAlign)
    pub fn position_align(&self) -> PositionAlignSetting {
        self.inner
            .get("positionAlign")
            .as_::<PositionAlignSetting>()
    }

    /// Setter of the `positionAlign` attribute.
    /// [`VTTCue.positionAlign`](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/positionAlign)
    pub fn set_position_align(&mut self, value: &PositionAlignSetting) {
        self.inner.set("positionAlign", value);
    }
}
impl VTTCue {
    /// Getter of the `size` attribute.
    /// [`VTTCue.size`](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/size)
    pub fn size(&self) -> f64 {
        self.inner.get("size").as_::<f64>()
    }

    /// Setter of the `size` attribute.
    /// [`VTTCue.size`](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/size)
    pub fn set_size(&mut self, value: f64) {
        self.inner.set("size", value);
    }
}
impl VTTCue {
    /// Getter of the `align` attribute.
    /// [`VTTCue.align`](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/align)
    pub fn align(&self) -> AlignSetting {
        self.inner.get("align").as_::<AlignSetting>()
    }

    /// Setter of the `align` attribute.
    /// [`VTTCue.align`](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/align)
    pub fn set_align(&mut self, value: &AlignSetting) {
        self.inner.set("align", value);
    }
}
impl VTTCue {
    /// Getter of the `text` attribute.
    /// [`VTTCue.text`](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/text)
    pub fn text(&self) -> String {
        self.inner.get("text").as_::<String>()
    }

    /// Setter of the `text` attribute.
    /// [`VTTCue.text`](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/text)
    pub fn set_text(&mut self, value: &str) {
        self.inner.set("text", value);
    }
}
impl VTTCue {
    /// The getCueAsHTML method.
    /// [`VTTCue.getCueAsHTML`](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/getCueAsHTML)
    pub fn get_cue_as_html(&self) -> DocumentFragment {
        self.inner
            .call("getCueAsHTML", &[])
            .as_::<DocumentFragment>()
    }
}
