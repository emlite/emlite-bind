use super::*;

pub fn assert0() -> jsbind::Undefined {
    emlite::Val::global("console")
        .call("assert", &[])
        .as_::<jsbind::Undefined>()
}

pub fn assert1(condition: bool) -> jsbind::Undefined {
    emlite::Val::global("console")
        .call("assert", &[condition.into()])
        .as_::<jsbind::Undefined>()
}

pub fn assert2(condition: bool, data: jsbind::Any) -> jsbind::Undefined {
    emlite::Val::global("console")
        .call("assert", &[condition.into(), data.into()])
        .as_::<jsbind::Undefined>()
}

pub fn clear() -> jsbind::Undefined {
    emlite::Val::global("console")
        .call("clear", &[])
        .as_::<jsbind::Undefined>()
}

pub fn debug(data: jsbind::Any) -> jsbind::Undefined {
    emlite::Val::global("console")
        .call("debug", &[data.into()])
        .as_::<jsbind::Undefined>()
}

pub fn error(data: jsbind::Any) -> jsbind::Undefined {
    emlite::Val::global("console")
        .call("error", &[data.into()])
        .as_::<jsbind::Undefined>()
}

pub fn info(data: jsbind::Any) -> jsbind::Undefined {
    emlite::Val::global("console")
        .call("info", &[data.into()])
        .as_::<jsbind::Undefined>()
}

pub fn log(data: jsbind::Any) -> jsbind::Undefined {
    emlite::Val::global("console")
        .call("log", &[data.into()])
        .as_::<jsbind::Undefined>()
}

pub fn table0() -> jsbind::Undefined {
    emlite::Val::global("console")
        .call("table", &[])
        .as_::<jsbind::Undefined>()
}

pub fn table1(tabular_data: jsbind::Any) -> jsbind::Undefined {
    emlite::Val::global("console")
        .call("table", &[tabular_data.into()])
        .as_::<jsbind::Undefined>()
}

pub fn table2(
    tabular_data: jsbind::Any,
    properties: jsbind::Sequence<jsbind::DOMString>,
) -> jsbind::Undefined {
    emlite::Val::global("console")
        .call("table", &[tabular_data.into(), properties.into()])
        .as_::<jsbind::Undefined>()
}

pub fn trace(data: jsbind::Any) -> jsbind::Undefined {
    emlite::Val::global("console")
        .call("trace", &[data.into()])
        .as_::<jsbind::Undefined>()
}

pub fn warn(data: jsbind::Any) -> jsbind::Undefined {
    emlite::Val::global("console")
        .call("warn", &[data.into()])
        .as_::<jsbind::Undefined>()
}

pub fn dir0() -> jsbind::Undefined {
    emlite::Val::global("console")
        .call("dir", &[])
        .as_::<jsbind::Undefined>()
}

pub fn dir1(item: jsbind::Any) -> jsbind::Undefined {
    emlite::Val::global("console")
        .call("dir", &[item.into()])
        .as_::<jsbind::Undefined>()
}

pub fn dir2(item: jsbind::Any, options: jsbind::Object) -> jsbind::Undefined {
    emlite::Val::global("console")
        .call("dir", &[item.into(), options.into()])
        .as_::<jsbind::Undefined>()
}

pub fn dirxml(data: jsbind::Any) -> jsbind::Undefined {
    emlite::Val::global("console")
        .call("dirxml", &[data.into()])
        .as_::<jsbind::Undefined>()
}

pub fn count0() -> jsbind::Undefined {
    emlite::Val::global("console")
        .call("count", &[])
        .as_::<jsbind::Undefined>()
}

pub fn count1(label: jsbind::DOMString) -> jsbind::Undefined {
    emlite::Val::global("console")
        .call("count", &[label.into()])
        .as_::<jsbind::Undefined>()
}

pub fn count_reset0() -> jsbind::Undefined {
    emlite::Val::global("console")
        .call("countReset", &[])
        .as_::<jsbind::Undefined>()
}

pub fn count_reset1(label: jsbind::DOMString) -> jsbind::Undefined {
    emlite::Val::global("console")
        .call("countReset", &[label.into()])
        .as_::<jsbind::Undefined>()
}

pub fn group(data: jsbind::Any) -> jsbind::Undefined {
    emlite::Val::global("console")
        .call("group", &[data.into()])
        .as_::<jsbind::Undefined>()
}

pub fn group_collapsed(data: jsbind::Any) -> jsbind::Undefined {
    emlite::Val::global("console")
        .call("groupCollapsed", &[data.into()])
        .as_::<jsbind::Undefined>()
}

pub fn group_end() -> jsbind::Undefined {
    emlite::Val::global("console")
        .call("groupEnd", &[])
        .as_::<jsbind::Undefined>()
}

pub fn time0() -> jsbind::Undefined {
    emlite::Val::global("console")
        .call("time", &[])
        .as_::<jsbind::Undefined>()
}

pub fn time1(label: jsbind::DOMString) -> jsbind::Undefined {
    emlite::Val::global("console")
        .call("time", &[label.into()])
        .as_::<jsbind::Undefined>()
}

pub fn time_log0() -> jsbind::Undefined {
    emlite::Val::global("console")
        .call("timeLog", &[])
        .as_::<jsbind::Undefined>()
}

pub fn time_log1(label: jsbind::DOMString) -> jsbind::Undefined {
    emlite::Val::global("console")
        .call("timeLog", &[label.into()])
        .as_::<jsbind::Undefined>()
}

pub fn time_log2(label: jsbind::DOMString, data: jsbind::Any) -> jsbind::Undefined {
    emlite::Val::global("console")
        .call("timeLog", &[label.into(), data.into()])
        .as_::<jsbind::Undefined>()
}

pub fn time_end0() -> jsbind::Undefined {
    emlite::Val::global("console")
        .call("timeEnd", &[])
        .as_::<jsbind::Undefined>()
}

pub fn time_end1(label: jsbind::DOMString) -> jsbind::Undefined {
    emlite::Val::global("console")
        .call("timeEnd", &[label.into()])
        .as_::<jsbind::Undefined>()
}
