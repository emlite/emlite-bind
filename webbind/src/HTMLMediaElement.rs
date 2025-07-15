use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLMediaElement {
    inner: HTMLElement,
}
impl FromVal for HTMLMediaElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLMediaElement { inner: HTMLElement::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HTMLMediaElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLMediaElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLMediaElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLMediaElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<HTMLMediaElement> for emlite::Val {
    fn from(s: HTMLMediaElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HTMLMediaElement);


impl HTMLMediaElement {
    pub fn error(&self) -> MediaError {
        self.inner.get("error").as_::<MediaError>()
    }

}
impl HTMLMediaElement {
    pub fn src(&self) -> USVString {
        self.inner.get("src").as_::<USVString>()
    }

    pub fn set_src(&mut self, value: USVString) {
        self.inner.set("src", value);
    }

}
impl HTMLMediaElement {
    pub fn src_object(&self) -> Any {
        self.inner.get("srcObject").as_::<Any>()
    }

    pub fn set_src_object(&mut self, value: Any) {
        self.inner.set("srcObject", value);
    }

}
impl HTMLMediaElement {
    pub fn current_src(&self) -> USVString {
        self.inner.get("currentSrc").as_::<USVString>()
    }

}
impl HTMLMediaElement {
    pub fn cross_origin(&self) -> DOMString {
        self.inner.get("crossOrigin").as_::<DOMString>()
    }

    pub fn set_cross_origin(&mut self, value: DOMString) {
        self.inner.set("crossOrigin", value);
    }

}
impl HTMLMediaElement {
    pub fn network_state(&self) -> u16 {
        self.inner.get("networkState").as_::<u16>()
    }

}
impl HTMLMediaElement {
    pub fn preload(&self) -> DOMString {
        self.inner.get("preload").as_::<DOMString>()
    }

    pub fn set_preload(&mut self, value: DOMString) {
        self.inner.set("preload", value);
    }

}
impl HTMLMediaElement {
    pub fn buffered(&self) -> TimeRanges {
        self.inner.get("buffered").as_::<TimeRanges>()
    }

}
impl HTMLMediaElement {
    pub fn load(&self, ) -> Undefined {
        self.inner.call("load", &[]).as_::<Undefined>()
    }

}
impl HTMLMediaElement {
    pub fn can_play_type(&self, type_: DOMString) -> CanPlayTypeResult {
        self.inner.call("canPlayType", &[type_.into(), ]).as_::<CanPlayTypeResult>()
    }

}
impl HTMLMediaElement {
    pub fn ready_state(&self) -> u16 {
        self.inner.get("readyState").as_::<u16>()
    }

}
impl HTMLMediaElement {
    pub fn seeking(&self) -> bool {
        self.inner.get("seeking").as_::<bool>()
    }

}
impl HTMLMediaElement {
    pub fn current_time(&self) -> f64 {
        self.inner.get("currentTime").as_::<f64>()
    }

    pub fn set_current_time(&mut self, value: f64) {
        self.inner.set("currentTime", value);
    }

}
impl HTMLMediaElement {
    pub fn fast_seek(&self, time: f64) -> Undefined {
        self.inner.call("fastSeek", &[time.into(), ]).as_::<Undefined>()
    }

}
impl HTMLMediaElement {
    pub fn duration(&self) -> f64 {
        self.inner.get("duration").as_::<f64>()
    }

}
impl HTMLMediaElement {
    pub fn get_start_date(&self, ) -> Object {
        self.inner.call("getStartDate", &[]).as_::<Object>()
    }

}
impl HTMLMediaElement {
    pub fn paused(&self) -> bool {
        self.inner.get("paused").as_::<bool>()
    }

}
impl HTMLMediaElement {
    pub fn default_playback_rate(&self) -> f64 {
        self.inner.get("defaultPlaybackRate").as_::<f64>()
    }

    pub fn set_default_playback_rate(&mut self, value: f64) {
        self.inner.set("defaultPlaybackRate", value);
    }

}
impl HTMLMediaElement {
    pub fn playback_rate(&self) -> f64 {
        self.inner.get("playbackRate").as_::<f64>()
    }

    pub fn set_playback_rate(&mut self, value: f64) {
        self.inner.set("playbackRate", value);
    }

}
impl HTMLMediaElement {
    pub fn preserves_pitch(&self) -> bool {
        self.inner.get("preservesPitch").as_::<bool>()
    }

    pub fn set_preserves_pitch(&mut self, value: bool) {
        self.inner.set("preservesPitch", value);
    }

}
impl HTMLMediaElement {
    pub fn played(&self) -> TimeRanges {
        self.inner.get("played").as_::<TimeRanges>()
    }

}
impl HTMLMediaElement {
    pub fn seekable(&self) -> TimeRanges {
        self.inner.get("seekable").as_::<TimeRanges>()
    }

}
impl HTMLMediaElement {
    pub fn ended(&self) -> bool {
        self.inner.get("ended").as_::<bool>()
    }

}
impl HTMLMediaElement {
    pub fn autoplay(&self) -> bool {
        self.inner.get("autoplay").as_::<bool>()
    }

