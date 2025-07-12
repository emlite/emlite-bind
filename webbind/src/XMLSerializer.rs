use super::*;

#[derive(Clone, Debug)]
pub struct XMLSerializer {
    inner: emlite::Val,
}
impl FromVal for XMLSerializer {
    fn from_val(v: &emlite::Val) -> Self {
        XMLSerializer {
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
impl std::ops::Deref for XMLSerializer {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XMLSerializer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XMLSerializer> for emlite::Val {
    fn from(s: XMLSerializer) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XMLSerializer {
    pub fn new() -> XMLSerializer {
        Self {
            inner: emlite::Val::global("XMLSerializer")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }
}
impl XMLSerializer {
    pub fn serialize_to_string(&self, root: Node) -> jsbind::DOMString {
        self.inner
            .call("serializeToString", &[root.into()])
            .as_::<jsbind::DOMString>()
    }
}
