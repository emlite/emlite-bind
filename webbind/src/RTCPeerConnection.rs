use super::*;

/// The RTCPeerConnection class.
/// [`RTCPeerConnection`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCPeerConnection {
    inner: EventTarget,
}

impl FromVal for RTCPeerConnection {
    fn from_val(v: &Any) -> Self {
        RTCPeerConnection {
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

impl core::ops::Deref for RTCPeerConnection {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCPeerConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCPeerConnection {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCPeerConnection {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<RTCPeerConnection> for Any {
    fn from(s: RTCPeerConnection) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCPeerConnection> for Any {
    fn from(s: &RTCPeerConnection) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(RTCPeerConnection);

impl RTCPeerConnection {
    /// Getter of the `localDescription` attribute.
    /// [`RTCPeerConnection.localDescription`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/localDescription)
    pub fn local_description(&self) -> RTCSessionDescription {
        self.inner
            .get("localDescription")
            .as_::<RTCSessionDescription>()
    }
}
impl RTCPeerConnection {
    /// Getter of the `currentLocalDescription` attribute.
    /// [`RTCPeerConnection.currentLocalDescription`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/currentLocalDescription)
    pub fn current_local_description(&self) -> RTCSessionDescription {
        self.inner
            .get("currentLocalDescription")
            .as_::<RTCSessionDescription>()
    }
}
impl RTCPeerConnection {
    /// Getter of the `pendingLocalDescription` attribute.
    /// [`RTCPeerConnection.pendingLocalDescription`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/pendingLocalDescription)
    pub fn pending_local_description(&self) -> RTCSessionDescription {
        self.inner
            .get("pendingLocalDescription")
            .as_::<RTCSessionDescription>()
    }
}
impl RTCPeerConnection {
    /// Getter of the `remoteDescription` attribute.
    /// [`RTCPeerConnection.remoteDescription`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/remoteDescription)
    pub fn remote_description(&self) -> RTCSessionDescription {
        self.inner
            .get("remoteDescription")
            .as_::<RTCSessionDescription>()
    }
}
impl RTCPeerConnection {
    /// Getter of the `currentRemoteDescription` attribute.
    /// [`RTCPeerConnection.currentRemoteDescription`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/currentRemoteDescription)
    pub fn current_remote_description(&self) -> RTCSessionDescription {
        self.inner
            .get("currentRemoteDescription")
            .as_::<RTCSessionDescription>()
    }
}
impl RTCPeerConnection {
    /// Getter of the `pendingRemoteDescription` attribute.
    /// [`RTCPeerConnection.pendingRemoteDescription`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/pendingRemoteDescription)
    pub fn pending_remote_description(&self) -> RTCSessionDescription {
        self.inner
            .get("pendingRemoteDescription")
            .as_::<RTCSessionDescription>()
    }
}
impl RTCPeerConnection {
    /// Getter of the `signalingState` attribute.
    /// [`RTCPeerConnection.signalingState`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/signalingState)
    pub fn signaling_state(&self) -> RTCSignalingState {
        self.inner.get("signalingState").as_::<RTCSignalingState>()
    }
}
impl RTCPeerConnection {
    /// Getter of the `iceGatheringState` attribute.
    /// [`RTCPeerConnection.iceGatheringState`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/iceGatheringState)
    pub fn ice_gathering_state(&self) -> RTCIceGatheringState {
        self.inner
            .get("iceGatheringState")
            .as_::<RTCIceGatheringState>()
    }
}
impl RTCPeerConnection {
    /// Getter of the `iceConnectionState` attribute.
    /// [`RTCPeerConnection.iceConnectionState`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/iceConnectionState)
    pub fn ice_connection_state(&self) -> RTCIceConnectionState {
        self.inner
            .get("iceConnectionState")
            .as_::<RTCIceConnectionState>()
    }
}
impl RTCPeerConnection {
    /// Getter of the `connectionState` attribute.
    /// [`RTCPeerConnection.connectionState`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/connectionState)
    pub fn connection_state(&self) -> RTCPeerConnectionState {
        self.inner
            .get("connectionState")
            .as_::<RTCPeerConnectionState>()
    }
}
impl RTCPeerConnection {
    /// Getter of the `canTrickleIceCandidates` attribute.
    /// [`RTCPeerConnection.canTrickleIceCandidates`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/canTrickleIceCandidates)
    pub fn can_trickle_ice_candidates(&self) -> bool {
        self.inner.get("canTrickleIceCandidates").as_::<bool>()
    }
}
impl RTCPeerConnection {
    /// Getter of the `onnegotiationneeded` attribute.
    /// [`RTCPeerConnection.onnegotiationneeded`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onnegotiationneeded)
    pub fn onnegotiationneeded(&self) -> Any {
        self.inner.get("onnegotiationneeded").as_::<Any>()
    }

    /// Setter of the `onnegotiationneeded` attribute.
    /// [`RTCPeerConnection.onnegotiationneeded`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onnegotiationneeded)
    pub fn set_onnegotiationneeded(&mut self, value: &Any) {
        self.inner.set("onnegotiationneeded", value);
    }
}
impl RTCPeerConnection {
    /// Getter of the `onicecandidate` attribute.
    /// [`RTCPeerConnection.onicecandidate`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onicecandidate)
    pub fn onicecandidate(&self) -> Any {
        self.inner.get("onicecandidate").as_::<Any>()
    }

    /// Setter of the `onicecandidate` attribute.
    /// [`RTCPeerConnection.onicecandidate`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onicecandidate)
    pub fn set_onicecandidate(&mut self, value: &Any) {
        self.inner.set("onicecandidate", value);
    }
}
impl RTCPeerConnection {
    /// Getter of the `onicecandidateerror` attribute.
    /// [`RTCPeerConnection.onicecandidateerror`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onicecandidateerror)
    pub fn onicecandidateerror(&self) -> Any {
        self.inner.get("onicecandidateerror").as_::<Any>()
    }

    /// Setter of the `onicecandidateerror` attribute.
    /// [`RTCPeerConnection.onicecandidateerror`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onicecandidateerror)
    pub fn set_onicecandidateerror(&mut self, value: &Any) {
        self.inner.set("onicecandidateerror", value);
    }
}
impl RTCPeerConnection {
    /// Getter of the `onsignalingstatechange` attribute.
    /// [`RTCPeerConnection.onsignalingstatechange`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onsignalingstatechange)
    pub fn onsignalingstatechange(&self) -> Any {
        self.inner.get("onsignalingstatechange").as_::<Any>()
    }

    /// Setter of the `onsignalingstatechange` attribute.
    /// [`RTCPeerConnection.onsignalingstatechange`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onsignalingstatechange)
    pub fn set_onsignalingstatechange(&mut self, value: &Any) {
        self.inner.set("onsignalingstatechange", value);
    }
}
impl RTCPeerConnection {
    /// Getter of the `oniceconnectionstatechange` attribute.
    /// [`RTCPeerConnection.oniceconnectionstatechange`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/oniceconnectionstatechange)
    pub fn oniceconnectionstatechange(&self) -> Any {
        self.inner.get("oniceconnectionstatechange").as_::<Any>()
    }

    /// Setter of the `oniceconnectionstatechange` attribute.
    /// [`RTCPeerConnection.oniceconnectionstatechange`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/oniceconnectionstatechange)
    pub fn set_oniceconnectionstatechange(&mut self, value: &Any) {
        self.inner.set("oniceconnectionstatechange", value);
    }
}
impl RTCPeerConnection {
    /// Getter of the `onicegatheringstatechange` attribute.
    /// [`RTCPeerConnection.onicegatheringstatechange`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onicegatheringstatechange)
    pub fn onicegatheringstatechange(&self) -> Any {
        self.inner.get("onicegatheringstatechange").as_::<Any>()
    }

    /// Setter of the `onicegatheringstatechange` attribute.
    /// [`RTCPeerConnection.onicegatheringstatechange`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onicegatheringstatechange)
    pub fn set_onicegatheringstatechange(&mut self, value: &Any) {
        self.inner.set("onicegatheringstatechange", value);
    }
}
impl RTCPeerConnection {
    /// Getter of the `onconnectionstatechange` attribute.
    /// [`RTCPeerConnection.onconnectionstatechange`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onconnectionstatechange)
    pub fn onconnectionstatechange(&self) -> Any {
        self.inner.get("onconnectionstatechange").as_::<Any>()
    }

    /// Setter of the `onconnectionstatechange` attribute.
    /// [`RTCPeerConnection.onconnectionstatechange`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onconnectionstatechange)
    pub fn set_onconnectionstatechange(&mut self, value: &Any) {
        self.inner.set("onconnectionstatechange", value);
    }
}
impl RTCPeerConnection {
    /// Getter of the `peerIdentity` attribute.
    /// [`RTCPeerConnection.peerIdentity`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/peerIdentity)
    pub fn peer_identity(&self) -> Promise<RTCIdentityAssertion> {
        self.inner
            .get("peerIdentity")
            .as_::<Promise<RTCIdentityAssertion>>()
    }
}
impl RTCPeerConnection {
    /// Getter of the `idpLoginUrl` attribute.
    /// [`RTCPeerConnection.idpLoginUrl`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/idpLoginUrl)
    pub fn idp_login_url(&self) -> JsString {
        self.inner.get("idpLoginUrl").as_::<JsString>()
    }
}
impl RTCPeerConnection {
    /// Getter of the `idpErrorInfo` attribute.
    /// [`RTCPeerConnection.idpErrorInfo`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/idpErrorInfo)
    pub fn idp_error_info(&self) -> JsString {
        self.inner.get("idpErrorInfo").as_::<JsString>()
    }
}
impl RTCPeerConnection {
    /// Getter of the `ontrack` attribute.
    /// [`RTCPeerConnection.ontrack`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/ontrack)
    pub fn ontrack(&self) -> Any {
        self.inner.get("ontrack").as_::<Any>()
    }

    /// Setter of the `ontrack` attribute.
    /// [`RTCPeerConnection.ontrack`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/ontrack)
    pub fn set_ontrack(&mut self, value: &Any) {
        self.inner.set("ontrack", value);
    }
}
impl RTCPeerConnection {
    /// Getter of the `sctp` attribute.
    /// [`RTCPeerConnection.sctp`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/sctp)
    pub fn sctp(&self) -> RTCSctpTransport {
        self.inner.get("sctp").as_::<RTCSctpTransport>()
    }
}
impl RTCPeerConnection {
    /// Getter of the `ondatachannel` attribute.
    /// [`RTCPeerConnection.ondatachannel`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/ondatachannel)
    pub fn ondatachannel(&self) -> Any {
        self.inner.get("ondatachannel").as_::<Any>()
    }

    /// Setter of the `ondatachannel` attribute.
    /// [`RTCPeerConnection.ondatachannel`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/ondatachannel)
    pub fn set_ondatachannel(&mut self, value: &Any) {
        self.inner.set("ondatachannel", value);
    }
}

impl RTCPeerConnection {
    /// The `new RTCPeerConnection(..)` constructor, creating a new RTCPeerConnection instance
    pub fn new() -> RTCPeerConnection {
        Self {
            inner: Any::global("RTCPeerConnection")
                .new(&[])
                .as_::<EventTarget>(),
        }
    }
}

impl RTCPeerConnection {
    /// The `new RTCPeerConnection(..)` constructor, creating a new RTCPeerConnection instance
    pub fn new_with_configuration(configuration: &RTCConfiguration) -> RTCPeerConnection {
        Self {
            inner: Any::global("RTCPeerConnection")
                .new(&[configuration.into()])
                .as_::<EventTarget>(),
        }
    }
}

impl RTCPeerConnection {
    /// The createOffer method.
    /// [`RTCPeerConnection.createOffer`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createOffer)
    pub fn create_offer(&self) -> Promise<RTCSessionDescriptionInit> {
        self.inner
            .call("createOffer", &[])
            .as_::<Promise<RTCSessionDescriptionInit>>()
    }
}
impl RTCPeerConnection {
    /// The createOffer method.
    /// [`RTCPeerConnection.createOffer`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createOffer)
    pub fn create_offer_with_options(
        &self,
        options: &RTCOfferOptions,
    ) -> Promise<RTCSessionDescriptionInit> {
        self.inner
            .call("createOffer", &[options.into()])
            .as_::<Promise<RTCSessionDescriptionInit>>()
    }
}
impl RTCPeerConnection {
    /// The createOffer method.
    /// [`RTCPeerConnection.createOffer`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createOffer)
    pub fn create_offer_with_success_callback_and_failure_callback(
        &self,
        success_callback: &Function,
        failure_callback: &Function,
    ) -> Promise<Undefined> {
        self.inner
            .call(
                "createOffer",
                &[success_callback.into(), failure_callback.into()],
            )
            .as_::<Promise<Undefined>>()
    }
}
impl RTCPeerConnection {
    /// The createOffer method.
    /// [`RTCPeerConnection.createOffer`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createOffer)
    pub fn create_offer_with_options_2(
        &self,
        success_callback: &Function,
        failure_callback: &Function,
        options: &RTCOfferOptions,
    ) -> Promise<Undefined> {
        self.inner
            .call(
                "createOffer",
                &[
                    success_callback.into(),
                    failure_callback.into(),
                    options.into(),
                ],
            )
            .as_::<Promise<Undefined>>()
    }
}
impl RTCPeerConnection {
    /// The createAnswer method.
    /// [`RTCPeerConnection.createAnswer`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createAnswer)
    pub fn create_answer(&self) -> Promise<RTCSessionDescriptionInit> {
        self.inner
            .call("createAnswer", &[])
            .as_::<Promise<RTCSessionDescriptionInit>>()
    }
}
impl RTCPeerConnection {
    /// The createAnswer method.
    /// [`RTCPeerConnection.createAnswer`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createAnswer)
    pub fn create_answer_with_options(
        &self,
        options: &RTCAnswerOptions,
    ) -> Promise<RTCSessionDescriptionInit> {
        self.inner
            .call("createAnswer", &[options.into()])
            .as_::<Promise<RTCSessionDescriptionInit>>()
    }
}
impl RTCPeerConnection {
    /// The createAnswer method.
    /// [`RTCPeerConnection.createAnswer`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createAnswer)
    pub fn create_answer_with_success_callback_and_failure_callback(
        &self,
        success_callback: &Function,
        failure_callback: &Function,
    ) -> Promise<Undefined> {
        self.inner
            .call(
                "createAnswer",
                &[success_callback.into(), failure_callback.into()],
            )
            .as_::<Promise<Undefined>>()
    }
}
impl RTCPeerConnection {
    /// The setLocalDescription method.
    /// [`RTCPeerConnection.setLocalDescription`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setLocalDescription)
    pub fn set_local_description(&self) -> Promise<Undefined> {
        self.inner
            .call("setLocalDescription", &[])
            .as_::<Promise<Undefined>>()
    }
}
impl RTCPeerConnection {
    /// The setLocalDescription method.
    /// [`RTCPeerConnection.setLocalDescription`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setLocalDescription)
    pub fn set_local_description_with_description(
        &self,
        description: &RTCLocalSessionDescriptionInit,
    ) -> Promise<Undefined> {
        self.inner
            .call("setLocalDescription", &[description.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl RTCPeerConnection {
    /// The setLocalDescription method.
    /// [`RTCPeerConnection.setLocalDescription`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setLocalDescription)
    pub fn set_local_description_with_description_and_success_callback_and_failure_callback(
        &self,
        description: &RTCLocalSessionDescriptionInit,
        success_callback: &Function,
        failure_callback: &Function,
    ) -> Promise<Undefined> {
        self.inner
            .call(
                "setLocalDescription",
                &[
                    description.into(),
                    success_callback.into(),
                    failure_callback.into(),
                ],
            )
            .as_::<Promise<Undefined>>()
    }
}
impl RTCPeerConnection {
    /// The setRemoteDescription method.
    /// [`RTCPeerConnection.setRemoteDescription`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setRemoteDescription)
    pub fn set_remote_description(
        &self,
        description: &RTCSessionDescriptionInit,
    ) -> Promise<Undefined> {
        self.inner
            .call("setRemoteDescription", &[description.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl RTCPeerConnection {
    /// The setRemoteDescription method.
    /// [`RTCPeerConnection.setRemoteDescription`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setRemoteDescription)
    pub fn set_remote_description_with_description_and_success_callback_and_failure_callback(
        &self,
        description: &RTCSessionDescriptionInit,
        success_callback: &Function,
        failure_callback: &Function,
    ) -> Promise<Undefined> {
        self.inner
            .call(
                "setRemoteDescription",
                &[
                    description.into(),
                    success_callback.into(),
                    failure_callback.into(),
                ],
            )
            .as_::<Promise<Undefined>>()
    }
}
impl RTCPeerConnection {
    /// The addIceCandidate method.
    /// [`RTCPeerConnection.addIceCandidate`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addIceCandidate)
    pub fn add_ice_candidate(&self) -> Promise<Undefined> {
        self.inner
            .call("addIceCandidate", &[])
            .as_::<Promise<Undefined>>()
    }
}
impl RTCPeerConnection {
    /// The addIceCandidate method.
    /// [`RTCPeerConnection.addIceCandidate`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addIceCandidate)
    pub fn add_ice_candidate_with_candidate(
        &self,
        candidate: &RTCIceCandidateInit,
    ) -> Promise<Undefined> {
        self.inner
            .call("addIceCandidate", &[candidate.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl RTCPeerConnection {
    /// The addIceCandidate method.
    /// [`RTCPeerConnection.addIceCandidate`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addIceCandidate)
    pub fn add_ice_candidate_with_candidate_and_success_callback_and_failure_callback(
        &self,
        candidate: &RTCIceCandidateInit,
        success_callback: &Function,
        failure_callback: &Function,
    ) -> Promise<Undefined> {
        self.inner
            .call(
                "addIceCandidate",
                &[
                    candidate.into(),
                    success_callback.into(),
                    failure_callback.into(),
                ],
            )
            .as_::<Promise<Undefined>>()
    }
}
impl RTCPeerConnection {
    /// The restartIce method.
    /// [`RTCPeerConnection.restartIce`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/restartIce)
    pub fn restart_ice(&self) -> Undefined {
        self.inner.call("restartIce", &[]).as_::<Undefined>()
    }
}
impl RTCPeerConnection {
    /// The getConfiguration method.
    /// [`RTCPeerConnection.getConfiguration`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getConfiguration)
    pub fn get_configuration(&self) -> RTCConfiguration {
        self.inner
            .call("getConfiguration", &[])
            .as_::<RTCConfiguration>()
    }
}
impl RTCPeerConnection {
    /// The setConfiguration method.
    /// [`RTCPeerConnection.setConfiguration`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setConfiguration)
    pub fn set_configuration(&self) -> Undefined {
        self.inner.call("setConfiguration", &[]).as_::<Undefined>()
    }
}
impl RTCPeerConnection {
    /// The setConfiguration method.
    /// [`RTCPeerConnection.setConfiguration`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setConfiguration)
    pub fn set_configuration_with_configuration(
        &self,
        configuration: &RTCConfiguration,
    ) -> Undefined {
        self.inner
            .call("setConfiguration", &[configuration.into()])
            .as_::<Undefined>()
    }
}
impl RTCPeerConnection {
    /// The close method.
    /// [`RTCPeerConnection.close`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/close)
    pub fn close(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
impl RTCPeerConnection {
    /// The setIdentityProvider method.
    /// [`RTCPeerConnection.setIdentityProvider`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setIdentityProvider)
    pub fn set_identity_provider(&self, provider: &JsString) -> Undefined {
        self.inner
            .call("setIdentityProvider", &[provider.into()])
            .as_::<Undefined>()
    }
}
impl RTCPeerConnection {
    /// The setIdentityProvider method.
    /// [`RTCPeerConnection.setIdentityProvider`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setIdentityProvider)
    pub fn set_identity_provider_with_options(
        &self,
        provider: &JsString,
        options: &RTCIdentityProviderOptions,
    ) -> Undefined {
        self.inner
            .call("setIdentityProvider", &[provider.into(), options.into()])
            .as_::<Undefined>()
    }
}
impl RTCPeerConnection {
    /// The getIdentityAssertion method.
    /// [`RTCPeerConnection.getIdentityAssertion`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getIdentityAssertion)
    pub fn get_identity_assertion(&self) -> Promise<JsString> {
        self.inner
            .call("getIdentityAssertion", &[])
            .as_::<Promise<JsString>>()
    }
}
impl RTCPeerConnection {
    /// The generateCertificate method.
    /// [`RTCPeerConnection.generateCertificate`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/generateCertificate)
    pub fn generate_certificate(keygen_algorithm: &Any) -> Promise<RTCCertificate> {
        Any::global("RTCPeerConnection")
            .call("generateCertificate", &[keygen_algorithm.into()])
            .as_::<Promise<RTCCertificate>>()
    }
}
impl RTCPeerConnection {
    /// The getSenders method.
    /// [`RTCPeerConnection.getSenders`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getSenders)
    pub fn get_senders(&self) -> TypedArray<RTCRtpSender> {
        self.inner
            .call("getSenders", &[])
            .as_::<TypedArray<RTCRtpSender>>()
    }
}
impl RTCPeerConnection {
    /// The getReceivers method.
    /// [`RTCPeerConnection.getReceivers`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getReceivers)
    pub fn get_receivers(&self) -> TypedArray<RTCRtpReceiver> {
        self.inner
            .call("getReceivers", &[])
            .as_::<TypedArray<RTCRtpReceiver>>()
    }
}
impl RTCPeerConnection {
    /// The getTransceivers method.
    /// [`RTCPeerConnection.getTransceivers`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getTransceivers)
    pub fn get_transceivers(&self) -> TypedArray<RTCRtpTransceiver> {
        self.inner
            .call("getTransceivers", &[])
            .as_::<TypedArray<RTCRtpTransceiver>>()
    }
}
impl RTCPeerConnection {
    /// The addTrack method.
    /// [`RTCPeerConnection.addTrack`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)
    pub fn add_track(&self, track: &MediaStreamTrack, streams: &MediaStream) -> RTCRtpSender {
        self.inner
            .call("addTrack", &[track.into(), streams.into()])
            .as_::<RTCRtpSender>()
    }
}
impl RTCPeerConnection {
    /// The removeTrack method.
    /// [`RTCPeerConnection.removeTrack`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/removeTrack)
    pub fn remove_track(&self, sender: &RTCRtpSender) -> Undefined {
        self.inner
            .call("removeTrack", &[sender.into()])
            .as_::<Undefined>()
    }
}
impl RTCPeerConnection {
    /// The addTransceiver method.
    /// [`RTCPeerConnection.addTransceiver`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTransceiver)
    pub fn add_transceiver(&self, track_or_kind: &Any) -> RTCRtpTransceiver {
        self.inner
            .call("addTransceiver", &[track_or_kind.into()])
            .as_::<RTCRtpTransceiver>()
    }
}
impl RTCPeerConnection {
    /// The addTransceiver method.
    /// [`RTCPeerConnection.addTransceiver`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTransceiver)
    pub fn add_transceiver_with_init(
        &self,
        track_or_kind: &Any,
        init: &RTCRtpTransceiverInit,
    ) -> RTCRtpTransceiver {
        self.inner
            .call("addTransceiver", &[track_or_kind.into(), init.into()])
            .as_::<RTCRtpTransceiver>()
    }
}
impl RTCPeerConnection {
    /// The createDataChannel method.
    /// [`RTCPeerConnection.createDataChannel`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createDataChannel)
    pub fn create_data_channel(&self, label: &JsString) -> RTCDataChannel {
        self.inner
            .call("createDataChannel", &[label.into()])
            .as_::<RTCDataChannel>()
    }
}
impl RTCPeerConnection {
    /// The createDataChannel method.
    /// [`RTCPeerConnection.createDataChannel`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createDataChannel)
    pub fn create_data_channel_with_data_channel_dict(
        &self,
        label: &JsString,
        data_channel_dict: &RTCDataChannelInit,
    ) -> RTCDataChannel {
        self.inner
            .call(
                "createDataChannel",
                &[label.into(), data_channel_dict.into()],
            )
            .as_::<RTCDataChannel>()
    }
}
impl RTCPeerConnection {
    /// The getStats method.
    /// [`RTCPeerConnection.getStats`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getStats)
    pub fn get_stats(&self) -> Promise<RTCStatsReport> {
        self.inner
            .call("getStats", &[])
            .as_::<Promise<RTCStatsReport>>()
    }
}
impl RTCPeerConnection {
    /// The getStats method.
    /// [`RTCPeerConnection.getStats`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getStats)
    pub fn get_stats_with_selector(&self, selector: &MediaStreamTrack) -> Promise<RTCStatsReport> {
        self.inner
            .call("getStats", &[selector.into()])
            .as_::<Promise<RTCStatsReport>>()
    }
}
