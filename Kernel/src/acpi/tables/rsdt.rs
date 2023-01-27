// Copyright (c) ChefKiss Inc 2021-2023. Licensed under the Thou Shalt Not Profit License version 1.0. See LICENSE for details.

#[repr(C, packed)]
pub struct Rsdt(super::SdtHeader);

impl core::ops::Deref for Rsdt {
    type Target = super::SdtHeader;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::fmt::Debug for Rsdt {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RSDT")
            .field("header", &self.0)
            .finish_non_exhaustive()
    }
}