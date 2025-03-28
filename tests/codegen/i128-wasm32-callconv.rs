//! Verify that Rust implements the expected calling convention for `i128`/`u128`.

//@ add-core-stubs
//@ compile-flags: -Copt-level=3 --target wasm32-wasip1
//@ needs-llvm-components: webassembly

#![crate_type = "lib"]
#![no_std]
#![no_core]
#![feature(no_core, lang_items)]

extern crate minicore;

extern "C" {
    fn extern_call(arg0: i128);
    fn extern_ret() -> i128;
}

#[no_mangle]
pub extern "C" fn pass(_arg0: u32, arg1: i128) {
    // CHECK-LABEL: @pass(
    // an i128 is passed via registers
    // CHECK-SAME: i128 noundef %arg1
    // CHECK: call void @extern_call
    unsafe { extern_call(arg1) };
}

// Check that we produce the correct return ABI
#[no_mangle]
pub extern "C" fn ret(_arg0: u32, arg1: i128) -> i128 {
    // CHECK-LABEL: @ret(
    // but an i128 is returned via the stack
    // CHECK-SAME: sret
    // CHECK: store i128 %arg1
    // CHECK-NEXT: ret void
    arg1
}

// Check that we consume the correct return ABI
#[no_mangle]
pub extern "C" fn forward(dst: *mut i128) {
    // CHECK-LABEL: @forward
    // CHECK-SAME: ptr{{.*}} %dst)
    // without optimizatons, an intermediate alloca is used
    // CHECK: call void @extern_ret
    // CHECK: store i128
    // CHECK: ret void
    unsafe { *dst = extern_ret() };
}
