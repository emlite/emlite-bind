use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CommandEvent {
    inner: Event,
}
impl FromVal for CommandEvent {
    fn from_val(v: &emlite::Val) -> Self {
        CommandEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CommandEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CommandEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CommandEvent> for emlite::Val {
    fn from(s: CommandEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CommandEvent {
    pub fn new0(type_: jsbind::DOMString) -> CommandEvent {
        Self {
            inner: emlite::Val::global("CommandEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> CommandEvent {
        Self {
            inner: emlite::Val::global("CommandEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl CommandEvent {
    pub fn source(&self) -> Element {
        self.inner.get("source").as_::<Element>()
    }
}
impl CommandEvent {
    pub fn command(&self) -> jsbind::DOMString {
        self.inner.get("command").as_::<jsbind::DOMString>()
    }
}
