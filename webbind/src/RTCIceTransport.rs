use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCIceParameters {
    inner: Any,
}
impl FromVal for RTCIceParameters {
    fn from_val(v: &Any) -> Self {
        RTCIceParameters { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCIceParameters {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCIceParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCIceParameters {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCIceParameters {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCIceParameters> for Any {
    fn from(s: RTCIceParameters) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCIceParameters> for Any {
    fn from(s: &RTCIceParameters) -> Any {
        s.inner.clone()
    }
}

impl RTCIceParameters {
    pub fn username_fragment(&self) -> JsString {
        self.inner.get("usernameFragment").as_::<JsString>()
    }

    pub fn set_username_fragment(&mut self, value: &JsString) {
        self.inner.set("usernameFragment", value);
    }
}
impl RTCIceParameters {
    pub fn password(&self) -> JsString {
        self.inner.get("password").as_::<JsString>()
    }

    pub fn set_password(&mut self, value: &JsString) {
        self.inner.set("password", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCIceGatherOptions {
    inner: Any,
}
impl FromVal for RTCIceGatherOptions {
    fn from_val(v: &Any) -> Self {
        RTCIceGatherOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCIceGatherOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCIceGatherOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCIceGatherOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCIceGatherOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCIceGatherOptions> for Any {
    fn from(s: RTCIceGatherOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCIceGatherOptions> for Any {
    fn from(s: &RTCIceGatherOptions) -> Any {
        s.inner.clone()
    }
}

impl RTCIceGatherOptions {
    pub fn gather_policy(&self) -> RTCIceTransportPolicy {
        self.inner
            .get("gatherPolicy")
            .as_::<RTCIceTransportPolicy>()
    }

    pub fn set_gather_policy(&mut self, value: &RTCIceTransportPolicy) {
        self.inner.set("gatherPolicy", value);
    }
}
impl RTCIceGatherOptions {
    pub fn ice_servers(&self) -> TypedArray<Any> {
        self.inner.get("iceServers").as_::<TypedArray<Any>>()
    }

    pub fn set_ice_servers(&mut self, value: &TypedArray<Any>) {
        self.inner.set("iceServers", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCIceCandidateInit {
    inner: Any,
}
impl FromVal for RTCIceCandidateInit {
    fn from_val(v: &Any) -> Self {
        RTCIceCandidateInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCIceCandidateInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCIceCandidateInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCIceCandidateInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCIceCandidateInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCIceCandidateInit> for Any {
    fn from(s: RTCIceCandidateInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCIceCandidateInit> for Any {
    fn from(s: &RTCIceCandidateInit) -> Any {
        s.inner.clone()
    }
}

impl RTCIceCandidateInit {
    pub fn candidate(&self) -> JsString {
        self.inner.get("candidate").as_::<JsString>()
    }

    pub fn set_candidate(&mut self, value: &JsString) {
        self.inner.set("candidate", value);
    }
}
impl RTCIceCandidateInit {
    pub fn sdp_mid(&self) -> JsString {
        self.inner.get("sdpMid").as_::<JsString>()
    }

    pub fn set_sdp_mid(&mut self, value: &JsString) {
        self.inner.set("sdpMid", value);
    }
}
impl RTCIceCandidateInit {
    pub fn sdp_m_line_index(&self) -> u16 {
        self.inner.get("sdpMLineIndex").as_::<u16>()
    }

    pub fn set_sdp_m_line_index(&mut self, value: u16) {
        self.inner.set("sdpMLineIndex", value);
    }
}
impl RTCIceCandidateInit {
    pub fn username_fragment(&self) -> JsString {
        self.inner.get("usernameFragment").as_::<JsString>()
    }

    pub fn set_username_fragment(&mut self, value: &JsString) {
        self.inner.set("usernameFragment", value);
    }
}
/// The RTCIceTransport class.
/// [`RTCIceTransport`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCIceTransport {
    inner: EventTarget,
}
impl FromVal for RTCIceTransport {
    fn from_val(v: &Any) -> Self {
        RTCIceTransport {
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
impl core::ops::Deref for RTCIceTransport {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCIceTransport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCIceTransport {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCIceTransport {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCIceTransport> for Any {
    fn from(s: RTCIceTransport) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCIceTransport> for Any {
    fn from(s: &RTCIceTransport) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(RTCIceTransport);

impl RTCIceTransport {
    /// Getter of the `role` attribute.
    /// [`RTCIceTransport.role`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport/role)
    pub fn role(&self) -> RTCIceRole {
        self.inner.get("role").as_::<RTCIceRole>()
    }
}
impl RTCIceTransport {
    /// Getter of the `component` attribute.
    /// [`RTCIceTransport.component`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport/component)
    pub fn component(&self) -> RTCIceComponent {
        self.inner.get("component").as_::<RTCIceComponent>()
    }
}
impl RTCIceTransport {
    /// Getter of the `state` attribute.
    /// [`RTCIceTransport.state`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport/state)
    pub fn state(&self) -> RTCIceTransportState {
        self.inner.get("state").as_::<RTCIceTransportState>()
    }
}
impl RTCIceTransport {
    /// Getter of the `gatheringState` attribute.
    /// [`RTCIceTransport.gatheringState`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport/gatheringState)
    pub fn gathering_state(&self) -> RTCIceGathererState {
        self.inner
            .get("gatheringState")
            .as_::<RTCIceGathererState>()
    }
}
impl RTCIceTransport {
    /// The getLocalCandidates method.
    /// [`RTCIceTransport.getLocalCandidates`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport/getLocalCandidates)
    pub fn get_local_candidates(&self) -> TypedArray<RTCIceCandidate> {
        self.inner
            .call("getLocalCandidates", &[])
            .as_::<TypedArray<RTCIceCandidate>>()
    }
}
impl RTCIceTransport {
    /// The getRemoteCandidates method.
    /// [`RTCIceTransport.getRemoteCandidates`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport/getRemoteCandidates)
    pub fn get_remote_candidates(&self) -> TypedArray<RTCIceCandidate> {
        self.inner
            .call("getRemoteCandidates", &[])
            .as_::<TypedArray<RTCIceCandidate>>()
    }
}
impl RTCIceTransport {
    /// The getSelectedCandidatePair method.
    /// [`RTCIceTransport.getSelectedCandidatePair`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport/getSelectedCandidatePair)
    pub fn get_selected_candidate_pair(&self) -> RTCIceCandidatePair {
        self.inner
            .call("getSelectedCandidatePair", &[])
            .as_::<RTCIceCandidatePair>()
    }
}
impl RTCIceTransport {
    /// The getLocalParameters method.
    /// [`RTCIceTransport.getLocalParameters`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport/getLocalParameters)
    pub fn get_local_parameters(&self) -> RTCIceParameters {
        self.inner
            .call("getLocalParameters", &[])
            .as_::<RTCIceParameters>()
    }
}
impl RTCIceTransport {
    /// The getRemoteParameters method.
    /// [`RTCIceTransport.getRemoteParameters`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport/getRemoteParameters)
    pub fn get_remote_parameters(&self) -> RTCIceParameters {
        self.inner
            .call("getRemoteParameters", &[])
            .as_::<RTCIceParameters>()
    }
}
impl RTCIceTransport {
    /// Getter of the `onstatechange` attribute.
    /// [`RTCIceTransport.onstatechange`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport/onstatechange)
    pub fn onstatechange(&self) -> Any {
        self.inner.get("onstatechange").as_::<Any>()
    }

    /// Setter of the `onstatechange` attribute.
    /// [`RTCIceTransport.onstatechange`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport/onstatechange)
    pub fn set_onstatechange(&mut self, value: &Any) {
        self.inner.set("onstatechange", value);
    }
}
impl RTCIceTransport {
    /// Getter of the `ongatheringstatechange` attribute.
    /// [`RTCIceTransport.ongatheringstatechange`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport/ongatheringstatechange)
    pub fn ongatheringstatechange(&self) -> Any {
        self.inner.get("ongatheringstatechange").as_::<Any>()
    }

    /// Setter of the `ongatheringstatechange` attribute.
    /// [`RTCIceTransport.ongatheringstatechange`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport/ongatheringstatechange)
    pub fn set_ongatheringstatechange(&mut self, value: &Any) {
        self.inner.set("ongatheringstatechange", value);
    }
}
impl RTCIceTransport {
    /// Getter of the `onselectedcandidatepairchange` attribute.
    /// [`RTCIceTransport.onselectedcandidatepairchange`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport/onselectedcandidatepairchange)
    pub fn onselectedcandidatepairchange(&self) -> Any {
        self.inner.get("onselectedcandidatepairchange").as_::<Any>()
    }

    /// Setter of the `onselectedcandidatepairchange` attribute.
    /// [`RTCIceTransport.onselectedcandidatepairchange`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport/onselectedcandidatepairchange)
    pub fn set_onselectedcandidatepairchange(&mut self, value: &Any) {
        self.inner.set("onselectedcandidatepairchange", value);
    }
}

impl RTCIceTransport {
    /// The `new RTCIceTransport(..)` constructor, creating a new RTCIceTransport instance
    pub fn new() -> RTCIceTransport {
        Self {
            inner: Any::global("RTCIceTransport").new(&[]).as_::<EventTarget>(),
        }
    }
}
impl RTCIceTransport {
    /// The gather method.
    /// [`RTCIceTransport.gather`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport/gather)
    pub fn gather0(&self) -> Undefined {
        self.inner.call("gather", &[]).as_::<Undefined>()
    }
    /// The gather method.
    /// [`RTCIceTransport.gather`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport/gather)
    pub fn gather1(&self, options: &RTCIceGatherOptions) -> Undefined {
        self.inner
            .call("gather", &[options.into()])
            .as_::<Undefined>()
    }
}
impl RTCIceTransport {
    /// The start method.
    /// [`RTCIceTransport.start`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport/start)
    pub fn start0(&self) -> Undefined {
        self.inner.call("start", &[]).as_::<Undefined>()
    }
    /// The start method.
    /// [`RTCIceTransport.start`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport/start)
    pub fn start1(&self, remote_parameters: &RTCIceParameters) -> Undefined {
        self.inner
            .call("start", &[remote_parameters.into()])
            .as_::<Undefined>()
    }
    /// The start method.
    /// [`RTCIceTransport.start`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport/start)
    pub fn start2(&self, remote_parameters: &RTCIceParameters, role: &RTCIceRole) -> Undefined {
        self.inner
            .call("start", &[remote_parameters.into(), role.into()])
            .as_::<Undefined>()
    }
}
impl RTCIceTransport {
    /// The stop method.
    /// [`RTCIceTransport.stop`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport/stop)
    pub fn stop(&self) -> Undefined {
        self.inner.call("stop", &[]).as_::<Undefined>()
    }
}
impl RTCIceTransport {
    /// The addRemoteCandidate method.
    /// [`RTCIceTransport.addRemoteCandidate`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport/addRemoteCandidate)
    pub fn add_remote_candidate0(&self) -> Undefined {
        self.inner
            .call("addRemoteCandidate", &[])
            .as_::<Undefined>()
    }
    /// The addRemoteCandidate method.
    /// [`RTCIceTransport.addRemoteCandidate`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport/addRemoteCandidate)
    pub fn add_remote_candidate1(&self, remote_candidate: &RTCIceCandidateInit) -> Undefined {
        self.inner
            .call("addRemoteCandidate", &[remote_candidate.into()])
            .as_::<Undefined>()
    }
}
impl RTCIceTransport {
    /// Getter of the `onerror` attribute.
    /// [`RTCIceTransport.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport/onerror)
    pub fn onerror(&self) -> Any {
        self.inner.get("onerror").as_::<Any>()
    }

    /// Setter of the `onerror` attribute.
    /// [`RTCIceTransport.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport/onerror)
    pub fn set_onerror(&mut self, value: &Any) {
        self.inner.set("onerror", value);
    }
}
impl RTCIceTransport {
    /// Getter of the `onicecandidate` attribute.
    /// [`RTCIceTransport.onicecandidate`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport/onicecandidate)
    pub fn onicecandidate(&self) -> Any {
        self.inner.get("onicecandidate").as_::<Any>()
    }

    /// Setter of the `onicecandidate` attribute.
    /// [`RTCIceTransport.onicecandidate`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport/onicecandidate)
    pub fn set_onicecandidate(&mut self, value: &Any) {
        self.inner.set("onicecandidate", value);
    }
}
