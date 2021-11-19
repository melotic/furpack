#![no_main]
#![no_std]
#![feature(alloc_error_handler)]

extern crate libc;

use aes_gcm::NewAead;
use aes_gcm::{aead::Aead, Aes128Gcm, Key, Nonce};
use libc_alloc::LibcAlloc;
use libc::{abort, fexecve, memfd_create, write};
use lz4_flex::decompress_size_prepended;

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8, _env: *const *const u8) -> isize {
    let key = include_bytes!("../../stub_data_key.bin");
    let nonce = include_bytes!("../../stub_data_nonce.bin");
    let data = include_bytes!("../../stub_data.bin");

    let key = Key::from_slice(key);
    let nonce = Nonce::from_slice(nonce);
    let cipher = Aes128Gcm::new(key);

    match cipher.decrypt(nonce, data.as_ref()) {
        Ok(plaintext) => {
            let uncompressed = match decompress_size_prepended(plaintext.as_ref()) {
                Ok(uncompressed) => uncompressed,
                Err(_) => unsafe {
                    abort();
                },
            };

            let fd = unsafe { memfd_create("stub_data\0".as_ptr() as *const _, 0) };

            // Write the plaintext to the file descriptor.
            unsafe {
                write(fd, uncompressed.as_ptr() as *const _, uncompressed.len());
            }

            // Execute the file.
            unsafe {
                fexecve(fd, _argv as *const *const _, _env as *const *const _);
            }
        }
        Err(_) => unsafe {
            abort();
        },
    }

    0
}

extern crate alloc;
#[alloc_error_handler]
fn alloc_error_handler(_: alloc::alloc::Layout) -> ! {
    unsafe {
        abort();
    }
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        abort();
    }
}
