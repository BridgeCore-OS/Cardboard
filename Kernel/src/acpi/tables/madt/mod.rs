// Copyright (c) ChefKiss Inc 2021-2023. Licensed under the Thou Shalt Not Profit License version 1.0. See LICENSE for details.

use core::mem::size_of;

use modular_bitfield::prelude::*;

use self::ic::{
    ioapic::{InterruptSourceOverride, IoApic, NMISource},
    proc_lapic::{LocalAPICAddrOverride, LocalAPICNMI, ProcessorLocalAPIC},
    ICHeader, InterruptController,
};

pub mod ic;

#[bitfield(bits = 32)]
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub struct MADTFlags {
    #[skip(setters)]
    pub pcat_compat: bool,
    #[skip]
    __: B31,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Madt {
    header: super::SdtHeader,
    local_ic_addr: u32,
    pub flags: MADTFlags,
}

pub struct MadtIter {
    ptr: *const u8,
    curr: usize,
    total: usize,
}

impl Iterator for MadtIter {
    type Item = InterruptController;

    fn next(&mut self) -> core::option::Option<<Self as core::iter::Iterator>::Item> {
        if self.curr == self.total {
            None
        } else {
            let next = unsafe { self.ptr.add(self.curr).cast::<ICHeader>().as_ref().unwrap() };
            self.curr += next.length();
            unsafe {
                Some(match next.type_ {
                    0 => InterruptController::ProcessorLocalAPIC(
                        (next as *const ICHeader)
                            .cast::<ProcessorLocalAPIC>()
                            .as_ref()
                            .unwrap(),
                    ),
                    1 => InterruptController::InputOutputAPIC(
                        (next as *const ICHeader).cast::<IoApic>().as_ref().unwrap(),
                    ),
                    2 => InterruptController::InterruptSourceOverride(
                        (next as *const ICHeader)
                            .cast::<InterruptSourceOverride>()
                            .as_ref()
                            .unwrap(),
                    ),
                    3 => InterruptController::NmiSrc(
                        (next as *const ICHeader)
                            .cast::<NMISource>()
                            .as_ref()
                            .unwrap(),
                    ),
                    4 => InterruptController::LApicNmi(
                        (next as *const ICHeader)
                            .cast::<LocalAPICNMI>()
                            .as_ref()
                            .unwrap(),
                    ),
                    5 => InterruptController::LApicAddrOverride(
                        (next as *const ICHeader)
                            .cast::<LocalAPICAddrOverride>()
                            .as_ref()
                            .unwrap(),
                    ),
                    6 => InterruptController::IoSapic(next),
                    7 => InterruptController::LocalSapic(next),
                    8 => InterruptController::PlatformInterruptSrcs(next),
                    9 => InterruptController::ProcessorLocalx2APIC(next),
                    0xA => InterruptController::Localx2APICNmi(next),
                    0xB => InterruptController::GicCpu(next),
                    0xC => InterruptController::GicDist(next),
                    0xD => InterruptController::GicMsiFrame(next),
                    0xE => InterruptController::GicRedist(next),
                    0xF => InterruptController::GicIts(next),
                    0x10 => InterruptController::MpWakeup(next),
                    0x11..=0x7F => InterruptController::Reserved(next),
                    0x80..=0xFF => InterruptController::OemReserved(next),
                })
            }
        }
    }
}

impl Madt {
    #[must_use]
    pub const fn local_ic_addr(&self) -> u64 {
        self.local_ic_addr as u64
    }

    #[must_use]
    pub fn as_iter(&self) -> MadtIter {
        MadtIter {
            ptr: unsafe { (self as *const Self).cast::<u8>().add(size_of::<Self>()) },
            curr: 0,
            total: self.length as usize - size_of::<Self>(),
        }
    }
}

impl core::ops::Deref for Madt {
    type Target = super::SdtHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}