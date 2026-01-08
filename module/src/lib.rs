#![no_std]
#![feature(allocator_api, global_asm)]

use kernel::prelude::*;
use kernel::file::{File, Operations};
use kernel::miscdevice::{Registration, MiscDevice};
use kernel::io_buffer::{IoBufferReader, IoBufferWriter};

module! {
    type: HelloRust,
    name: "hello_rust",
    author: "Rustacean",
    description: "Minimal Rust kernel module",
    license: "GPL",
}

struct HelloRust;

impl kernel::Module for HelloRust {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        let _reg = Registration::new_pinned::<HelloDevice>(c"hello_rust")?;
        pr_info!("Rust module loaded, device at /dev/hello_rust\n");
        Ok(HelloRust)
    }
}

struct HelloDevice;

impl Operations for HelloDevice {
    kernel::declare_file_operations!(read, write, ioctl);

    fn open(_data: &(), _file: &File) -> Result<()> {
        Ok(())
    }

    fn read(_data: &(), file: &File, buf: &mut impl IoBufferWriter, _offset: u64) -> Result<usize> {
        let msg = b"Hello from kernel!\n";
        let len = msg.len().min(buf.len());
        buf.write_slice(&msg[..len])?;
        Ok(len)
    }

    fn write(_data: &(), file: &File, buf: &mut impl IoBufferReader, _offset: u64) -> Result<usize> {
        let mut input = [0u8; 64];
        let len = input.len().min(buf.len());
        buf.read_slice(&mut input[..len])?;
        pr_info!("Rust kernel received: {:?}\n", &input[..len]);
        Ok(len)
    }

    fn ioctl(_data: &(), file: &File, cmd: u32, arg: usize) -> Result<i32> {
        const HELLO_MAGIC: u32 = b'R' as u32;
        const HELLO_GET: u32 = kernel::ioctl::_IOR(HELLO_MAGIC, 1, core::mem::size_of::<i32>());
        const HELLO_SET: u32 = kernel::ioctl::_IOW(HELLO_MAGIC, 2, core::mem::size_of::<i32>());

        match cmd {
            HELLO_GET => {
                let val: i32 = 42;
                unsafe { core::ptr::write_volatile(arg as *mut i32, val) };
                pr_info!("Rust kernel: GET returns {}\n", val);
                Ok(0)
            }
            HELLO_SET => {
                let val = unsafe { core::ptr::read_volatile(arg as *const i32) };
                pr_info!("Rust kernel: SET received {}\n", val);
                Ok(0)
            }
            _ => Err(kernel::Error::EINVAL)
        }
    }
}
