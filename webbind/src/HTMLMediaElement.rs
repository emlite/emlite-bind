use super::*;

/// The HTMLMediaElement class.
/// [`HTMLMediaElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLMediaElement {
    inner: HTMLElement,
}
impl FromVal for HTMLMediaElement {
    fn from_val(v: &Any) -> Self {
        HTMLMediaElement {
            inner: HTMLElement::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for HTMLMediaElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HTMLMediaElement {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HTMLMediaElement> for Any {
    fn from(s: HTMLMediaElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HTMLMediaElement> for Any {
    fn from(s: &HTMLMediaElement) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLMediaElement);

impl HTMLMediaElement {
    /// Getter of the `error` attribute.
    /// [`HTMLMediaElement.error`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/error)
    pub fn error(&self) -> MediaError {
        self.inner.get("error").as_::<MediaError>()
    }
}
impl HTMLMediaElement {
    /// Getter of the `src` attribute.
    /// [`HTMLMediaElement.src`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/src)
    pub fn src(&self) -> String {
        self.inner.get("src").as_::<String>()
    }

    /// Setter of the `src` attribute.
    /// [`HTMLMediaElement.src`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/src)
    pub fn set_src(&mut self, value: &str) {
        self.inner.set("src", value);
    }
}
impl HTMLMediaElement {
    /// Getter of the `srcObject` attribute.
    /// [`HTMLMediaElement.srcObject`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/srcObject)
    pub fn src_object(&self) -> Any {
        self.inner.get("srcObject").as_::<Any>()
    }

    /// Setter of the `srcObject` attribute.
    /// [`HTMLMediaElement.srcObject`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/srcObject)
    pub fn set_src_object(&mut self, value: &Any) {
        self.inner.set("srcObject", value);
    }
}
impl HTMLMediaElement {
    /// Getter of the `currentSrc` attribute.
    /// [`HTMLMediaElement.currentSrc`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/currentSrc)
    pub fn current_src(&self) -> String {
        self.inner.get("currentSrc").as_::<String>()
    }
}
impl HTMLMediaElement {
    /// Getter of the `crossOrigin` attribute.
    /// [`HTMLMediaElement.crossOrigin`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/crossOrigin)
    pub fn cross_origin(&self) -> String {
        self.inner.get("crossOrigin").as_::<String>()
    }

    /// Setter of the `crossOrigin` attribute.
    /// [`HTMLMediaElement.crossOrigin`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/crossOrigin)
    pub fn set_cross_origin(&mut self, value: &str) {
        self.inner.set("crossOrigin", value);
    }
}
impl HTMLMediaElement {
    /// Getter of the `networkState` attribute.
    /// [`HTMLMediaElement.networkState`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/networkState)
    pub fn network_state(&self) -> u16 {
        self.inner.get("networkState").as_::<u16>()
    }
}
impl HTMLMediaElement {
    /// Getter of the `preload` attribute.
    /// [`HTMLMediaElement.preload`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/preload)
    pub fn preload(&self) -> String {
        self.inner.get("preload").as_::<String>()
    }

    /// Setter of the `preload` attribute.
    /// [`HTMLMediaElement.preload`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/preload)
    pub fn set_preload(&mut self, value: &str) {
        self.inner.set("preload", value);
    }
}
impl HTMLMediaElement {
    /// Getter of the `buffered` attribute.
    /// [`HTMLMediaElement.buffered`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/buffered)
    pub fn buffered(&self) -> TimeRanges {
        self.inner.get("buffered").as_::<TimeRanges>()
    }
}
impl HTMLMediaElement {
    /// The load method.
    /// [`HTMLMediaElement.load`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/load)
    pub fn load(&self) -> Undefined {
        self.inner.call("load", &[]).as_::<Undefined>()
    }
}
impl HTMLMediaElement {
    /// The canPlayType method.
    /// [`HTMLMediaElement.canPlayType`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/canPlayType)
    pub fn can_play_type(&self, type_: &str) -> CanPlayTypeResult {
        self.inner
            .call("canPlayType", &[type_.into()])
            .as_::<CanPlayTypeResult>()
    }
}
impl HTMLMediaElement {
    /// Getter of the `readyState` attribute.
    /// [`HTMLMediaElement.readyState`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/readyState)
    pub fn ready_state(&self) -> u16 {
        self.inner.get("readyState").as_::<u16>()
    }
}
impl HTMLMediaElement {
    /// Getter of the `seeking` attribute.
    /// [`HTMLMediaElement.seeking`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/seeking)
    pub fn seeking(&self) -> bool {
        self.inner.get("seeking").as_::<bool>()
    }
}
impl HTMLMediaElement {
    /// Getter of the `currentTime` attribute.
    /// [`HTMLMediaElement.currentTime`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/currentTime)
    pub fn current_time(&self) -> f64 {
        self.inner.get("currentTime").as_::<f64>()
    }

    /// Setter of the `currentTime` attribute.
    /// [`HTMLMediaElement.currentTime`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/currentTime)
    pub fn set_current_time(&mut self, value: f64) {
        self.inner.set("currentTime", value);
    }
}
impl HTMLMediaElement {
    /// The fastSeek method.
    /// [`HTMLMediaElement.fastSeek`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/fastSeek)
    pub fn fast_seek(&self, time: f64) -> Undefined {
        self.inner
            .call("fastSeek", &[time.into()])
            .as_::<Undefined>()
    }
}
impl HTMLMediaElement {
    /// Getter of the `duration` attribute.
    /// [`HTMLMediaElement.duration`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/duration)
    pub fn duration(&self) -> f64 {
        self.inner.get("duration").as_::<f64>()
    }
}
impl HTMLMediaElement {
    /// The getStartDate method.
    /// [`HTMLMediaElement.getStartDate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/getStartDate)
    pub fn get_start_date(&self) -> Object {
        self.inner.call("getStartDate", &[]).as_::<Object>()
    }
}
impl HTMLMediaElement {
    /// Getter of the `paused` attribute.
    /// [`HTMLMediaElement.paused`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/paused)
    pub fn paused(&self) -> bool {
        self.inner.get("paused").as_::<bool>()
    }
}
impl HTMLMediaElement {
    /// Getter of the `defaultPlaybackRate` attribute.
    /// [`HTMLMediaElement.defaultPlaybackRate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/defaultPlaybackRate)
    pub fn default_playback_rate(&self) -> f64 {
        self.inner.get("defaultPlaybackRate").as_::<f64>()
    }

    /// Setter of the `defaultPlaybackRate` attribute.
    /// [`HTMLMediaElement.defaultPlaybackRate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/defaultPlaybackRate)
    pub fn set_default_playback_rate(&mut self, value: f64) {
        self.inner.set("defaultPlaybackRate", value);
    }
}
impl HTMLMediaElement {
    /// Getter of the `playbackRate` attribute.
    /// [`HTMLMediaElement.playbackRate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/playbackRate)
    pub fn playback_rate(&self) -> f64 {
        self.inner.get("playbackRate").as_::<f64>()
    }

    /// Setter of the `playbackRate` attribute.
    /// [`HTMLMediaElement.playbackRate`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/playbackRate)
    pub fn set_playback_rate(&mut self, value: f64) {
        self.inner.set("playbackRate", value);
    }
}
impl HTMLMediaElement {
    /// Getter of the `preservesPitch` attribute.
    /// [`HTMLMediaElement.preservesPitch`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/preservesPitch)
    pub fn preserves_pitch(&self) -> bool {
        self.inner.get("preservesPitch").as_::<bool>()
    }

    /// Setter of the `preservesPitch` attribute.
    /// [`HTMLMediaElement.preservesPitch`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/preservesPitch)
    pub fn set_preserves_pitch(&mut self, value: bool) {
        self.inner.set("preservesPitch", value);
    }
}
impl HTMLMediaElement {
    /// Getter of the `played` attribute.
    /// [`HTMLMediaElement.played`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/played)
    pub fn played(&self) -> TimeRanges {
        self.inner.get("played").as_::<TimeRanges>()
    }
}
impl HTMLMediaElement {
    /// Getter of the `seekable` attribute.
    /// [`HTMLMediaElement.seekable`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/seekable)
    pub fn seekable(&self) -> TimeRanges {
        self.inner.get("seekable").as_::<TimeRanges>()
    }
}
impl HTMLMediaElement {
    /// Getter of the `ended` attribute.
    /// [`HTMLMediaElement.ended`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/ended)
    pub fn ended(&self) -> bool {
        self.inner.get("ended").as_::<bool>()
    }
}
impl HTMLMediaElement {
    /// Getter of the `autoplay` attribute.
    /// [`HTMLMediaElement.autoplay`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/autoplay)
    pub fn autoplay(&self) -> bool {
        self.inner.get("autoplay").as_::<bool>()
    }

    /// Setter of the `autoplay` attribute.
    /// [`HTMLMediaElement.autoplay`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/autoplay)
    pub fn set_autoplay(&mut self, value: bool) {
        self.inner.set("autoplay", value);
    }
}
impl HTMLMediaElement {
    /// Getter of the `loop` attribute.
    /// [`HTMLMediaElement.loop`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loop)
    pub fn loop_(&self) -> bool {
        self.inner.get("loop").as_::<bool>()
    }

    /// Setter of the `loop` attribute.
    /// [`HTMLMediaElement.loop`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loop)
    pub fn set_loop_(&mut self, value: bool) {
        self.inner.set("loop", value);
    }
}
impl HTMLMediaElement {
    /// The play method.
    /// [`HTMLMediaElement.play`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/play)
    pub fn play(&self) -> Promise {
        self.inner.call("play", &[]).as_::<Promise>()
    }
}
impl HTMLMediaElement {
    /// The pause method.
    /// [`HTMLMediaElement.pause`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/pause)
    pub fn pause(&self) -> Undefined {
        self.inner.call("pause", &[]).as_::<Undefined>()
    }
}
impl HTMLMediaElement {
    /// Getter of the `controls` attribute.
    /// [`HTMLMediaElement.controls`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/controls)
    pub fn controls(&self) -> bool {
        self.inner.get("controls").as_::<bool>()
    }

    /// Setter of the `controls` attribute.
    /// [`HTMLMediaElement.controls`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/controls)
    pub fn set_controls(&mut self, value: bool) {
        self.inner.set("controls", value);
    }
}
impl HTMLMediaElement {
    /// Getter of the `volume` attribute.
    /// [`HTMLMediaElement.volume`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/volume)
    pub fn volume(&self) -> f64 {
        self.inner.get("volume").as_::<f64>()
    }

    /// Setter of the `volume` attribute.
    /// [`HTMLMediaElement.volume`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/volume)
    pub fn set_volume(&mut self, value: f64) {
        self.inner.set("volume", value);
    }
}
impl HTMLMediaElement {
    /// Getter of the `muted` attribute.
    /// [`HTMLMediaElement.muted`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/muted)
    pub fn muted(&self) -> bool {
        self.inner.get("muted").as_::<bool>()
    }

    /// Setter of the `muted` attribute.
    /// [`HTMLMediaElement.muted`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/muted)
    pub fn set_muted(&mut self, value: bool) {
        self.inner.set("muted", value);
    }
}
impl HTMLMediaElement {
    /// Getter of the `defaultMuted` attribute.
    /// [`HTMLMediaElement.defaultMuted`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/defaultMuted)
    pub fn default_muted(&self) -> bool {
        self.inner.get("defaultMuted").as_::<bool>()
    }

    /// Setter of the `defaultMuted` attribute.
    /// [`HTMLMediaElement.defaultMuted`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/defaultMuted)
    pub fn set_default_muted(&mut self, value: bool) {
        self.inner.set("defaultMuted", value);
    }
}
impl HTMLMediaElement {
    /// Getter of the `audioTracks` attribute.
    /// [`HTMLMediaElement.audioTracks`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/audioTracks)
    pub fn audio_tracks(&self) -> AudioTrackList {
        self.inner.get("audioTracks").as_::<AudioTrackList>()
    }
}
impl HTMLMediaElement {
    /// Getter of the `videoTracks` attribute.
    /// [`HTMLMediaElement.videoTracks`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/videoTracks)
    pub fn video_tracks(&self) -> VideoTrackList {
        self.inner.get("videoTracks").as_::<VideoTrackList>()
    }
}
impl HTMLMediaElement {
    /// Getter of the `textTracks` attribute.
    /// [`HTMLMediaElement.textTracks`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/textTracks)
    pub fn text_tracks(&self) -> TextTrackList {
        self.inner.get("textTracks").as_::<TextTrackList>()
    }
}
impl HTMLMediaElement {
    /// The addTextTrack method.
    /// [`HTMLMediaElement.addTextTrack`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/addTextTrack)
    pub fn add_text_track0(&self, kind: &TextTrackKind) -> TextTrack {
        self.inner
            .call("addTextTrack", &[kind.into()])
            .as_::<TextTrack>()
    }
    /// The addTextTrack method.
    /// [`HTMLMediaElement.addTextTrack`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/addTextTrack)
    pub fn add_text_track1(&self, kind: &TextTrackKind, label: &str) -> TextTrack {
        self.inner
            .call("addTextTrack", &[kind.into(), label.into()])
            .as_::<TextTrack>()
    }
    /// The addTextTrack method.
    /// [`HTMLMediaElement.addTextTrack`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/addTextTrack)
    pub fn add_text_track2(&self, kind: &TextTrackKind, label: &str, language: &str) -> TextTrack {
        self.inner
            .call(
                "addTextTrack",
                &[kind.into(), label.into(), language.into()],
            )
            .as_::<TextTrack>()
    }
}
impl HTMLMediaElement {
    /// Getter of the `sinkId` attribute.
    /// [`HTMLMediaElement.sinkId`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/sinkId)
    pub fn sink_id(&self) -> String {
        self.inner.get("sinkId").as_::<String>()
    }
}
impl HTMLMediaElement {
    /// The setSinkId method.
    /// [`HTMLMediaElement.setSinkId`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/setSinkId)
    pub fn set_sink_id(&self, sink_id: &str) -> Promise {
        self.inner
            .call("setSinkId", &[sink_id.into()])
            .as_::<Promise>()
    }
}
impl HTMLMediaElement {
    /// Getter of the `mediaKeys` attribute.
    /// [`HTMLMediaElement.mediaKeys`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/mediaKeys)
    pub fn media_keys(&self) -> MediaKeys {
        self.inner.get("mediaKeys").as_::<MediaKeys>()
    }
}
impl HTMLMediaElement {
    /// Getter of the `onencrypted` attribute.
    /// [`HTMLMediaElement.onencrypted`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/onencrypted)
    pub fn onencrypted(&self) -> Any {
        self.inner.get("onencrypted").as_::<Any>()
    }

    /// Setter of the `onencrypted` attribute.
    /// [`HTMLMediaElement.onencrypted`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/onencrypted)
    pub fn set_onencrypted(&mut self, value: &Any) {
        self.inner.set("onencrypted", value);
    }
}
impl HTMLMediaElement {
    /// Getter of the `onwaitingforkey` attribute.
    /// [`HTMLMediaElement.onwaitingforkey`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/onwaitingforkey)
    pub fn onwaitingforkey(&self) -> Any {
        self.inner.get("onwaitingforkey").as_::<Any>()
    }

    /// Setter of the `onwaitingforkey` attribute.
    /// [`HTMLMediaElement.onwaitingforkey`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/onwaitingforkey)
    pub fn set_onwaitingforkey(&mut self, value: &Any) {
        self.inner.set("onwaitingforkey", value);
    }
}
impl HTMLMediaElement {
    /// The setMediaKeys method.
    /// [`HTMLMediaElement.setMediaKeys`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/setMediaKeys)
    pub fn set_media_keys(&self, media_keys: &MediaKeys) -> Promise {
        self.inner
            .call("setMediaKeys", &[media_keys.into()])
            .as_::<Promise>()
    }
}
impl HTMLMediaElement {
    /// The captureStream method.
    /// [`HTMLMediaElement.captureStream`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/captureStream)
    pub fn capture_stream(&self) -> MediaStream {
        self.inner.call("captureStream", &[]).as_::<MediaStream>()
    }
}
impl HTMLMediaElement {
    /// Getter of the `remote` attribute.
    /// [`HTMLMediaElement.remote`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/remote)
    pub fn remote(&self) -> RemotePlayback {
        self.inner.get("remote").as_::<RemotePlayback>()
    }
}
impl HTMLMediaElement {
    /// Getter of the `disableRemotePlayback` attribute.
    /// [`HTMLMediaElement.disableRemotePlayback`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/disableRemotePlayback)
    pub fn disable_remote_playback(&self) -> bool {
        self.inner.get("disableRemotePlayback").as_::<bool>()
    }

    /// Setter of the `disableRemotePlayback` attribute.
    /// [`HTMLMediaElement.disableRemotePlayback`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/disableRemotePlayback)
    pub fn set_disable_remote_playback(&mut self, value: bool) {
        self.inner.set("disableRemotePlayback", value);
    }
}
