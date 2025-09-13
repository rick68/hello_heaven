use {
    libc::{_IO, _IOR, _IOW, O_RDWR, c_int},
    rust_misc_device::cvt,
    std::{
        fs::File,
        io,
        mem::{self, MaybeUninit},
        os::fd::{AsRawFd, FromRawFd},
    },
};

const RUST_MISC_DEV_FAIL: u64 = _IO('|' as u32, 0x00);
const RUST_MISC_DEV_HELLO: u64 = _IO('|' as u32, 0x80);
const RUST_MISC_DEV_GET_VALUE: u64 = _IOR::<i32>('|' as u32, 0x81);
const RUST_MISC_DEV_SET_VALUE: u64 = _IOW::<i32>('|' as u32, 0x82);

fn main() -> io::Result<()> {
    let mut value: MaybeUninit<c_int> = MaybeUninit::uninit();
    let new_value: MaybeUninit<c_int> = MaybeUninit::uninit();

    // Open the device file
    println!("Opening /dev/rust-misc-device for reading and writing");

    let file: File = unsafe {
        let fd: c_int = cvt(libc::open(c"/dev/rust-misc-device".as_ptr(), O_RDWR))?;
        File::from_raw_fd(fd.as_raw_fd())
    };

    // Make call into driver to say "hello"
    println!("Calling Hello");
    let _: c_int = cvt(unsafe { libc::ioctl(file.as_raw_fd(), RUST_MISC_DEV_HELLO, 0) })?;

    // Get initial value
    println!("Fetching initial value");
    let _: c_int =
        cvt(unsafe { libc::ioctl(file.as_raw_fd(), RUST_MISC_DEV_GET_VALUE, value.as_ptr()) })?;

    unsafe {
        *value.assume_init_mut() += 1;
    }

    // Set value to something different
    println!("Submitting new value ({})", unsafe {
        value.assume_init_read()
    });
    let _: c_int =
        cvt(unsafe { libc::ioctl(file.as_raw_fd(), RUST_MISC_DEV_SET_VALUE, value.as_ptr()) })?;

    // Ensure new value was applied
    println!("Fetching new value");
    let _: c_int = cvt(unsafe {
        libc::ioctl(
            file.as_raw_fd(),
            RUST_MISC_DEV_GET_VALUE,
            new_value.as_ptr(),
        )
    })?;

    unsafe {
        let value: c_int = value.assume_init_read();
        let new_value: c_int = new_value.assume_init_read();

        if value != new_value {
            panic!("Failed: Committed and retrieved values are different ({value} - {new_value})")
        }
    }

    // Call the unsuccessful ioctl
    println!("Attempting to call in to an non-existent IOCTL");
    match cvt(unsafe { libc::ioctl(file.as_raw_fd(), RUST_MISC_DEV_FAIL, 0) }) {
        Ok(_) => {
            eprintln!("ioctl: Failed to fail");
            return Err(io::Error::from(io::ErrorKind::Other));
        }
        Err(_) => {
            println!("ioctl: Succeeded to fail - this was expected");
        }
    };

    // Close the device file
    println!("Closing /dev/rust-misc-device");
    () = mem::drop(file);

    println!("Succenss");
    Ok(())
}
