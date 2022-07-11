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

pub const STORM_APP_SYSTEM: u16 = 0x0000;
pub const STORM_APP_CHAT: u16 = 0x0001;
pub const STORM_APP_FILE_TRANSFER: u16 = 0x0002;
pub const STORM_APP_STORAGE: u16 = 0x0003;
pub const STORM_APP_SEARCH: u16 = 0x0004;
pub const STORM_APP_RGB_CONTRACTS: u16 = 0x0010;
pub const STORM_APP_RGB_TRANSFERS: u16 = 0x0011;
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
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub enum StormApp {
    /// System Storm app.
    #[display("system")]
    System,

    /// Chat messaging storm app.
    #[display("chat")]
    Chat,

    /// Distributed storage storm app.
    #[display("file-transfer")]
    FileTransfer,

    /// Distributed storage storm app.
    #[display("storage")]
    Storage,

    /// Distributed data system with storage and search as a storm app.
    #[display("search")]
    Search,

    /// RGB smart contracts distribution network.
    #[display("rgb-contracts")]
    RgbContracts,

    /// State transfers between RGB smart contracts.
    #[display("rgb-transfers")]
    RgbTransfers,

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
            StormApp::System => STORM_APP_SYSTEM,
            StormApp::Chat => STORM_APP_CHAT,
            StormApp::Storage => STORM_APP_STORAGE,
            StormApp::FileTransfer => STORM_APP_FILE_TRANSFER,
            StormApp::Search => STORM_APP_SEARCH,
            StormApp::RgbContracts => STORM_APP_RGB_CONTRACTS,
            StormApp::RgbTransfers => STORM_APP_RGB_TRANSFERS,
            StormApp::Future(app) => app,
            StormApp::Vendor(vendor) => vendor,
        }
    }
}

impl StrictEncode for StormApp {
    fn strict_encode<E: io::Write>(
        &self,
        e: E,
    ) -> Result<usize, strict_encoding::Error> {
        self.app_code().strict_encode(e)
    }
}

impl StrictDecode for StormApp {
    fn strict_decode<D: io::Read>(
        d: D,
    ) -> Result<Self, strict_encoding::Error> {
        u16::strict_decode(d).map(StormApp::from)
    }
}

impl From<StormApp> for u16 {
    fn from(app: StormApp) -> Self { app.app_code() }
}

impl From<u16> for StormApp {
    fn from(code: u16) -> Self {
        match code {
            STORM_APP_SYSTEM => StormApp::System,
            STORM_APP_CHAT => StormApp::Chat,
            STORM_APP_STORAGE => StormApp::Storage,
            STORM_APP_FILE_TRANSFER => StormApp::FileTransfer,
            STORM_APP_SEARCH => StormApp::Search,
            STORM_APP_RGB_CONTRACTS => StormApp::RgbContracts,
            STORM_APP_RGB_TRANSFERS => StormApp::RgbTransfers,
            vendor if vendor & STORM_APP_VENDOR_MASK > 0 => {
                StormApp::Vendor(vendor)
            }
            future => StormApp::Future(future),
        }
    }
}
