use super::*;

pub fn assert0() -> Undefined {
    Any::global("console")
        .call("assert", &[])
        .as_::<Undefined>()
}

pub fn assert1(condition: bool) -> Undefined {
    Any::global("console")
        .call("assert", &[condition.into()])
        .as_::<Undefined>()
}

pub fn assert2(condition: bool, data: &Any) -> Undefined {
    Any::global("console")
        .call("assert", &[condition.into(), data.into()])
        .as_::<Undefined>()
}

pub fn clear() -> Undefined {
    Any::global("console").call("clear", &[]).as_::<Undefined>()
}

pub fn debug(data: &Any) -> Undefined {
    Any::global("console")
        .call("debug", &[data.into()])
        .as_::<Undefined>()
}

pub fn error(data: &Any) -> Undefined {
    Any::global("console")
        .call("error", &[data.into()])
        .as_::<Undefined>()
}

pub fn info(data: &Any) -> Undefined {
    Any::global("console")
        .call("info", &[data.into()])
        .as_::<Undefined>()
}

pub fn log(data: &Any) -> Undefined {
    Any::global("console")
        .call("log", &[data.into()])
        .as_::<Undefined>()
}

pub fn table0() -> Undefined {
    Any::global("console").call("table", &[]).as_::<Undefined>()
}

pub fn table1(tabular_data: &Any) -> Undefined {
    Any::global("console")
        .call("table", &[tabular_data.into()])
        .as_::<Undefined>()
}

pub fn table2(tabular_data: &Any, properties: &TypedArray<JsString>) -> Undefined {
    Any::global("console")
        .call("table", &[tabular_data.into(), properties.into()])
        .as_::<Undefined>()
}

pub fn trace(data: &Any) -> Undefined {
    Any::global("console")
        .call("trace", &[data.into()])
        .as_::<Undefined>()
}

pub fn warn(data: &Any) -> Undefined {
    Any::global("console")
        .call("warn", &[data.into()])
        .as_::<Undefined>()
}

pub fn dir0() -> Undefined {
    Any::global("console").call("dir", &[]).as_::<Undefined>()
}

pub fn dir1(item: &Any) -> Undefined {
    Any::global("console")
        .call("dir", &[item.into()])
        .as_::<Undefined>()
}

pub fn dir2(item: &Any, options: &Object) -> Undefined {
    Any::global("console")
        .call("dir", &[item.into(), options.into()])
        .as_::<Undefined>()
}

pub fn dirxml(data: &Any) -> Undefined {
    Any::global("console")
        .call("dirxml", &[data.into()])
        .as_::<Undefined>()
}

pub fn count0() -> Undefined {
    Any::global("console").call("count", &[]).as_::<Undefined>()
}

pub fn count1(label: &JsString) -> Undefined {
    Any::global("console")
        .call("count", &[label.into()])
        .as_::<Undefined>()
}

pub fn count_reset0() -> Undefined {
    Any::global("console")
        .call("countReset", &[])
        .as_::<Undefined>()
}

pub fn count_reset1(label: &JsString) -> Undefined {
    Any::global("console")
        .call("countReset", &[label.into()])
        .as_::<Undefined>()
}

pub fn group(data: &Any) -> Undefined {
    Any::global("console")
        .call("group", &[data.into()])
        .as_::<Undefined>()
}

pub fn group_collapsed(data: &Any) -> Undefined {
    Any::global("console")
        .call("groupCollapsed", &[data.into()])
        .as_::<Undefined>()
}

pub fn group_end() -> Undefined {
    Any::global("console")
        .call("groupEnd", &[])
        .as_::<Undefined>()
}

pub fn time0() -> Undefined {
    Any::global("console").call("time", &[]).as_::<Undefined>()
}

pub fn time1(label: &JsString) -> Undefined {
    Any::global("console")
        .call("time", &[label.into()])
        .as_::<Undefined>()
}

pub fn time_log0() -> Undefined {
    Any::global("console")
        .call("timeLog", &[])
        .as_::<Undefined>()
}

pub fn time_log1(label: &JsString) -> Undefined {
    Any::global("console")
        .call("timeLog", &[label.into()])
        .as_::<Undefined>()
}

pub fn time_log2(label: &JsString, data: &Any) -> Undefined {
    Any::global("console")
        .call("timeLog", &[label.into(), data.into()])
        .as_::<Undefined>()
}

pub fn time_end0() -> Undefined {
    Any::global("console")
        .call("timeEnd", &[])
        .as_::<Undefined>()
}

pub fn time_end1(label: &JsString) -> Undefined {
    Any::global("console")
        .call("timeEnd", &[label.into()])
        .as_::<Undefined>()
}
