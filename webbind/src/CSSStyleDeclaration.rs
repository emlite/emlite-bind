use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSStyleDeclaration {
    inner: emlite::Val,
}
impl FromVal for CSSStyleDeclaration {
    fn from_val(v: &emlite::Val) -> Self {
        CSSStyleDeclaration { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSStyleDeclaration {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSStyleDeclaration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSStyleDeclaration {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSStyleDeclaration {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<CSSStyleDeclaration> for emlite::Val {
    fn from(s: CSSStyleDeclaration) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CSSStyleDeclaration);


impl CSSStyleDeclaration {
    pub fn css_text(&self) -> CSSOMString {
        self.inner.get("cssText").as_::<CSSOMString>()
    }

    pub fn set_css_text(&mut self, value: CSSOMString) {
        self.inner.set("cssText", value);
    }

}
impl CSSStyleDeclaration {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

}
impl CSSStyleDeclaration {
    pub fn item(&self, index: u32) -> CSSOMString {
        self.inner.call("item", &[index.into(), ]).as_::<CSSOMString>()
    }

}
impl CSSStyleDeclaration {
    pub fn get_property_value(&self, property: CSSOMString) -> CSSOMString {
        self.inner.call("getPropertyValue", &[property.into(), ]).as_::<CSSOMString>()
    }

}
impl CSSStyleDeclaration {
    pub fn get_property_priority(&self, property: CSSOMString) -> CSSOMString {
        self.inner.call("getPropertyPriority", &[property.into(), ]).as_::<CSSOMString>()
    }

}
impl CSSStyleDeclaration {
    pub fn set_property0(&self, property: CSSOMString, value: CSSOMString) -> Undefined {
        self.inner.call("setProperty", &[property.into(), value.into(), ]).as_::<Undefined>()
    }

    pub fn set_property1(&self, property: CSSOMString, value: CSSOMString, priority: CSSOMString) -> Undefined {
        self.inner.call("setProperty", &[property.into(), value.into(), priority.into(), ]).as_::<Undefined>()
    }

}
impl CSSStyleDeclaration {
    pub fn remove_property(&self, property: CSSOMString) -> CSSOMString {
        self.inner.call("removeProperty", &[property.into(), ]).as_::<CSSOMString>()
    }

}
impl CSSStyleDeclaration {
    pub fn parent_rule(&self) -> CSSRule {
        self.inner.get("parentRule").as_::<CSSRule>()
    }

}
