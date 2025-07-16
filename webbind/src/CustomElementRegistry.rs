use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ElementDefinitionOptions {
    inner: Any,
}
impl FromVal for ElementDefinitionOptions {
    fn from_val(v: &Any) -> Self {
        ElementDefinitionOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ElementDefinitionOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ElementDefinitionOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ElementDefinitionOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ElementDefinitionOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ElementDefinitionOptions> for Any {
    fn from(s: ElementDefinitionOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ElementDefinitionOptions> for Any {
    fn from(s: &ElementDefinitionOptions) -> Any {
        s.inner.clone()
    }
}

impl ElementDefinitionOptions {
    pub fn extends(&self) -> String {
        self.inner.get("extends").as_::<String>()
    }

    pub fn set_extends(&mut self, value: &str) {
        self.inner.set("extends", value);
    }
}
/// The CustomElementRegistry class.
/// [`CustomElementRegistry`](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CustomElementRegistry {
    inner: Any,
}
impl FromVal for CustomElementRegistry {
    fn from_val(v: &Any) -> Self {
        CustomElementRegistry {
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
impl core::ops::Deref for CustomElementRegistry {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CustomElementRegistry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CustomElementRegistry {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CustomElementRegistry {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CustomElementRegistry> for Any {
    fn from(s: CustomElementRegistry) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CustomElementRegistry> for Any {
    fn from(s: &CustomElementRegistry) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CustomElementRegistry);

impl CustomElementRegistry {
    /// The `new CustomElementRegistry(..)` constructor, creating a new CustomElementRegistry instance
    pub fn new() -> CustomElementRegistry {
        Self {
            inner: Any::global("CustomElementRegistry").new(&[]).as_::<Any>(),
        }
    }
}
impl CustomElementRegistry {
    /// The define method.
    /// [`CustomElementRegistry.define`](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/define)
    pub fn define0(&self, name: &str, constructor: &Function) -> Undefined {
        self.inner
            .call("define", &[name.into(), constructor.into()])
            .as_::<Undefined>()
    }
    /// The define method.
    /// [`CustomElementRegistry.define`](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/define)
    pub fn define1(
        &self,
        name: &str,
        constructor: &Function,
        options: &ElementDefinitionOptions,
    ) -> Undefined {
        self.inner
            .call("define", &[name.into(), constructor.into(), options.into()])
            .as_::<Undefined>()
    }
}
impl CustomElementRegistry {
    /// The get method.
    /// [`CustomElementRegistry.get`](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/get)
    pub fn get(&self, name: &str) -> Any {
        self.inner.call("get", &[name.into()]).as_::<Any>()
    }
}
impl CustomElementRegistry {
    /// The getName method.
    /// [`CustomElementRegistry.getName`](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/getName)
    pub fn get_name(&self, constructor: &Function) -> String {
        self.inner
            .call("getName", &[constructor.into()])
            .as_::<String>()
    }
}
impl CustomElementRegistry {
    /// The whenDefined method.
    /// [`CustomElementRegistry.whenDefined`](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/whenDefined)
    pub fn when_defined(&self, name: &str) -> Promise {
        self.inner
            .call("whenDefined", &[name.into()])
            .as_::<Promise>()
    }
}
impl CustomElementRegistry {
    /// The upgrade method.
    /// [`CustomElementRegistry.upgrade`](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/upgrade)
    pub fn upgrade(&self, root: &Node) -> Undefined {
        self.inner
            .call("upgrade", &[root.into()])
            .as_::<Undefined>()
    }
}
impl CustomElementRegistry {
    /// The initialize method.
    /// [`CustomElementRegistry.initialize`](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/initialize)
    pub fn initialize(&self, root: &Node) -> Undefined {
        self.inner
            .call("initialize", &[root.into()])
            .as_::<Undefined>()
    }
}
