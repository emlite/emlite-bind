use super::*;

/// The assert function from the console namespace.
pub fn assert0() -> Undefined {
    Any::global("console")
        .call("assert", &[])
        .as_::<Undefined>()
}

/// The assert function from the console namespace.
pub fn assert1(condition: bool) -> Undefined {
    Any::global("console")
        .call("assert", &[condition.into()])
        .as_::<Undefined>()
}

/// The assert function from the console namespace.
pub fn assert2(condition: bool, data: &Any) -> Undefined {
    Any::global("console")
        .call("assert", &[condition.into(), data.into()])
        .as_::<Undefined>()
}

/// The clear function from the console namespace.
pub fn clear() -> Undefined {
    Any::global("console").call("clear", &[]).as_::<Undefined>()
}

/// The debug function from the console namespace.
pub fn debug(data: &Any) -> Undefined {
    Any::global("console")
        .call("debug", &[data.into()])
        .as_::<Undefined>()
}

/// The error function from the console namespace.
pub fn error(data: &Any) -> Undefined {
    Any::global("console")
        .call("error", &[data.into()])
        .as_::<Undefined>()
}

/// The info function from the console namespace.
pub fn info(data: &Any) -> Undefined {
    Any::global("console")
        .call("info", &[data.into()])
        .as_::<Undefined>()
}

/// The log function from the console namespace.
pub fn log(data: &Any) -> Undefined {
    Any::global("console")
        .call("log", &[data.into()])
        .as_::<Undefined>()
}

/// The table function from the console namespace.
pub fn table0() -> Undefined {
    Any::global("console").call("table", &[]).as_::<Undefined>()
}

/// The table function from the console namespace.
pub fn table1(tabular_data: &Any) -> Undefined {
    Any::global("console")
        .call("table", &[tabular_data.into()])
        .as_::<Undefined>()
}

/// The table function from the console namespace.
pub fn table2(tabular_data: &Any, properties: &TypedArray<JsString>) -> Undefined {
    Any::global("console")
        .call("table", &[tabular_data.into(), properties.into()])
        .as_::<Undefined>()
}

/// The trace function from the console namespace.
pub fn trace(data: &Any) -> Undefined {
    Any::global("console")
        .call("trace", &[data.into()])
        .as_::<Undefined>()
}

/// The warn function from the console namespace.
pub fn warn(data: &Any) -> Undefined {
    Any::global("console")
        .call("warn", &[data.into()])
        .as_::<Undefined>()
}

/// The dir function from the console namespace.
pub fn dir0() -> Undefined {
    Any::global("console").call("dir", &[]).as_::<Undefined>()
}

/// The dir function from the console namespace.
pub fn dir1(item: &Any) -> Undefined {
    Any::global("console")
        .call("dir", &[item.into()])
        .as_::<Undefined>()
}

/// The dir function from the console namespace.
pub fn dir2(item: &Any, options: &Object) -> Undefined {
    Any::global("console")
        .call("dir", &[item.into(), options.into()])
        .as_::<Undefined>()
}

/// The dirxml function from the console namespace.
pub fn dirxml(data: &Any) -> Undefined {
    Any::global("console")
        .call("dirxml", &[data.into()])
        .as_::<Undefined>()
}

/// The count function from the console namespace.
pub fn count0() -> Undefined {
    Any::global("console").call("count", &[]).as_::<Undefined>()
}

/// The count function from the console namespace.
pub fn count1(label: &JsString) -> Undefined {
    Any::global("console")
        .call("count", &[label.into()])
        .as_::<Undefined>()
}

/// The countReset function from the console namespace.
pub fn count_reset0() -> Undefined {
    Any::global("console")
        .call("countReset", &[])
        .as_::<Undefined>()
}

/// The countReset function from the console namespace.
pub fn count_reset1(label: &JsString) -> Undefined {
    Any::global("console")
        .call("countReset", &[label.into()])
        .as_::<Undefined>()
}

/// The group function from the console namespace.
pub fn group(data: &Any) -> Undefined {
    Any::global("console")
        .call("group", &[data.into()])
        .as_::<Undefined>()
}

/// The groupCollapsed function from the console namespace.
pub fn group_collapsed(data: &Any) -> Undefined {
    Any::global("console")
        .call("groupCollapsed", &[data.into()])
        .as_::<Undefined>()
}

/// The groupEnd function from the console namespace.
pub fn group_end() -> Undefined {
    Any::global("console")
        .call("groupEnd", &[])
        .as_::<Undefined>()
}

/// The time function from the console namespace.
pub fn time0() -> Undefined {
    Any::global("console").call("time", &[]).as_::<Undefined>()
}

/// The time function from the console namespace.
pub fn time1(label: &JsString) -> Undefined {
    Any::global("console")
        .call("time", &[label.into()])
        .as_::<Undefined>()
}

/// The timeLog function from the console namespace.
pub fn time_log0() -> Undefined {
    Any::global("console")
        .call("timeLog", &[])
        .as_::<Undefined>()
}

/// The timeLog function from the console namespace.
pub fn time_log1(label: &JsString) -> Undefined {
    Any::global("console")
        .call("timeLog", &[label.into()])
        .as_::<Undefined>()
}

/// The timeLog function from the console namespace.
pub fn time_log2(label: &JsString, data: &Any) -> Undefined {
    Any::global("console")
        .call("timeLog", &[label.into(), data.into()])
        .as_::<Undefined>()
}

/// The timeEnd function from the console namespace.
pub fn time_end0() -> Undefined {
    Any::global("console")
        .call("timeEnd", &[])
        .as_::<Undefined>()
}

/// The timeEnd function from the console namespace.
pub fn time_end1(label: &JsString) -> Undefined {
    Any::global("console")
        .call("timeEnd", &[label.into()])
        .as_::<Undefined>()
}
