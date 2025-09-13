// SPDX-License-Identifier: GPL-2.0

//! Rust Platform driver sample.

use {
    core::pin::Pin,
    kernel::{
        alloc::{flags::GFP_KERNEL, kbox::KBox},
        c_str, dev_dbg, dev_info,
        device::Core,
        error::Result,
        module_platform_driver, of, of_device_table, platform,
        types::ARef,
    },
};

struct SampleDriver {
    pdev: ARef<platform::Device>,
}

struct Info(u32);

of_device_table!(
    OF_TABLE,
    MODULE_OF_TABLE,
    <SampleDriver as platform::Driver>::IdInfo,
    [(of::DeviceId::new(c_str!("test,rust-device")), Info(42))]
);

impl platform::Driver for SampleDriver {
    type IdInfo = Info;

    const OF_ID_TABLE: Option<of::IdTable<Self::IdInfo>> = Some(&OF_TABLE);

    fn probe(
        pdev: &platform::Device<Core>,
        info: Option<&Self::IdInfo>,
    ) -> Result<Pin<KBox<Self>>> {
        dev_dbg!(pdev.as_ref(), "Probe Rust Platform driver sample.\n");

        if let Some(info) = info {
            dev_info!(pdev.as_ref(), "Probed with info: '{}'.\n", info.0);
        }

        let drvdata: KBox<SampleDriver> = KBox::new(
            Self {
                pdev: ARef::from(pdev),
            },
            GFP_KERNEL,
        )?;

        Ok(unsafe { Pin::new_unchecked(drvdata) })
    }
}

impl Drop for SampleDriver {
    fn drop(&mut self) {
        dev_dbg!(self.pdev.as_ref(), "Remove Rust Platform driver sample.\n");
    }
}

module_platform_driver! {
    type: SampleDriver,
    name: "rust_driver_platform",
    authors: ["Danilo Krummrich"],
    description: "Rust Platform driver",
    license: "GPL v2",
}
