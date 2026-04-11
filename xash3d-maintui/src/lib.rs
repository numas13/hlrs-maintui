#![no_std]

#[macro_use]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[macro_use]
extern crate log;

macro_rules! define_strings {
    ($($name:ident { $($body:tt)* })*) => {
        $(pub mod $name {
            define_strings!($($body)*);
        })*
    };
    ($($name:ident = $value:expr),* $(,)?) => {
        $(pub const $name: &str = $value;)*
    };
}

mod config_list;
mod export;
mod i18n;
mod input;
mod macros;
mod menu;
mod prelude;
mod saved_servers;
mod server_info;
mod strings;
mod ui;
mod widgets;

#[cfg(not(feature = "std"))]
#[cfg(not(test))]
#[global_allocator]
static ALLOCATOR: xash3d_allocator::System = xash3d_allocator::System::new();

#[cfg(not(feature = "std"))]
#[cfg(panic = "abort")]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    error!("{info}");
    unsafe { libc::abort() }
}

#[cfg(not(feature = "std"))]
#[cfg(panic = "abort")]
#[unsafe(no_mangle)]
fn rust_eh_personality() {}
