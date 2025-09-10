use super::*;

/// The USBInterface class.
/// [`USBInterface`](https://developer.mozilla.org/en-US/docs/Web/API/USBInterface)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct USBInterface {
    inner: Any,
}

impl FromVal for USBInterface {
    fn from_val(v: &Any) -> Self {
        USBInterface {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for USBInterface {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for USBInterface {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for USBInterface {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for USBInterface {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<USBInterface> for Any {
    fn from(s: USBInterface) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&USBInterface> for Any {
    fn from(s: &USBInterface) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(USBInterface);

impl USBInterface {
    /// The `new USBInterface(..)` constructor, creating a new USBInterface instance
    pub fn new(configuration: &USBConfiguration, interface_number: u8) -> USBInterface {
        Self {
            inner: Any::global("USBInterface")
                .new(&[configuration.into(), interface_number.into()])
                .as_::<Any>(),
        }
    }
}
impl USBInterface {
    /// Getter of the `interfaceNumber` attribute.
    /// [`USBInterface.interfaceNumber`](https://developer.mozilla.org/en-US/docs/Web/API/USBInterface/interfaceNumber)
    pub fn interface_number(&self) -> u8 {
        self.inner.get("interfaceNumber").as_::<u8>()
    }
}
impl USBInterface {
    /// Getter of the `alternate` attribute.
    /// [`USBInterface.alternate`](https://developer.mozilla.org/en-US/docs/Web/API/USBInterface/alternate)
    pub fn alternate(&self) -> USBAlternateInterface {
        self.inner.get("alternate").as_::<USBAlternateInterface>()
    }
}
impl USBInterface {
    /// Getter of the `alternates` attribute.
    /// [`USBInterface.alternates`](https://developer.mozilla.org/en-US/docs/Web/API/USBInterface/alternates)
    pub fn alternates(&self) -> TypedArray<USBAlternateInterface> {
        self.inner
            .get("alternates")
            .as_::<TypedArray<USBAlternateInterface>>()
    }
}
impl USBInterface {
    /// Getter of the `claimed` attribute.
    /// [`USBInterface.claimed`](https://developer.mozilla.org/en-US/docs/Web/API/USBInterface/claimed)
    pub fn claimed(&self) -> bool {
        self.inner.get("claimed").as_::<bool>()
    }
}
