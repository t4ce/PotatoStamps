#![no_std]

pub mod scene;

// Pure host tests link the public TRUEOS vGPU descriptor types but never call
// into a Blueprint. Provide only the two no-op symbols pulled in by that API.
#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn trueos_cabi_blueprint_shutdown(_: *const u8, _: usize) -> i32 {
    0
}

#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn trueos_cabi_write(_: u32, _: *const u8, _: usize) {}
