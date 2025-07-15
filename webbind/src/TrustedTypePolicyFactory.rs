use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TrustedTypePolicyOptions {
    inner: emlite::Val,
}
impl FromVal for TrustedTypePolicyOptions {
    fn from_val(v: &emlite::Val) -> Self {
        TrustedTypePolicyOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for TrustedTypePolicyOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TrustedTypePolicyOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for TrustedTypePolicyOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for TrustedTypePolicyOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<TrustedTypePolicyOptions> for emlite::Val {
    fn from(s: TrustedTypePolicyOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl TrustedTypePolicyOptions {
    pub fn create_html(&self) -> Function {
        self.inner.get("createHTML").as_::<Function>()
    }

    pub fn set_create_html(&mut self, value: Function) {
        self.inner.set("createHTML", value);
    }

}
impl TrustedTypePolicyOptions {
    pub fn create_script(&self) -> Function {
        self.inner.get("createScript").as_::<Function>()
    }

    pub fn set_create_script(&mut self, value: Function) {
        self.inner.set("createScript", value);
    }

}
impl TrustedTypePolicyOptions {
    pub fn create_script_url(&self) -> Function {
        self.inner.get("createScriptURL").as_::<Function>()
    }

    pub fn set_create_script_url(&mut self, value: Function) {
        self.inner.set("createScriptURL", value);
    }

}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TrustedTypePolicyFactory {
    inner: emlite::Val,
}
impl FromVal for TrustedTypePolicyFactory {
    fn from_val(v: &emlite::Val) -> Self {
        TrustedTypePolicyFactory { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for TrustedTypePolicyFactory {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TrustedTypePolicyFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for TrustedTypePolicyFactory {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for TrustedTypePolicyFactory {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<TrustedTypePolicyFactory> for emlite::Val {
    fn from(s: TrustedTypePolicyFactory) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(TrustedTypePolicyFactory);


impl TrustedTypePolicyFactory {
    pub fn create_policy0(&self, policy_name: DOMString) -> TrustedTypePolicy {
        self.inner.call("createPolicy", &[policy_name.into(), ]).as_::<TrustedTypePolicy>()
    }

    pub fn create_policy1(&self, policy_name: DOMString, policy_options: TrustedTypePolicyOptions) -> TrustedTypePolicy {
        self.inner.call("createPolicy", &[policy_name.into(), policy_options.into(), ]).as_::<TrustedTypePolicy>()
    }

}
impl TrustedTypePolicyFactory {
    pub fn is_html(&self, value: Any) -> bool {
        self.inner.call("isHTML", &[value.into(), ]).as_::<bool>()
    }

}
impl TrustedTypePolicyFactory {
    pub fn is_script(&self, value: Any) -> bool {
        self.inner.call("isScript", &[value.into(), ]).as_::<bool>()
    }

}
impl TrustedTypePolicyFactory {
    pub fn is_script_url(&self, value: Any) -> bool {
        self.inner.call("isScriptURL", &[value.into(), ]).as_::<bool>()
    }

}
impl TrustedTypePolicyFactory {
    pub fn empty_html(&self) -> TrustedHTML {
        self.inner.get("emptyHTML").as_::<TrustedHTML>()
    }

}
impl TrustedTypePolicyFactory {
    pub fn empty_script(&self) -> TrustedScript {
        self.inner.get("emptyScript").as_::<TrustedScript>()
    }

}
impl TrustedTypePolicyFactory {
    pub fn get_attribute_type0(&self, tag_name: DOMString, attribute: DOMString) -> DOMString {
        self.inner.call("getAttributeType", &[tag_name.into(), attribute.into(), ]).as_::<DOMString>()
    }

    pub fn get_attribute_type1(&self, tag_name: DOMString, attribute: DOMString, element_ns: DOMString) -> DOMString {
        self.inner.call("getAttributeType", &[tag_name.into(), attribute.into(), element_ns.into(), ]).as_::<DOMString>()
    }

    pub fn get_attribute_type2(&self, tag_name: DOMString, attribute: DOMString, element_ns: DOMString, attr_ns: DOMString) -> DOMString {
        self.inner.call("getAttributeType", &[tag_name.into(), attribute.into(), element_ns.into(), attr_ns.into(), ]).as_::<DOMString>()
    }

}
impl TrustedTypePolicyFactory {
    pub fn get_property_type0(&self, tag_name: DOMString, property: DOMString) -> DOMString {
        self.inner.call("getPropertyType", &[tag_name.into(), property.into(), ]).as_::<DOMString>()
    }

    pub fn get_property_type1(&self, tag_name: DOMString, property: DOMString, element_ns: DOMString) -> DOMString {
        self.inner.call("getPropertyType", &[tag_name.into(), property.into(), element_ns.into(), ]).as_::<DOMString>()
    }

}
impl TrustedTypePolicyFactory {
    pub fn default_policy(&self) -> TrustedTypePolicy {
        self.inner.get("defaultPolicy").as_::<TrustedTypePolicy>()
    }

}