    pub fn set_autoplay(&mut self, value: bool) {
        self.inner.set("autoplay", value);
    }

}
impl HTMLMediaElement {
    pub fn loop_(&self) -> bool {
        self.inner.get("loop").as_::<bool>()
    }

    pub fn set_loop_(&mut self, value: bool) {
        self.inner.set("loop", value);
    }

}
impl HTMLMediaElement {
    pub fn play(&self, ) -> Promise {
        self.inner.call("play", &[]).as_::<Promise>()
    }

}
impl HTMLMediaElement {
    pub fn pause(&self, ) -> Undefined {
        self.inner.call("pause", &[]).as_::<Undefined>()
    }

}
impl HTMLMediaElement {
    pub fn controls(&self) -> bool {
        self.inner.get("controls").as_::<bool>()
    }

    pub fn set_controls(&mut self, value: bool) {
        self.inner.set("controls", value);
    }

}
impl HTMLMediaElement {
    pub fn volume(&self) -> f64 {
        self.inner.get("volume").as_::<f64>()
    }

    pub fn set_volume(&mut self, value: f64) {
        self.inner.set("volume", value);
    }

}
impl HTMLMediaElement {
    pub fn muted(&self) -> bool {
        self.inner.get("muted").as_::<bool>()
    }

    pub fn set_muted(&mut self, value: bool) {
        self.inner.set("muted", value);
    }

}
impl HTMLMediaElement {
    pub fn default_muted(&self) -> bool {
        self.inner.get("defaultMuted").as_::<bool>()
    }

    pub fn set_default_muted(&mut self, value: bool) {
        self.inner.set("defaultMuted", value);
    }

}
impl HTMLMediaElement {
    pub fn audio_tracks(&self) -> AudioTrackList {
        self.inner.get("audioTracks").as_::<AudioTrackList>()
    }

}
impl HTMLMediaElement {
    pub fn video_tracks(&self) -> VideoTrackList {
        self.inner.get("videoTracks").as_::<VideoTrackList>()
    }

}
impl HTMLMediaElement {
    pub fn text_tracks(&self) -> TextTrackList {
        self.inner.get("textTracks").as_::<TextTrackList>()
    }

}
impl HTMLMediaElement {
    pub fn add_text_track0(&self, kind: TextTrackKind) -> TextTrack {
        self.inner.call("addTextTrack", &[kind.into(), ]).as_::<TextTrack>()
    }

    pub fn add_text_track1(&self, kind: TextTrackKind, label: DOMString) -> TextTrack {
        self.inner.call("addTextTrack", &[kind.into(), label.into(), ]).as_::<TextTrack>()
    }

    pub fn add_text_track2(&self, kind: TextTrackKind, label: DOMString, language: DOMString) -> TextTrack {
        self.inner.call("addTextTrack", &[kind.into(), label.into(), language.into(), ]).as_::<TextTrack>()
    }

}
impl HTMLMediaElement {
    pub fn sink_id(&self) -> DOMString {
        self.inner.get("sinkId").as_::<DOMString>()
    }

}
impl HTMLMediaElement {
    pub fn set_sink_id(&self, sink_id: DOMString) -> Promise {
        self.inner.call("setSinkId", &[sink_id.into(), ]).as_::<Promise>()
    }

}
impl HTMLMediaElement {
    pub fn media_keys(&self) -> MediaKeys {
        self.inner.get("mediaKeys").as_::<MediaKeys>()
    }

}
impl HTMLMediaElement {
    pub fn onencrypted(&self) -> Any {
        self.inner.get("onencrypted").as_::<Any>()
    }

    pub fn set_onencrypted(&mut self, value: Any) {
        self.inner.set("onencrypted", value);
    }

}
impl HTMLMediaElement {
    pub fn onwaitingforkey(&self) -> Any {
        self.inner.get("onwaitingforkey").as_::<Any>()
    }

    pub fn set_onwaitingforkey(&mut self, value: Any) {
        self.inner.set("onwaitingforkey", value);
    }

}
impl HTMLMediaElement {
    pub fn set_media_keys(&self, media_keys: MediaKeys) -> Promise {
        self.inner.call("setMediaKeys", &[media_keys.into(), ]).as_::<Promise>()
    }

}
impl HTMLMediaElement {
    pub fn capture_stream(&self, ) -> MediaStream {
        self.inner.call("captureStream", &[]).as_::<MediaStream>()
    }

}
impl HTMLMediaElement {
    pub fn remote(&self) -> RemotePlayback {
        self.inner.get("remote").as_::<RemotePlayback>()
    }

}
impl HTMLMediaElement {
    pub fn disable_remote_playback(&self) -> bool {
        self.inner.get("disableRemotePlayback").as_::<bool>()
    }

    pub fn set_disable_remote_playback(&mut self, value: bool) {
        self.inner.set("disableRemotePlayback", value);
    }

}
