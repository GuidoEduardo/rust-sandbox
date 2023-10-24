use v8;
use std::{fs, path::Path};

fn main() {
    let path = Path::new("assets/source.js");
    let source_file = fs::read_to_string(&path).unwrap();

    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(Default::default());

    let scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(scope);
    let scope = &mut v8::ContextScope::new(scope, context);

    let source = v8::String::new(scope, &source_file).unwrap();

    let script = v8::Script::compile(scope, source, None).unwrap();

    let result = script.run(scope).unwrap();
    let result = result.to_string(scope).unwrap();

    println!("Result: {}", result.to_rust_string_lossy(scope));
}
