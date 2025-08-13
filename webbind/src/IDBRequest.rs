use super::*;




/// The IDBRequest class.
/// [`IDBRequest`](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IDBRequest {
    inner: EventTarget,
}

impl FromVal for IDBRequest {
    fn from_val(v: &Any) -> Self {
        IDBRequest { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for IDBRequest {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for IDBRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for IDBRequest {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for IDBRequest {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<IDBRequest> for Any {
    fn from(s: IDBRequest) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&IDBRequest> for Any {
    fn from(s: &IDBRequest) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(IDBRequest);


impl IDBRequest {
    /// Getter of the `result` attribute.
    /// [`IDBRequest.result`](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/result)
    pub fn result(&self) -> Any {
        self.inner.get("result").as_::<Any>()
    }

}
impl IDBRequest {
    /// Getter of the `error` attribute.
    /// [`IDBRequest.error`](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/error)
    pub fn error(&self) -> DOMException {
        self.inner.get("error").as_::<DOMException>()
    }

}
impl IDBRequest {
    /// Getter of the `source` attribute.
    /// [`IDBRequest.source`](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/source)
    pub fn source(&self) -> Any {
        self.inner.get("source").as_::<Any>()
    }

}
impl IDBRequest {
    /// Getter of the `transaction` attribute.
    /// [`IDBRequest.transaction`](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/transaction)
    pub fn transaction(&self) -> IDBTransaction {
        self.inner.get("transaction").as_::<IDBTransaction>()
    }

}
impl IDBRequest {
    /// Getter of the `readyState` attribute.
    /// [`IDBRequest.readyState`](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/readyState)
    pub fn ready_state(&self) -> IDBRequestReadyState {
        self.inner.get("readyState").as_::<IDBRequestReadyState>()
    }

}
impl IDBRequest {
    /// Getter of the `onsuccess` attribute.
    /// [`IDBRequest.onsuccess`](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/onsuccess)
    pub fn onsuccess(&self) -> Any {
        self.inner.get("onsuccess").as_::<Any>()
    }

    /// Setter of the `onsuccess` attribute.
    /// [`IDBRequest.onsuccess`](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/onsuccess)
    pub fn set_onsuccess(&mut self, value: &Any) {
        self.inner.set("onsuccess", value);
    }
}
impl IDBRequest {
    /// Getter of the `onerror` attribute.
    /// [`IDBRequest.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/onerror)
    pub fn onerror(&self) -> Any {
        self.inner.get("onerror").as_::<Any>()
    }

    /// Setter of the `onerror` attribute.
    /// [`IDBRequest.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/onerror)
    pub fn set_onerror(&mut self, value: &Any) {
        self.inner.set("onerror", value);
    }
}
