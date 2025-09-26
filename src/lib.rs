// cargo +nightly b --target wasm32-wasip1-threads
// If you run this command, you wait infinitely.
// wasmtime run -Sthreads=y --invoke spawn target/wasm32-wasip1-threads/debug/small_example_wasip1_lib_spawn.wasm
// If you run this commands, it works fine.
// cargo +nightly b --features patch --target wasm32-wasip1-threads
// wasmtime run -Sthreads=y --invoke spawn target/wasm32-wasip1-threads/debug/small_example_wasip1_lib_spawn.wasm

#[unsafe(no_mangle)]
extern "C" fn spawn() {
    #[cfg(feature = "patch")]
    patch::_initialize();

    std::thread::spawn(|| println!("Hello from a thread spawned in the `init` function!"))
        .join()
        .unwrap();
}

#[cfg(feature = "patch")]
mod patch {
    use std::sync::atomic::AtomicBool;

    unsafe extern "C" {
        // Initialize thread-local storage for WASI threads
        // In the current version of Rust,
        // thread initialization is not performed.
        // Therefore, we must force linking to perform initialization.
        // https://github.com/rust-lang/rust/pull/108097
        fn __wasi_init_tp();
        fn __wasm_call_ctors();
    }

    pub extern "C" fn _initialize() {
        static FIRST: AtomicBool = AtomicBool::new(true);
        if !FIRST.swap(false, std::sync::atomic::Ordering::SeqCst) {
            return;
        }
        unsafe { __wasi_init_tp() };
        unsafe { __wasm_call_ctors() };
    }
}
