// Storm Core library: distributed storage & messaging for lightning network.
//
// Written in 2022 by
//     Dr. Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2022 by LNP/BP Standards Association, Switzerland.
//
// You should have received a copy of the MIT License along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use std::io;

use strict_encoding::{StrictDecode, StrictEncode};

pub const STORM_APP_RGB: u16 = 0x0001;
pub const STORM_APP_VENDOR_MASK: u16 = 0x8000;

/// Storm application identifier.
///
/// Range up to `0..0x8000` is reserved for applications registered as
/// LNPBP standards. Range `0x8000-0xFFFF` (custom user range) can be used
/// by any application without registration.
///
/// It is strongly advised to use random numbers from custom user range;
/// for instance by taking first two bytes of the SHA256 hash of the
/// application name or developer domain name and do a binary OR operation
/// with `0x8000`.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
pub enum StormApp {
    /// RGB smart contracts.
    #[display("rgb")]
    Rgb,

    /// Future applications. Numbers are reserved for LNPBP standardized apps.
    #[display("future({0:#06})")]
    Future(u16),

    /// Vendor-specific applications which does not standardized by LNP/BP
    /// Standards Association.
    #[display("vendor({0:#06})")]
    Vendor(u16),
}

impl StormApp {
    pub fn app_code(self) -> u16 {
        match self {
            StormApp::Rgb => STORM_APP_RGB,
            StormApp::Future(app) => app,
            StormApp::Vendor(vendor) => vendor,
        }
    }
}

impl StrictEncode for StormApp {
    fn strict_encode<E: io::Write>(&self, e: E) -> Result<usize, strict_encoding::Error> {
        self.app_code().strict_encode(e)
    }
}

impl StrictDecode for StormApp {
    fn strict_decode<D: io::Read>(d: D) -> Result<Self, strict_encoding::Error> {
        u16::strict_decode(d).map(StormApp::from)
    }
}

impl From<StormApp> for u16 {
    fn from(app: StormApp) -> Self { app.app_code() }
}

impl From<u16> for StormApp {
    fn from(code: u16) -> Self {
        match code {
            STORM_APP_RGB => StormApp::Rgb,
            vendor if vendor & STORM_APP_VENDOR_MASK > 0 => StormApp::Vendor(vendor),
            future => StormApp::Future(future),
        }
    }
}
