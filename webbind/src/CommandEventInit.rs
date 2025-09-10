use super::*;

/// The CommandEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CommandEventInit {
    inner: Any,
}

impl FromVal for CommandEventInit {
    fn from_val(v: &Any) -> Self {
        CommandEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CommandEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CommandEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CommandEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CommandEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CommandEventInit> for Any {
    fn from(s: CommandEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CommandEventInit> for Any {
    fn from(s: &CommandEventInit) -> Any {
        s.inner.clone()
    }
}

impl CommandEventInit {
    /// Getter of the `source` attribute.
    pub fn source(&self) -> Element {
        self.inner.get("source").as_::<Element>()
    }

    /// Setter of the `source` attribute.
    pub fn set_source(&mut self, value: &Element) {
        self.inner.set("source", value);
    }
}
impl CommandEventInit {
    /// Getter of the `command` attribute.
    pub fn command(&self) -> JsString {
        self.inner.get("command").as_::<JsString>()
    }

    /// Setter of the `command` attribute.
    pub fn set_command(&mut self, value: &JsString) {
        self.inner.set("command", value);
    }
}
