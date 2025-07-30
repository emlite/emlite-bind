use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioOutputOptions {
    inner: Any,
}
impl FromVal for AudioOutputOptions {
    fn from_val(v: &Any) -> Self {
        AudioOutputOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AudioOutputOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AudioOutputOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AudioOutputOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AudioOutputOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AudioOutputOptions> for Any {
    fn from(s: AudioOutputOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AudioOutputOptions> for Any {
    fn from(s: &AudioOutputOptions) -> Any {
        s.inner.clone()
    }
}

impl AudioOutputOptions {
    pub fn device_id(&self) -> JsString {
        self.inner.get("deviceId").as_::<JsString>()
    }

    pub fn set_device_id(&mut self, value: &JsString) {
        self.inner.set("deviceId", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CaptureHandleConfig {
    inner: Any,
}
impl FromVal for CaptureHandleConfig {
    fn from_val(v: &Any) -> Self {
        CaptureHandleConfig { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CaptureHandleConfig {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CaptureHandleConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CaptureHandleConfig {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CaptureHandleConfig {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CaptureHandleConfig> for Any {
    fn from(s: CaptureHandleConfig) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CaptureHandleConfig> for Any {
    fn from(s: &CaptureHandleConfig) -> Any {
        s.inner.clone()
    }
}

impl CaptureHandleConfig {
    pub fn expose_origin(&self) -> bool {
        self.inner.get("exposeOrigin").as_::<bool>()
    }

    pub fn set_expose_origin(&mut self, value: bool) {
        self.inner.set("exposeOrigin", value);
    }
}
impl CaptureHandleConfig {
    pub fn handle(&self) -> JsString {
        self.inner.get("handle").as_::<JsString>()
    }

    pub fn set_handle(&mut self, value: &JsString) {
        self.inner.set("handle", value);
    }
}
impl CaptureHandleConfig {
    pub fn permitted_origins(&self) -> TypedArray<JsString> {
        self.inner
            .get("permittedOrigins")
            .as_::<TypedArray<JsString>>()
    }

    pub fn set_permitted_origins(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("permittedOrigins", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaTrackSupportedConstraints {
    inner: Any,
}
impl FromVal for MediaTrackSupportedConstraints {
    fn from_val(v: &Any) -> Self {
        MediaTrackSupportedConstraints { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MediaTrackSupportedConstraints {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaTrackSupportedConstraints {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MediaTrackSupportedConstraints {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MediaTrackSupportedConstraints {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MediaTrackSupportedConstraints> for Any {
    fn from(s: MediaTrackSupportedConstraints) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MediaTrackSupportedConstraints> for Any {
    fn from(s: &MediaTrackSupportedConstraints) -> Any {
        s.inner.clone()
    }
}

impl MediaTrackSupportedConstraints {
    pub fn display_surface(&self) -> bool {
        self.inner.get("displaySurface").as_::<bool>()
    }

    pub fn set_display_surface(&mut self, value: bool) {
        self.inner.set("displaySurface", value);
    }
}
impl MediaTrackSupportedConstraints {
    pub fn logical_surface(&self) -> bool {
        self.inner.get("logicalSurface").as_::<bool>()
    }

    pub fn set_logical_surface(&mut self, value: bool) {
        self.inner.set("logicalSurface", value);
    }
}
impl MediaTrackSupportedConstraints {
    pub fn cursor(&self) -> bool {
        self.inner.get("cursor").as_::<bool>()
    }

    pub fn set_cursor(&mut self, value: bool) {
        self.inner.set("cursor", value);
    }
}
impl MediaTrackSupportedConstraints {
    pub fn restrict_own_audio(&self) -> bool {
        self.inner.get("restrictOwnAudio").as_::<bool>()
    }

    pub fn set_restrict_own_audio(&mut self, value: bool) {
        self.inner.set("restrictOwnAudio", value);
    }
}
impl MediaTrackSupportedConstraints {
    pub fn suppress_local_audio_playback(&self) -> bool {
        self.inner.get("suppressLocalAudioPlayback").as_::<bool>()
    }

    pub fn set_suppress_local_audio_playback(&mut self, value: bool) {
        self.inner.set("suppressLocalAudioPlayback", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaStreamConstraints {
    inner: Any,
}
impl FromVal for MediaStreamConstraints {
    fn from_val(v: &Any) -> Self {
        MediaStreamConstraints { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MediaStreamConstraints {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaStreamConstraints {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MediaStreamConstraints {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MediaStreamConstraints {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MediaStreamConstraints> for Any {
    fn from(s: MediaStreamConstraints) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MediaStreamConstraints> for Any {
    fn from(s: &MediaStreamConstraints) -> Any {
        s.inner.clone()
    }
}

impl MediaStreamConstraints {
    pub fn peer_identity(&self) -> JsString {
        self.inner.get("peerIdentity").as_::<JsString>()
    }

    pub fn set_peer_identity(&mut self, value: &JsString) {
        self.inner.set("peerIdentity", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DisplayMediaStreamOptions {
    inner: Any,
}
impl FromVal for DisplayMediaStreamOptions {
    fn from_val(v: &Any) -> Self {
        DisplayMediaStreamOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for DisplayMediaStreamOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DisplayMediaStreamOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for DisplayMediaStreamOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for DisplayMediaStreamOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<DisplayMediaStreamOptions> for Any {
    fn from(s: DisplayMediaStreamOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&DisplayMediaStreamOptions> for Any {
    fn from(s: &DisplayMediaStreamOptions) -> Any {
        s.inner.clone()
    }
}

impl DisplayMediaStreamOptions {
    pub fn video(&self) -> Any {
        self.inner.get("video").as_::<Any>()
    }

    pub fn set_video(&mut self, value: &Any) {
        self.inner.set("video", value);
    }
}
impl DisplayMediaStreamOptions {
    pub fn audio(&self) -> Any {
        self.inner.get("audio").as_::<Any>()
    }

    pub fn set_audio(&mut self, value: &Any) {
        self.inner.set("audio", value);
    }
}
impl DisplayMediaStreamOptions {
    pub fn controller(&self) -> CaptureController {
        self.inner.get("controller").as_::<CaptureController>()
    }

    pub fn set_controller(&mut self, value: &CaptureController) {
        self.inner.set("controller", value);
    }
}
impl DisplayMediaStreamOptions {
    pub fn self_browser_surface(&self) -> SelfCapturePreferenceEnum {
        self.inner
            .get("selfBrowserSurface")
            .as_::<SelfCapturePreferenceEnum>()
    }

    pub fn set_self_browser_surface(&mut self, value: &SelfCapturePreferenceEnum) {
        self.inner.set("selfBrowserSurface", value);
    }
}
impl DisplayMediaStreamOptions {
    pub fn system_audio(&self) -> SystemAudioPreferenceEnum {
        self.inner
            .get("systemAudio")
            .as_::<SystemAudioPreferenceEnum>()
    }

    pub fn set_system_audio(&mut self, value: &SystemAudioPreferenceEnum) {
        self.inner.set("systemAudio", value);
    }
}
impl DisplayMediaStreamOptions {
    pub fn window_audio(&self) -> WindowAudioPreferenceEnum {
        self.inner
            .get("windowAudio")
            .as_::<WindowAudioPreferenceEnum>()
    }

    pub fn set_window_audio(&mut self, value: &WindowAudioPreferenceEnum) {
        self.inner.set("windowAudio", value);
    }
}
impl DisplayMediaStreamOptions {
    pub fn surface_switching(&self) -> SurfaceSwitchingPreferenceEnum {
        self.inner
            .get("surfaceSwitching")
            .as_::<SurfaceSwitchingPreferenceEnum>()
    }

    pub fn set_surface_switching(&mut self, value: &SurfaceSwitchingPreferenceEnum) {
        self.inner.set("surfaceSwitching", value);
    }
}
impl DisplayMediaStreamOptions {
    pub fn monitor_type_surfaces(&self) -> MonitorTypeSurfacesEnum {
        self.inner
            .get("monitorTypeSurfaces")
            .as_::<MonitorTypeSurfacesEnum>()
    }

    pub fn set_monitor_type_surfaces(&mut self, value: &MonitorTypeSurfacesEnum) {
        self.inner.set("monitorTypeSurfaces", value);
    }
}
/// The MediaDevices class.
/// [`MediaDevices`](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaDevices {
    inner: EventTarget,
}
impl FromVal for MediaDevices {
    fn from_val(v: &Any) -> Self {
        MediaDevices {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MediaDevices {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaDevices {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MediaDevices {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MediaDevices {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MediaDevices> for Any {
    fn from(s: MediaDevices) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MediaDevices> for Any {
    fn from(s: &MediaDevices) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MediaDevices);

impl MediaDevices {
    /// Getter of the `ondevicechange` attribute.
    /// [`MediaDevices.ondevicechange`](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/ondevicechange)
    pub fn ondevicechange(&self) -> Any {
        self.inner.get("ondevicechange").as_::<Any>()
    }

    /// Setter of the `ondevicechange` attribute.
    /// [`MediaDevices.ondevicechange`](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/ondevicechange)
    pub fn set_ondevicechange(&mut self, value: &Any) {
        self.inner.set("ondevicechange", value);
    }
}
impl MediaDevices {
    /// The enumerateDevices method.
    /// [`MediaDevices.enumerateDevices`](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/enumerateDevices)
    pub fn enumerate_devices(&self) -> Promise<TypedArray<MediaDeviceInfo>> {
        self.inner
            .call("enumerateDevices", &[])
            .as_::<Promise<TypedArray<MediaDeviceInfo>>>()
    }
}
impl MediaDevices {
    /// The selectAudioOutput method.
    /// [`MediaDevices.selectAudioOutput`](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/selectAudioOutput)
    pub fn select_audio_output0(&self) -> Promise<MediaDeviceInfo> {
        self.inner
            .call("selectAudioOutput", &[])
            .as_::<Promise<MediaDeviceInfo>>()
    }
    /// The selectAudioOutput method.
    /// [`MediaDevices.selectAudioOutput`](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/selectAudioOutput)
    pub fn select_audio_output1(&self, options: &AudioOutputOptions) -> Promise<MediaDeviceInfo> {
        self.inner
            .call("selectAudioOutput", &[options.into()])
            .as_::<Promise<MediaDeviceInfo>>()
    }
}
impl MediaDevices {
    /// The setCaptureHandleConfig method.
    /// [`MediaDevices.setCaptureHandleConfig`](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/setCaptureHandleConfig)
    pub fn set_capture_handle_config0(&self) -> Undefined {
        self.inner
            .call("setCaptureHandleConfig", &[])
            .as_::<Undefined>()
    }
    /// The setCaptureHandleConfig method.
    /// [`MediaDevices.setCaptureHandleConfig`](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/setCaptureHandleConfig)
    pub fn set_capture_handle_config1(&self, config: &CaptureHandleConfig) -> Undefined {
        self.inner
            .call("setCaptureHandleConfig", &[config.into()])
            .as_::<Undefined>()
    }
}
impl MediaDevices {
    /// The setSupportedCaptureActions method.
    /// [`MediaDevices.setSupportedCaptureActions`](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/setSupportedCaptureActions)
    pub fn set_supported_capture_actions(&self, actions: &TypedArray<JsString>) -> Undefined {
        self.inner
            .call("setSupportedCaptureActions", &[actions.into()])
            .as_::<Undefined>()
    }
}
impl MediaDevices {
    /// Getter of the `oncaptureaction` attribute.
    /// [`MediaDevices.oncaptureaction`](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/oncaptureaction)
    pub fn oncaptureaction(&self) -> Any {
        self.inner.get("oncaptureaction").as_::<Any>()
    }

    /// Setter of the `oncaptureaction` attribute.
    /// [`MediaDevices.oncaptureaction`](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/oncaptureaction)
    pub fn set_oncaptureaction(&mut self, value: &Any) {
        self.inner.set("oncaptureaction", value);
    }
}
impl MediaDevices {
    /// The getSupportedConstraints method.
    /// [`MediaDevices.getSupportedConstraints`](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getSupportedConstraints)
    pub fn get_supported_constraints(&self) -> MediaTrackSupportedConstraints {
        self.inner
            .call("getSupportedConstraints", &[])
            .as_::<MediaTrackSupportedConstraints>()
    }
}
impl MediaDevices {
    /// The getUserMedia method.
    /// [`MediaDevices.getUserMedia`](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getUserMedia)
    pub fn get_user_media0(&self) -> Promise<MediaStream> {
        self.inner
            .call("getUserMedia", &[])
            .as_::<Promise<MediaStream>>()
    }
    /// The getUserMedia method.
    /// [`MediaDevices.getUserMedia`](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getUserMedia)
    pub fn get_user_media1(&self, constraints: &MediaStreamConstraints) -> Promise<MediaStream> {
        self.inner
            .call("getUserMedia", &[constraints.into()])
            .as_::<Promise<MediaStream>>()
    }
}
impl MediaDevices {
    /// The getViewportMedia method.
    /// [`MediaDevices.getViewportMedia`](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getViewportMedia)
    pub fn get_viewport_media0(&self) -> Promise<MediaStream> {
        self.inner
            .call("getViewportMedia", &[])
            .as_::<Promise<MediaStream>>()
    }
    /// The getViewportMedia method.
    /// [`MediaDevices.getViewportMedia`](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getViewportMedia)
    pub fn get_viewport_media1(&self, options: &DisplayMediaStreamOptions) -> Promise<MediaStream> {
        self.inner
            .call("getViewportMedia", &[options.into()])
            .as_::<Promise<MediaStream>>()
    }
}
impl MediaDevices {
    /// The getDisplayMedia method.
    /// [`MediaDevices.getDisplayMedia`](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getDisplayMedia)
    pub fn get_display_media0(&self) -> Promise<MediaStream> {
        self.inner
            .call("getDisplayMedia", &[])
            .as_::<Promise<MediaStream>>()
    }
    /// The getDisplayMedia method.
    /// [`MediaDevices.getDisplayMedia`](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getDisplayMedia)
    pub fn get_display_media1(&self, options: &DisplayMediaStreamOptions) -> Promise<MediaStream> {
        self.inner
            .call("getDisplayMedia", &[options.into()])
            .as_::<Promise<MediaStream>>()
    }
}
