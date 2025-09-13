// SPDX-License-Identifier: GPL-2.0

// Copyright (C) 2024 Google LLC.

//! Rust misc device sample.

use {
    core::pin::Pin,
    kernel::{
        InPlaceModule, ThisModule,
        alloc::{flags::GFP_KERNEL, kbox::KBox},
        c_str, dev_err, dev_info,
        device::Device,
        error::{Error, Result, code::ENOTTY},
        fs::File,
        init::InPlaceInit,
        ioctl::{_IO, _IOC_SIZE, _IOR, _IOW},
        macros::module,
        miscdevice::{MiscDevice, MiscDeviceOptions, MiscDeviceRegistration},
        new_mutex, pr_info,
        prelude::vtable,
        sync::{
            Mutex,
            lock::{Guard, mutex::MutexBackend},
        },
        try_pin_init,
        types::ARef,
        uaccess::{UserSlice, UserSliceReader, UserSliceWriter},
    },
    pin_init::{PinInit, pin_data, pinned_drop},
};

const RUST_MISC_DEV_HELLO: u32 = _IO('|' as u32, 0x80);
const RUST_MISC_DEV_GET_VALUE: u32 = _IOR::<i32>('|' as u32, 0x81);
const RUST_MISC_DEV_SET_VALUE: u32 = _IOW::<i32>('|' as u32, 0x82);

struct Inner {
    value: i32,
}

#[pin_data(PinnedDrop)]
struct RustMiscDevice {
    #[pin]
    inner: Mutex<Inner>,
    dev: ARef<Device>,
}

#[pinned_drop]
impl PinnedDrop for RustMiscDevice {
    fn drop(self: Pin<&mut Self>) {
        dev_info!(self.dev, "Exiting the Rust Misc Device Sample\n");
    }
}

impl RustMiscDevice {
    fn set_value(&self, mut reader: UserSliceReader) -> Result<isize> {
        let new_value: i32 = reader.read::<i32>()?;
        let mut guard: Guard<'_, Inner, MutexBackend> = self.inner.lock();

        dev_info!(
            self.dev,
            "-> Copying data from userspace (value: {new_value})\n",
        );

        guard.value = new_value;
        Ok(0)
    }

    fn get_value(&self, mut writer: UserSliceWriter) -> Result<isize> {
        let guard: Guard<'_, Inner, MutexBackend> = self.inner.lock();
        let value: i32 = guard.value;

        // Free-up the lock and use our locally cached instance from here
        () = drop(guard);

        dev_info!(
            self.dev,
            "-> Copying data to userspace (value: {})\n",
            &value
        );

        () = writer.write::<i32>(&value)?;
        Ok(0)
    }

    fn hello(&self) -> Result<isize> {
        dev_info!(self.dev, "-> Hello from the Rust Misc Device\n");
        Ok(0)
    }
}

#[vtable]
impl MiscDevice for RustMiscDevice {
    type Ptr = Pin<KBox<Self>>;

    fn open(_file: &File, misc: &MiscDeviceRegistration<Self>) -> Result<Pin<KBox<Self>>> {
        let dev: ARef<Device> = ARef::from(misc.device());

        dev_info!(dev, "Opening Rust Misc Device Sample\n");

        KBox::try_pin_init(
            try_pin_init! {
                RustMiscDevice {
                    inner <- new_mutex!( Inner{ value: 0_i32 } ),
                    dev,
                }
            },
            GFP_KERNEL,
        )
    }

    fn ioctl(me: Pin<&RustMiscDevice>, _file: &File, cmd: u32, arg: usize) -> Result<isize> {
        dev_info!(me.dev, "IOCTLing Rust Misc Device Sample\n");

        let size: usize = _IOC_SIZE(cmd);

        match cmd {
            RUST_MISC_DEV_GET_VALUE => me.get_value(UserSlice::new(arg, size).writer())?,
            RUST_MISC_DEV_SET_VALUE => me.set_value(UserSlice::new(arg, size).reader())?,
            RUST_MISC_DEV_HELLO => me.hello()?,
            _ => {
                dev_err!(me.dev, "-> IOCTL not recognised: {cmd}\n");
                return Err(ENOTTY);
            }
        };

        Ok(0)
    }
}

#[pin_data]
struct RustMiscDeviceModule {
    #[pin]
    _miscdev: MiscDeviceRegistration<RustMiscDevice>,
}

impl InPlaceModule for RustMiscDeviceModule {
    fn init(_module: &'static ThisModule) -> impl PinInit<Self, Error> {
        pr_info!("Initialising Rust Misc Device Sample\n");

        let options: MiscDeviceOptions = MiscDeviceOptions {
            name: c_str!("rust-misc-device"),
        };

        try_pin_init!(Self {
            _miscdev <- MiscDeviceRegistration::register(options),
        })
    }
}

module! {
    type: RustMiscDeviceModule,
    name: "rust_misc_device",
    authors: ["Lee Jones"],
    description: "Rust minimal sample",
    license: "GPL",
}
