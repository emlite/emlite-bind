use super::*;

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

impl RTCIceTransport {
    /// The `new RTCIceTransport(..)` constructor, creating a new RTCIceTransport instance
    pub fn new() -> RTCIceTransport {
        Self {
            inner: Any::global("RTCIceTransport").new(&[]).as_::<EventTarget>(),
        }
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
    /// The gather method.
    /// [`RTCIceTransport.gather`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport/gather)
    pub fn gather(&self) -> Undefined {
        self.inner.call("gather", &[]).as_::<Undefined>()
    }
}
impl RTCIceTransport {
    /// The gather method.
    /// [`RTCIceTransport.gather`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport/gather)
    pub fn gather_with_options(&self, options: &RTCIceGatherOptions) -> Undefined {
        self.inner
            .call("gather", &[options.into()])
            .as_::<Undefined>()
    }
}
impl RTCIceTransport {
    /// The start method.
    /// [`RTCIceTransport.start`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport/start)
    pub fn start(&self) -> Undefined {
        self.inner.call("start", &[]).as_::<Undefined>()
    }
}
impl RTCIceTransport {
    /// The start method.
    /// [`RTCIceTransport.start`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport/start)
    pub fn start_with_remote_parameters(&self, remote_parameters: &RTCIceParameters) -> Undefined {
        self.inner
            .call("start", &[remote_parameters.into()])
            .as_::<Undefined>()
    }
}
impl RTCIceTransport {
    /// The start method.
    /// [`RTCIceTransport.start`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport/start)
    pub fn start_with_remote_parameters_and_role(
        &self,
        remote_parameters: &RTCIceParameters,
        role: &RTCIceRole,
    ) -> Undefined {
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
    pub fn add_remote_candidate(&self) -> Undefined {
        self.inner
            .call("addRemoteCandidate", &[])
            .as_::<Undefined>()
    }
}
impl RTCIceTransport {
    /// The addRemoteCandidate method.
    /// [`RTCIceTransport.addRemoteCandidate`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport/addRemoteCandidate)
    pub fn add_remote_candidate_with_remote_candidate(
        &self,
        remote_candidate: &RTCIceCandidateInit,
    ) -> Undefined {
        self.inner
            .call("addRemoteCandidate", &[remote_candidate.into()])
            .as_::<Undefined>()
    }
}
