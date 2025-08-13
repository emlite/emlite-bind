use super::*;




/// The SVGComponentTransferFunctionElement class.
/// [`SVGComponentTransferFunctionElement`](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGComponentTransferFunctionElement {
    inner: SVGElement,
}

impl FromVal for SVGComponentTransferFunctionElement {
    fn from_val(v: &Any) -> Self {
        SVGComponentTransferFunctionElement { inner: SVGElement::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGComponentTransferFunctionElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGComponentTransferFunctionElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGComponentTransferFunctionElement {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGComponentTransferFunctionElement {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SVGComponentTransferFunctionElement> for Any {
    fn from(s: SVGComponentTransferFunctionElement) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGComponentTransferFunctionElement> for Any {
    fn from(s: &SVGComponentTransferFunctionElement) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGComponentTransferFunctionElement);


impl SVGComponentTransferFunctionElement {
    /// Getter of the `type` attribute.
    /// [`SVGComponentTransferFunctionElement.type`](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/type)
    pub fn type_(&self) -> SVGAnimatedEnumeration {
        self.inner.get("type").as_::<SVGAnimatedEnumeration>()
    }

}
impl SVGComponentTransferFunctionElement {
    /// Getter of the `tableValues` attribute.
    /// [`SVGComponentTransferFunctionElement.tableValues`](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/tableValues)
    pub fn table_values(&self) -> SVGAnimatedNumberList {
        self.inner.get("tableValues").as_::<SVGAnimatedNumberList>()
    }

}
impl SVGComponentTransferFunctionElement {
    /// Getter of the `slope` attribute.
    /// [`SVGComponentTransferFunctionElement.slope`](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/slope)
    pub fn slope(&self) -> SVGAnimatedNumber {
        self.inner.get("slope").as_::<SVGAnimatedNumber>()
    }

}
impl SVGComponentTransferFunctionElement {
    /// Getter of the `intercept` attribute.
    /// [`SVGComponentTransferFunctionElement.intercept`](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/intercept)
    pub fn intercept(&self) -> SVGAnimatedNumber {
        self.inner.get("intercept").as_::<SVGAnimatedNumber>()
    }

}
impl SVGComponentTransferFunctionElement {
    /// Getter of the `amplitude` attribute.
    /// [`SVGComponentTransferFunctionElement.amplitude`](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/amplitude)
    pub fn amplitude(&self) -> SVGAnimatedNumber {
        self.inner.get("amplitude").as_::<SVGAnimatedNumber>()
    }

}
impl SVGComponentTransferFunctionElement {
    /// Getter of the `exponent` attribute.
    /// [`SVGComponentTransferFunctionElement.exponent`](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/exponent)
    pub fn exponent(&self) -> SVGAnimatedNumber {
        self.inner.get("exponent").as_::<SVGAnimatedNumber>()
    }

}
impl SVGComponentTransferFunctionElement {
    /// Getter of the `offset` attribute.
    /// [`SVGComponentTransferFunctionElement.offset`](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/offset)
    pub fn offset(&self) -> SVGAnimatedNumber {
        self.inner.get("offset").as_::<SVGAnimatedNumber>()
    }

}
