use super::*;

#[derive(Clone, Debug)]
pub struct HTMLCanvasElement {
    inner: HTMLElement,
}
impl FromVal for HTMLCanvasElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLCanvasElement {
            inner: HTMLElement::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for HTMLCanvasElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HTMLCanvasElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLCanvasElement> for emlite::Val {
    fn from(s: HTMLCanvasElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLCanvasElement {
    pub fn new() -> HTMLCanvasElement {
        Self {
            inner: emlite::Val::global("HTMLCanvasElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLCanvasElement {
    pub fn width(&self) -> u32 {
        self.inner.get("width").as_::<u32>()
    }

    pub fn set_width(&mut self, value: u32) {
        self.inner.set("width", value);
    }
}
impl HTMLCanvasElement {
    pub fn height(&self) -> u32 {
        self.inner.get("height").as_::<u32>()
    }

    pub fn set_height(&mut self, value: u32) {
        self.inner.set("height", value);
    }
}
impl HTMLCanvasElement {
    pub fn get_context0(&self, context_id: jsbind::DOMString) -> jsbind::Any {
        self.inner
            .call("getContext", &[context_id.into()])
            .as_::<jsbind::Any>()
    }

    pub fn get_context1(&self, context_id: jsbind::DOMString, options: jsbind::Any) -> jsbind::Any {
        self.inner
            .call("getContext", &[context_id.into(), options.into()])
            .as_::<jsbind::Any>()
    }
}
impl HTMLCanvasElement {
    pub fn to_data_url0(&self) -> jsbind::USVString {
        self.inner.call("toDataURL", &[]).as_::<jsbind::USVString>()
    }

    pub fn to_data_url1(&self, type_: jsbind::DOMString) -> jsbind::USVString {
        self.inner
            .call("toDataURL", &[type_.into()])
            .as_::<jsbind::USVString>()
    }

    pub fn to_data_url2(
        &self,
        type_: jsbind::DOMString,
        quality: jsbind::Any,
    ) -> jsbind::USVString {
        self.inner
            .call("toDataURL", &[type_.into(), quality.into()])
            .as_::<jsbind::USVString>()
    }
}
impl HTMLCanvasElement {
    pub fn to_blob0(&self, callback: jsbind::Function) -> jsbind::Undefined {
        self.inner
            .call("toBlob", &[callback.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn to_blob1(
        &self,
        callback: jsbind::Function,
        type_: jsbind::DOMString,
    ) -> jsbind::Undefined {
        self.inner
            .call("toBlob", &[callback.into(), type_.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn to_blob2(
        &self,
        callback: jsbind::Function,
        type_: jsbind::DOMString,
        quality: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call("toBlob", &[callback.into(), type_.into(), quality.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl HTMLCanvasElement {
    pub fn transfer_control_to_offscreen(&self) -> OffscreenCanvas {
        self.inner
            .call("transferControlToOffscreen", &[])
            .as_::<OffscreenCanvas>()
    }
}
impl HTMLCanvasElement {
    pub fn capture_stream0(&self) -> MediaStream {
        self.inner.call("captureStream", &[]).as_::<MediaStream>()
    }

    pub fn capture_stream1(&self, frame_request_rate: f64) -> MediaStream {
        self.inner
            .call("captureStream", &[frame_request_rate.into()])
            .as_::<MediaStream>()
    }
}
