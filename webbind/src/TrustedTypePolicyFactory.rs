use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TrustedTypePolicyOptions {
    inner: Any,
}
impl FromVal for TrustedTypePolicyOptions {
    fn from_val(v: &Any) -> Self {
        TrustedTypePolicyOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for TrustedTypePolicyOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TrustedTypePolicyOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for TrustedTypePolicyOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for TrustedTypePolicyOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<TrustedTypePolicyOptions> for Any {
    fn from(s: TrustedTypePolicyOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&TrustedTypePolicyOptions> for Any {
    fn from(s: &TrustedTypePolicyOptions) -> Any {
        s.inner.clone()
    }
}

impl TrustedTypePolicyOptions {
    pub fn create_html(&self) -> Function {
        self.inner.get("createHTML").as_::<Function>()
    }

    pub fn set_create_html(&mut self, value: &Function) {
        self.inner.set("createHTML", value);
    }
}
impl TrustedTypePolicyOptions {
    pub fn create_script(&self) -> Function {
        self.inner.get("createScript").as_::<Function>()
    }

    pub fn set_create_script(&mut self, value: &Function) {
        self.inner.set("createScript", value);
    }
}
impl TrustedTypePolicyOptions {
    pub fn create_script_url(&self) -> Function {
        self.inner.get("createScriptURL").as_::<Function>()
    }

    pub fn set_create_script_url(&mut self, value: &Function) {
        self.inner.set("createScriptURL", value);
    }
}
/// The TrustedTypePolicyFactory class.
/// [`TrustedTypePolicyFactory`](https://developer.mozilla.org/en-US/docs/Web/API/TrustedTypePolicyFactory)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TrustedTypePolicyFactory {
    inner: Any,
}
impl FromVal for TrustedTypePolicyFactory {
    fn from_val(v: &Any) -> Self {
        TrustedTypePolicyFactory {
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
impl core::ops::Deref for TrustedTypePolicyFactory {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TrustedTypePolicyFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for TrustedTypePolicyFactory {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for TrustedTypePolicyFactory {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<TrustedTypePolicyFactory> for Any {
    fn from(s: TrustedTypePolicyFactory) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&TrustedTypePolicyFactory> for Any {
    fn from(s: &TrustedTypePolicyFactory) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(TrustedTypePolicyFactory);

impl TrustedTypePolicyFactory {
    /// The createPolicy method.
    /// [`TrustedTypePolicyFactory.createPolicy`](https://developer.mozilla.org/en-US/docs/Web/API/TrustedTypePolicyFactory/createPolicy)
    pub fn create_policy0(&self, policy_name: &str) -> TrustedTypePolicy {
        self.inner
            .call("createPolicy", &[policy_name.into()])
            .as_::<TrustedTypePolicy>()
    }
    /// The createPolicy method.
    /// [`TrustedTypePolicyFactory.createPolicy`](https://developer.mozilla.org/en-US/docs/Web/API/TrustedTypePolicyFactory/createPolicy)
    pub fn create_policy1(
        &self,
        policy_name: &str,
        policy_options: &TrustedTypePolicyOptions,
    ) -> TrustedTypePolicy {
        self.inner
            .call("createPolicy", &[policy_name.into(), policy_options.into()])
            .as_::<TrustedTypePolicy>()
    }
}
impl TrustedTypePolicyFactory {
    /// The isHTML method.
    /// [`TrustedTypePolicyFactory.isHTML`](https://developer.mozilla.org/en-US/docs/Web/API/TrustedTypePolicyFactory/isHTML)
    pub fn is_html(&self, value: &Any) -> bool {
        self.inner.call("isHTML", &[value.into()]).as_::<bool>()
    }
}
impl TrustedTypePolicyFactory {
    /// The isScript method.
    /// [`TrustedTypePolicyFactory.isScript`](https://developer.mozilla.org/en-US/docs/Web/API/TrustedTypePolicyFactory/isScript)
    pub fn is_script(&self, value: &Any) -> bool {
        self.inner.call("isScript", &[value.into()]).as_::<bool>()
    }
}
impl TrustedTypePolicyFactory {
    /// The isScriptURL method.
    /// [`TrustedTypePolicyFactory.isScriptURL`](https://developer.mozilla.org/en-US/docs/Web/API/TrustedTypePolicyFactory/isScriptURL)
    pub fn is_script_url(&self, value: &Any) -> bool {
        self.inner
            .call("isScriptURL", &[value.into()])
            .as_::<bool>()
    }
}
impl TrustedTypePolicyFactory {
    /// Getter of the `emptyHTML` attribute.
    /// [`TrustedTypePolicyFactory.emptyHTML`](https://developer.mozilla.org/en-US/docs/Web/API/TrustedTypePolicyFactory/emptyHTML)
    pub fn empty_html(&self) -> TrustedHTML {
        self.inner.get("emptyHTML").as_::<TrustedHTML>()
    }
}
impl TrustedTypePolicyFactory {
    /// Getter of the `emptyScript` attribute.
    /// [`TrustedTypePolicyFactory.emptyScript`](https://developer.mozilla.org/en-US/docs/Web/API/TrustedTypePolicyFactory/emptyScript)
    pub fn empty_script(&self) -> TrustedScript {
        self.inner.get("emptyScript").as_::<TrustedScript>()
    }
}
impl TrustedTypePolicyFactory {
    /// The getAttributeType method.
    /// [`TrustedTypePolicyFactory.getAttributeType`](https://developer.mozilla.org/en-US/docs/Web/API/TrustedTypePolicyFactory/getAttributeType)
    pub fn get_attribute_type0(&self, tag_name: &str, attribute: &str) -> String {
        self.inner
            .call("getAttributeType", &[tag_name.into(), attribute.into()])
            .as_::<String>()
    }
    /// The getAttributeType method.
    /// [`TrustedTypePolicyFactory.getAttributeType`](https://developer.mozilla.org/en-US/docs/Web/API/TrustedTypePolicyFactory/getAttributeType)
    pub fn get_attribute_type1(&self, tag_name: &str, attribute: &str, element_ns: &str) -> String {
        self.inner
            .call(
                "getAttributeType",
                &[tag_name.into(), attribute.into(), element_ns.into()],
            )
            .as_::<String>()
    }
    /// The getAttributeType method.
    /// [`TrustedTypePolicyFactory.getAttributeType`](https://developer.mozilla.org/en-US/docs/Web/API/TrustedTypePolicyFactory/getAttributeType)
    pub fn get_attribute_type2(
        &self,
        tag_name: &str,
        attribute: &str,
        element_ns: &str,
        attr_ns: &str,
    ) -> String {
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
            .as_::<String>()
    }
}
impl TrustedTypePolicyFactory {
    /// The getPropertyType method.
    /// [`TrustedTypePolicyFactory.getPropertyType`](https://developer.mozilla.org/en-US/docs/Web/API/TrustedTypePolicyFactory/getPropertyType)
    pub fn get_property_type0(&self, tag_name: &str, property: &str) -> String {
        self.inner
            .call("getPropertyType", &[tag_name.into(), property.into()])
            .as_::<String>()
    }
    /// The getPropertyType method.
    /// [`TrustedTypePolicyFactory.getPropertyType`](https://developer.mozilla.org/en-US/docs/Web/API/TrustedTypePolicyFactory/getPropertyType)
    pub fn get_property_type1(&self, tag_name: &str, property: &str, element_ns: &str) -> String {
        self.inner
            .call(
                "getPropertyType",
                &[tag_name.into(), property.into(), element_ns.into()],
            )
            .as_::<String>()
    }
}
impl TrustedTypePolicyFactory {
    /// Getter of the `defaultPolicy` attribute.
    /// [`TrustedTypePolicyFactory.defaultPolicy`](https://developer.mozilla.org/en-US/docs/Web/API/TrustedTypePolicyFactory/defaultPolicy)
    pub fn default_policy(&self) -> TrustedTypePolicy {
        self.inner.get("defaultPolicy").as_::<TrustedTypePolicy>()
    }
}
