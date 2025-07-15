# emlite-bind

emlite-bind is a Rust project that offers bindngs to Web APIs for use with wasm. It's backend agnostic, and should work with wasm32-unknown-unknown, wasm32-wasi, wasm32-wasip1 and wasm32-unknown-emscripten. It depends on the [emlite](https://github.com/emlite/emlite-js) library to make this happen.
emlite-bind consists of 2 crates:
- jsbind
- webbind

jsbind currently provides the minimal set of basic JS types needed for webbind. More types are planned later.

webbind wraps the webidl api in Rust. And it currently wraps most of the DOM API's.

Usage of the crate should not require an extra js glue step, but with the caveat that the emlite.js version be compatible!

## Requirement
A recent enough Rust compiler with your required target added. wasm32-wasip2 wasn't tested against emlite or emlite-bind.

## Usage
Add jsbind and webbind to your project:
```toml
[dependencies]
jsbind = "0.1"
webbind = "0.1"
```

Create a .cargo/config.toml with the following flags:
```toml
[target.wasm32-wasip1]
rustflags = ["-Clink-args=--no-entry --allow-undefined --export-dynamic --export-if-defined=main --export-table --import-memory --export-memory --strip-all"]

[target.wasm32-unknown-unknown]
rustflags = ["-Clink-args=--no-entry --allow-undefined --export-dynamic --export-if-defined=main --export-if-defined=_start --export-table --import-memory --export-memory --strip-all"]

[target.wasm32-unknown-emscripten]
rustflags = ["-Clink-args=-sERROR_ON_UNDEFINED_SYMBOLS=0 -sALLOW_MEMORY_GROWTH=1 -sEXPORTED_RUNTIME_METHODS=wasmTable -Wl,--strip-all -sMODULARIZE=1 -sEXPORT_ES6=1"]

# This is required for better stripping of the generated wasm file
[profile.release]
lto = true
```

Usage in your Rust source files:
```rust
use jsbind::prelude::*;
use webbind::html_button_element::HTMLButtonElement;
use webbind::node::Node;
use webbind::pointer_event::PointerEvent;
use webbind::window;

fn main() {
    let con = Console::get();
    let document = window().document();
    let bodies = document.get_elements_by_tag_name("body".into());
    if bodies.length() == 0 {
        con.log(&["I Ain't got Nobody!".into()]);
        return;
    }
    let body = bodies.item(0);
    let mut button = document
        .create_element0("BUTTON".into())
        .dyn_into::<HTMLButtonElement>()
        .unwrap();
    
    let style = button.style();
    style.set_property0("color".into(), "red".into());
    style.set_property0("background-color".into(), "#aaf".into());
    style.set_property0("border".into(), "solid".into());
    
    button.set_text_content("Click me".into());
    button.add_event_listener0(
        "click".into(),
        Closure::bind1(move |p: PointerEvent| {
            con.log(&[p.client_x().into()]);
        })
        .into(),
    );
    body.append_child(button.dyn_into::<Node>().unwrap().clone());
}
```

Run the build with your required target:
```bash
cargo build --target=wasm32-unknown-unknown
```

Load the generated wasm module in your index.html:
```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
</head>
<body>
    <script type="module">
        import { Emlite } from "https://unpkg.com/emlite";
        // or (if you decide to vendor emlite.js)
        // import { Emlite } from "./src/emlite.js";

        window.onload = async () => {
            const emlite = new Emlite();
            const bytes = await emlite.readFile(new URL("./target/wasm32-unknown-unknown/release/examples/button.wasm", import.meta.url));
            let wasm = await WebAssembly.compile(bytes);
            let inst = await WebAssembly.instantiate(wasm, {
                "env": {...emlite.env },
            });
            emlite.setExports(inst.exports);
            inst.exports.main();
        };
    </script>
</body>
</html>
```

If targeting wasi, you will need a wasi shim. The following example uses @bjorn3/browser_wasi_shim:
```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
</head>
<body>
    <script type="module">
        import { WASI, File, OpenFile, ConsoleStdout } from "https://unpkg.com/@bjorn3/browser_wasi_shim";
        import { Emlite } from "https://unpkg.com/emlite";
        // or (if you decide to vendor emlite.js)
        // import { Emlite } from "./src/emlite.js";

        window.onload = async () => {
            let fds = [
                new OpenFile(new File([])), // 0, stdin
                ConsoleStdout.lineBuffered(msg => console.log(`[WASI stdout] ${msg}`)), // 1, stdout
                ConsoleStdout.lineBuffered(msg => console.warn(`[WASI stderr] ${msg}`)), // 2, stderr
            ];
            let wasi = new WASI([], [], fds);
            const emlite = new Emlite();
            const bytes = await emlite.readFile(new URL("./target/wasm32-wasip1/release/examples/button.wasm", import.meta.url));
            let wasm = await WebAssembly.compile(bytes);
            let inst = await WebAssembly.instantiate(wasm, {
                "wasi_snapshot_preview1": wasi.wasiImport,
                "env": emlite.env,
            });
            emlite.setExports(inst.exports);
            // if your Rust has a main function, use: `wasi.start(inst)`. If not, use `wasi.initialize(inst)`, then call the required exported function
            wasi.start(inst);
        };
    </script>
</body>
</html>
```

If targeting emscripten, the link flags in .cargo/config.toml instruct the emscripten toolchain to generate an ES6 module. As such, you would need to initialize emscripten's js module:
```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
</head>
<body>
    <script type="module">
        import { Emlite } from "./src/emlite.js";
        import initModule from "./target/wasm32-unknown-emscripten/release/examples/button.js";
        window.onload = async () => {
            const emlite = new Emlite();
            const mymain = await initModule();
        };
    </script>
</body>
</html>
```

In the above html examples, emlite.js is fetched from unpkg, you can alternatively vendor the source, or create a node project and `npm i emlite`. If creating a node project, it's advisable to use a bundler (such as webpack for example).

## Testing
This repo comes with an index.html which you can use for testing purposes. Note that loading wasm in the browser requires an http server.
```bash
# after cloning and cd'ing into this repo
cargo build --examples --release --target=wasm32-unknown-unknown
# start a server
python3 -m http.server
```
Alternatively you can run:
```bash
# after cloning and cd'ing into this repo
npm install
npm run cargo
npm run serve # will run the http-server npm package
```