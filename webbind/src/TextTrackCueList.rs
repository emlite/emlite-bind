use super::*;




/// The TextTrackCueList class.
/// [`TextTrackCueList`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCueList)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TextTrackCueList {
    inner: Any,
}

impl FromVal for TextTrackCueList {
    fn from_val(v: &Any) -> Self {
        TextTrackCueList { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for TextTrackCueList {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for TextTrackCueList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for TextTrackCueList {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for TextTrackCueList {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<TextTrackCueList> for Any {
    fn from(s: TextTrackCueList) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&TextTrackCueList> for Any {
    fn from(s: &TextTrackCueList) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(TextTrackCueList);


impl TextTrackCueList {
    /// Getter of the `length` attribute.
    /// [`TextTrackCueList.length`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCueList/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

}
impl TextTrackCueList {
    /// The getCueById method.
    /// [`TextTrackCueList.getCueById`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCueList/getCueById)
    pub fn get_cue_by_id(&self, id: &JsString) -> TextTrackCue {
        self.inner.call("getCueById", &[id.into(), ]).as_::<TextTrackCue>()
    }
}
