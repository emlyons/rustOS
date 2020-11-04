#![feature(alloc_error_handler)]
#![feature(const_fn)]
#![feature(decl_macro)]
#![feature(asm)]
#![feature(global_asm)]
#![feature(optin_builtin_traits)]
#![feature(ptr_internals)]
#![feature(raw_vec_internals)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#[cfg(not(test))]
mod init;

extern crate alloc;

pub mod allocator;
pub mod console;
pub mod fs;
pub mod mutex;
pub mod shell;
pub mod param;
pub mod process;
pub mod traps;
pub mod vm;

use console::{kprint, kprintln, CONSOLE};
use core::time::Duration;
use pi::timer::spin_sleep;
use pi::atags;

use allocator::Allocator;
use fs::FileSystem;
use process::GlobalScheduler;
use traps::irq::Irq;
use vm::VMManager;
use aarch64::*;

#[cfg_attr(not(test), global_allocator)]
pub static ALLOCATOR: Allocator = Allocator::uninitialized();
pub static FILESYSTEM: FileSystem = FileSystem::uninitialized();
pub static SCHEDULER: GlobalScheduler = GlobalScheduler::uninitialized();
pub static VMM: VMManager = VMManager::uninitialized();
pub static IRQ: Irq = Irq::uninitialized();

fn kmain() -> ! {

    spin_sleep(Duration::from_secs(1));
    
    // ATAG report
    let atag = atags::Atags::get();
    atag.for_each(|x| kprintln!("{:#?}\n\n", x));

    unsafe {
        ALLOCATOR.initialize();
	kprintln!("memory allocation initialized");
        FILESYSTEM.initialize();
	kprintln!("file system initialized");
	IRQ.initialize();
	kprintln!("irq system initialized");
	VMM.initialize();
	kprintln!("virtual memory system initialized");
	SCHEDULER.initialize();
	kprintln!("scheduler initialized");
	SCHEDULER.start();
	kprintln!("scheduler started");
    }

        kprintln!("
   .~~.   .~~.
  '. \\ ' ' / .'
   .~ .~~~..~.
  : .~.'~'.~. :
 ~ (   ) (   ) ~
( : '~'.~.'~' : )
 ~ .~ (   ) ~. ~
  (  : '~' :  )
   '~ .~~~. ~'
       '~'
Welcome to rustOS on Raspberry Pi!
");
    


    loop {
	shell::shell(">");
    }

}

// TODO: TEMP
pub extern "C" fn temp_shell() {
    use crate::shell;
    loop {
	shell::shell("$");
    }
}
