use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VTTCue {
    inner: TextTrackCue,
}
impl FromVal for VTTCue {
    fn from_val(v: &emlite::Val) -> Self {
        VTTCue { inner: TextTrackCue::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for VTTCue {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for VTTCue {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<VTTCue> for emlite::Val {
    fn from(s: VTTCue) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(VTTCue);



impl VTTCue {
    pub fn new(start_time: f64, end_time: f64, text: DOMString) -> VTTCue {
        Self {
            inner: emlite::Val::global("VTTCue").new(&[start_time.into(), end_time.into(), text.into()]).as_::<TextTrackCue>(),
        }
    }

}
impl VTTCue {
    pub fn region(&self) -> VTTRegion {
        self.inner.get("region").as_::<VTTRegion>()
    }

    pub fn set_region(&mut self, value: VTTRegion) {
        self.inner.set("region", value);
    }

}
impl VTTCue {
    pub fn vertical(&self) -> DirectionSetting {
        self.inner.get("vertical").as_::<DirectionSetting>()
    }

    pub fn set_vertical(&mut self, value: DirectionSetting) {
        self.inner.set("vertical", value);
    }

}
impl VTTCue {
    pub fn snap_to_lines(&self) -> bool {
        self.inner.get("snapToLines").as_::<bool>()
    }

    pub fn set_snap_to_lines(&mut self, value: bool) {
        self.inner.set("snapToLines", value);
    }

}
impl VTTCue {
    pub fn line(&self) -> Any {
        self.inner.get("line").as_::<Any>()
    }

    pub fn set_line(&mut self, value: Any) {
        self.inner.set("line", value);
    }

}
impl VTTCue {
    pub fn line_align(&self) -> LineAlignSetting {
        self.inner.get("lineAlign").as_::<LineAlignSetting>()
    }

    pub fn set_line_align(&mut self, value: LineAlignSetting) {
        self.inner.set("lineAlign", value);
    }

}
impl VTTCue {
    pub fn position(&self) -> Any {
        self.inner.get("position").as_::<Any>()
    }

    pub fn set_position(&mut self, value: Any) {
        self.inner.set("position", value);
    }

}
impl VTTCue {
    pub fn position_align(&self) -> PositionAlignSetting {
        self.inner.get("positionAlign").as_::<PositionAlignSetting>()
    }

    pub fn set_position_align(&mut self, value: PositionAlignSetting) {
        self.inner.set("positionAlign", value);
    }

}
impl VTTCue {
    pub fn size(&self) -> f64 {
        self.inner.get("size").as_::<f64>()
    }

    pub fn set_size(&mut self, value: f64) {
        self.inner.set("size", value);
    }

}
impl VTTCue {
    pub fn align(&self) -> AlignSetting {
        self.inner.get("align").as_::<AlignSetting>()
    }

    pub fn set_align(&mut self, value: AlignSetting) {
        self.inner.set("align", value);
    }

}
impl VTTCue {
    pub fn text(&self) -> DOMString {
        self.inner.get("text").as_::<DOMString>()
    }

    pub fn set_text(&mut self, value: DOMString) {
        self.inner.set("text", value);
    }

}
impl VTTCue {
    pub fn get_cue_as_html(&self, ) -> DocumentFragment {
        self.inner.call("getCueAsHTML", &[]).as_::<DocumentFragment>()
    }

}
