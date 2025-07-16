use super::*;

/// The CommandEvent class.
/// [`CommandEvent`](https://developer.mozilla.org/en-US/docs/Web/API/CommandEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CommandEvent {
    inner: Event,
}
impl FromVal for CommandEvent {
    fn from_val(v: &Any) -> Self {
        CommandEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for CommandEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CommandEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CommandEvent> for Any {
    fn from(s: CommandEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CommandEvent> for Any {
    fn from(s: &CommandEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CommandEvent);

impl CommandEvent {
    /// The `new CommandEvent(..)` constructor, creating a new CommandEvent instance
    pub fn new0(type_: &str) -> CommandEvent {
        Self {
            inner: Any::global("CommandEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new CommandEvent(..)` constructor, creating a new CommandEvent instance
    pub fn new1(type_: &str, event_init_dict: &Any) -> CommandEvent {
        Self {
            inner: Any::global("CommandEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl CommandEvent {
    /// Getter of the `source` attribute.
    /// [`CommandEvent.source`](https://developer.mozilla.org/en-US/docs/Web/API/CommandEvent/source)
    pub fn source(&self) -> Element {
        self.inner.get("source").as_::<Element>()
    }
}
impl CommandEvent {
    /// Getter of the `command` attribute.
    /// [`CommandEvent.command`](https://developer.mozilla.org/en-US/docs/Web/API/CommandEvent/command)
    pub fn command(&self) -> String {
        self.inner.get("command").as_::<String>()
    }
}
