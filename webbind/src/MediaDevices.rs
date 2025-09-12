use super::*;

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
