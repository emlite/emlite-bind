use super::*;

#[derive(Clone, Debug)]
pub struct LayoutEdges {
    inner: emlite::Val,
}
impl FromVal for LayoutEdges {
    fn from_val(v: &emlite::Val) -> Self {
        LayoutEdges {
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
impl std::ops::Deref for LayoutEdges {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for LayoutEdges {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<LayoutEdges> for emlite::Val {
    fn from(s: LayoutEdges) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl LayoutEdges {
    pub fn inline_start(&self) -> f64 {
        self.inner.get("inlineStart").as_::<f64>()
    }
}
impl LayoutEdges {
    pub fn inline_end(&self) -> f64 {
        self.inner.get("inlineEnd").as_::<f64>()
    }
}
impl LayoutEdges {
    pub fn block_start(&self) -> f64 {
        self.inner.get("blockStart").as_::<f64>()
    }
}
impl LayoutEdges {
    pub fn block_end(&self) -> f64 {
        self.inner.get("blockEnd").as_::<f64>()
    }
}
impl LayoutEdges {
    pub fn inline(&self) -> f64 {
        self.inner.get("inline").as_::<f64>()
    }
}
impl LayoutEdges {
    pub fn block(&self) -> f64 {
        self.inner.get("block").as_::<f64>()
    }
}
