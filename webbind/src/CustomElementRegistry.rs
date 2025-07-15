use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ElementDefinitionOptions {
    inner: emlite::Val,
}
impl FromVal for ElementDefinitionOptions {
    fn from_val(v: &emlite::Val) -> Self {
        ElementDefinitionOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ElementDefinitionOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ElementDefinitionOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ElementDefinitionOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ElementDefinitionOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ElementDefinitionOptions> for emlite::Val {
    fn from(s: ElementDefinitionOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&ElementDefinitionOptions> for emlite::Val {
    fn from(s: &ElementDefinitionOptions) -> emlite::Val {
        s.inner.clone()
    }
}

impl ElementDefinitionOptions {
    pub fn extends(&self) -> DOMString {
        self.inner.get("extends").as_::<DOMString>()
    }

    pub fn set_extends(&mut self, value: DOMString) {
        self.inner.set("extends", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CustomElementRegistry {
    inner: emlite::Val,
}
impl FromVal for CustomElementRegistry {
    fn from_val(v: &emlite::Val) -> Self {
        CustomElementRegistry {
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
impl core::ops::Deref for CustomElementRegistry {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CustomElementRegistry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CustomElementRegistry {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CustomElementRegistry {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CustomElementRegistry> for emlite::Val {
    fn from(s: CustomElementRegistry) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CustomElementRegistry> for emlite::Val {
    fn from(s: &CustomElementRegistry) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CustomElementRegistry);

impl CustomElementRegistry {
    pub fn new() -> CustomElementRegistry {
        Self {
            inner: emlite::Val::global("CustomElementRegistry")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }
}
impl CustomElementRegistry {
    pub fn define0(&self, name: DOMString, constructor: Function) -> Undefined {
        self.inner
            .call("define", &[name.into(), constructor.into()])
            .as_::<Undefined>()
    }

    pub fn define1(
        &self,
        name: DOMString,
        constructor: Function,
        options: ElementDefinitionOptions,
    ) -> Undefined {
        self.inner
            .call("define", &[name.into(), constructor.into(), options.into()])
            .as_::<Undefined>()
    }
}
impl CustomElementRegistry {
    pub fn get(&self, name: DOMString) -> Any {
        self.inner.call("get", &[name.into()]).as_::<Any>()
    }
}
impl CustomElementRegistry {
    pub fn get_name(&self, constructor: Function) -> DOMString {
        self.inner
            .call("getName", &[constructor.into()])
            .as_::<DOMString>()
    }
}
impl CustomElementRegistry {
    pub fn when_defined(&self, name: DOMString) -> Promise {
        self.inner
            .call("whenDefined", &[name.into()])
            .as_::<Promise>()
    }
}
impl CustomElementRegistry {
    pub fn upgrade(&self, root: Node) -> Undefined {
        self.inner
            .call("upgrade", &[root.into()])
            .as_::<Undefined>()
    }
}
impl CustomElementRegistry {
    pub fn initialize(&self, root: Node) -> Undefined {
        self.inner
            .call("initialize", &[root.into()])
            .as_::<Undefined>()
    }
}
