// Copyright (c) ChefKiss Inc 2021-2023. Licensed under the Thou Shalt Not Profit License version 1.0. See LICENSE for details.

use serde::{Deserialize, Serialize};

#[cfg(feature = "userspace")]
use super::syscall::SystemCall;

#[derive(Debug, Clone)]
pub struct Message {
    pub id: u64,
    pub pid: u64,
    pub data: &'static [u8],
}

impl Message {
    #[cfg(not(feature = "userspace"))]
    #[inline]
    #[must_use]
    pub const fn new(id: u64, pid: u64, data: &'static [u8]) -> Self {
        Self { id, pid, data }
    }

    #[cfg(feature = "userspace")]
    #[inline]
    #[must_use]
    pub const fn new(pid: u64, data: &'static [u8]) -> Self {
        Self { id: 0, pid, data }
    }
}

#[cfg(feature = "userspace")]
impl Message {
    #[must_use]
    pub unsafe fn receive() -> Self {
        let (mut id, mut pid): (u64, u64);
        let (mut ptr, mut len): (u64, u64);
        core::arch::asm!(
            "int 249",
            in("rdi") SystemCall::ReceiveMessage as u64,
            out("rax") id,
            lateout("rdi") pid,
            out("rsi") ptr,
            out("rdx") len,
            options(nostack, preserves_flags),
        );
        Self {
            id,
            pid,
            data: core::slice::from_raw_parts(ptr as *const u8, len as _),
        }
    }

    pub unsafe fn send(self) {
        core::arch::asm!(
            "int 249",
            in("rdi") SystemCall::SendMessage as u64,
            in("rsi") self.pid,
            in("rdx") self.data.as_ptr() as u64,
            in("rcx") self.data.len() as u64,
            options(nostack, preserves_flags),
        );
    }
}

#[cfg(feature = "userspace")]
impl Drop for Message {
    fn drop(&mut self) {
        if self.id == 0 {
            return;
        }
        unsafe {
            core::arch::asm!(
                "int 249",
                in("rdi") SystemCall::AckMessage as u64,
                in("rsi") self.id,
                options(nostack, preserves_flags),
            );
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(C)]
pub enum KernelMessage {
    IRQFired(u8),
}