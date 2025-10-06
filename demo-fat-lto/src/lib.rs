#[cfg(any(debug_assertions, miri))]
compile_error!(
    "Build only with release profile, so that 'fat lto' has effect. See static*.sh and literal*.sh."
);
