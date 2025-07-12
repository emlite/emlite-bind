use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for TrustedTypePolicyOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for TrustedTypePolicyOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<TrustedTypePolicyOptions> for emlite::Val {
    fn from(s: TrustedTypePolicyOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl TrustedTypePolicyOptions {
    pub fn create_html(&self) -> jsbind::Function {
        self.inner.get("createHTML").as_::<jsbind::Function>()
    }

    pub fn set_create_html(&mut self, value: jsbind::Function) {
        self.inner.set("createHTML", value);
    }
}
impl TrustedTypePolicyOptions {
    pub fn create_script(&self) -> jsbind::Function {
        self.inner.get("createScript").as_::<jsbind::Function>()
    }

    pub fn set_create_script(&mut self, value: jsbind::Function) {
        self.inner.set("createScript", value);
    }
}
impl TrustedTypePolicyOptions {
    pub fn create_script_url(&self) -> jsbind::Function {
        self.inner.get("createScriptURL").as_::<jsbind::Function>()
    }

    pub fn set_create_script_url(&mut self, value: jsbind::Function) {
        self.inner.set("createScriptURL", value);
    }
}
#[derive(Clone, Debug)]
pub struct TrustedTypePolicyFactory {
    inner: emlite::Val,
}
impl FromVal for TrustedTypePolicyFactory {
    fn from_val(v: &emlite::Val) -> Self {
        TrustedTypePolicyFactory {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for TrustedTypePolicyFactory {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for TrustedTypePolicyFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<TrustedTypePolicyFactory> for emlite::Val {
    fn from(s: TrustedTypePolicyFactory) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl TrustedTypePolicyFactory {
    pub fn create_policy0(&self, policy_name: jsbind::DOMString) -> TrustedTypePolicy {
        self.inner
            .call("createPolicy", &[policy_name.into()])
            .as_::<TrustedTypePolicy>()
    }

    pub fn create_policy1(
        &self,
        policy_name: jsbind::DOMString,
        policy_options: TrustedTypePolicyOptions,
    ) -> TrustedTypePolicy {
        self.inner
            .call("createPolicy", &[policy_name.into(), policy_options.into()])
            .as_::<TrustedTypePolicy>()
    }
}
impl TrustedTypePolicyFactory {
    pub fn is_html(&self, value: jsbind::Any) -> bool {
        self.inner.call("isHTML", &[value.into()]).as_::<bool>()
    }
}
impl TrustedTypePolicyFactory {
    pub fn is_script(&self, value: jsbind::Any) -> bool {
        self.inner.call("isScript", &[value.into()]).as_::<bool>()
    }
}
impl TrustedTypePolicyFactory {
    pub fn is_script_url(&self, value: jsbind::Any) -> bool {
        self.inner
            .call("isScriptURL", &[value.into()])
            .as_::<bool>()
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
    pub fn get_attribute_type0(
        &self,
        tag_name: jsbind::DOMString,
        attribute: jsbind::DOMString,
    ) -> jsbind::DOMString {
        self.inner
            .call("getAttributeType", &[tag_name.into(), attribute.into()])
            .as_::<jsbind::DOMString>()
    }

    pub fn get_attribute_type1(
        &self,
        tag_name: jsbind::DOMString,
        attribute: jsbind::DOMString,
        element_ns: jsbind::DOMString,
    ) -> jsbind::DOMString {
        self.inner
            .call(
                "getAttributeType",
                &[tag_name.into(), attribute.into(), element_ns.into()],
            )
            .as_::<jsbind::DOMString>()
    }

    pub fn get_attribute_type2(
        &self,
        tag_name: jsbind::DOMString,
        attribute: jsbind::DOMString,
        element_ns: jsbind::DOMString,
        attr_ns: jsbind::DOMString,
    ) -> jsbind::DOMString {
        self.inner
            .call(
                "getAttributeType",
                &[
                    tag_name.into(),
                    attribute.into(),
                    element_ns.into(),
                    attr_ns.into(),
                ],
            )
            .as_::<jsbind::DOMString>()
    }
}
impl TrustedTypePolicyFactory {
    pub fn get_property_type0(
        &self,
        tag_name: jsbind::DOMString,
        property: jsbind::DOMString,
    ) -> jsbind::DOMString {
        self.inner
            .call("getPropertyType", &[tag_name.into(), property.into()])
            .as_::<jsbind::DOMString>()
    }

    pub fn get_property_type1(
        &self,
        tag_name: jsbind::DOMString,
        property: jsbind::DOMString,
        element_ns: jsbind::DOMString,
    ) -> jsbind::DOMString {
        self.inner
            .call(
                "getPropertyType",
                &[tag_name.into(), property.into(), element_ns.into()],
            )
            .as_::<jsbind::DOMString>()
    }
}
impl TrustedTypePolicyFactory {
    pub fn default_policy(&self) -> TrustedTypePolicy {
        self.inner.get("defaultPolicy").as_::<TrustedTypePolicy>()
    }
}
