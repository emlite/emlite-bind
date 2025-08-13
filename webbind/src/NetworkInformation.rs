use super::*;




/// The NetworkInformation class.
/// [`NetworkInformation`](https://developer.mozilla.org/en-US/docs/Web/API/NetworkInformation)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NetworkInformation {
    inner: EventTarget,
}

impl FromVal for NetworkInformation {
    fn from_val(v: &Any) -> Self {
        NetworkInformation { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for NetworkInformation {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for NetworkInformation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for NetworkInformation {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for NetworkInformation {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<NetworkInformation> for Any {
    fn from(s: NetworkInformation) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&NetworkInformation> for Any {
    fn from(s: &NetworkInformation) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(NetworkInformation);


impl NetworkInformation {
    /// Getter of the `type` attribute.
    /// [`NetworkInformation.type`](https://developer.mozilla.org/en-US/docs/Web/API/NetworkInformation/type)
    pub fn type_(&self) -> ConnectionType {
        self.inner.get("type").as_::<ConnectionType>()
    }

}
impl NetworkInformation {
    /// Getter of the `effectiveType` attribute.
    /// [`NetworkInformation.effectiveType`](https://developer.mozilla.org/en-US/docs/Web/API/NetworkInformation/effectiveType)
    pub fn effective_type(&self) -> EffectiveConnectionType {
        self.inner.get("effectiveType").as_::<EffectiveConnectionType>()
    }

}
impl NetworkInformation {
    /// Getter of the `downlinkMax` attribute.
    /// [`NetworkInformation.downlinkMax`](https://developer.mozilla.org/en-US/docs/Web/API/NetworkInformation/downlinkMax)
    pub fn downlink_max(&self) -> Any {
        self.inner.get("downlinkMax").as_::<Any>()
    }

}
impl NetworkInformation {
    /// Getter of the `downlink` attribute.
    /// [`NetworkInformation.downlink`](https://developer.mozilla.org/en-US/docs/Web/API/NetworkInformation/downlink)
    pub fn downlink(&self) -> Any {
        self.inner.get("downlink").as_::<Any>()
    }

}
impl NetworkInformation {
    /// Getter of the `rtt` attribute.
    /// [`NetworkInformation.rtt`](https://developer.mozilla.org/en-US/docs/Web/API/NetworkInformation/rtt)
    pub fn rtt(&self) -> Any {
        self.inner.get("rtt").as_::<Any>()
    }

}
impl NetworkInformation {
    /// Getter of the `onchange` attribute.
    /// [`NetworkInformation.onchange`](https://developer.mozilla.org/en-US/docs/Web/API/NetworkInformation/onchange)
    pub fn onchange(&self) -> Any {
        self.inner.get("onchange").as_::<Any>()
    }

    /// Setter of the `onchange` attribute.
    /// [`NetworkInformation.onchange`](https://developer.mozilla.org/en-US/docs/Web/API/NetworkInformation/onchange)
    pub fn set_onchange(&mut self, value: &Any) {
        self.inner.set("onchange", value);
    }
}
impl NetworkInformation {
    /// Getter of the `saveData` attribute.
    /// [`NetworkInformation.saveData`](https://developer.mozilla.org/en-US/docs/Web/API/NetworkInformation/saveData)
    pub fn save_data(&self) -> bool {
        self.inner.get("saveData").as_::<bool>()
    }

}
