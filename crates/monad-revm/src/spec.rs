//! Contains the `[MonadSpecId]` type and its implementation.
use core::str::FromStr;
use revm::primitives::hardfork::{SpecId, UnknownHardfork};

/// Monad spec id.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(non_camel_case_types)]
pub enum MonadSpecId {
    /// Monad launch spec id.
    #[default]
    MonadEight = 100,
}

impl MonadSpecId {
    /// Converts the [`MonadSpecId`] into a [`SpecId`].
    pub const fn into_eth_spec(self) -> SpecId {
        match self {
            Self::MonadEight => SpecId::PRAGUE,
        }
    }

    /// Checks if the [`MonadSpecId`] is enabled in the other [`MonadSpecId`].
    pub const fn is_enabled_in(self, other: MonadSpecId) -> bool {
        other as u8 <= self as u8
    }
}

impl From<MonadSpecId> for SpecId {
    fn from(spec: MonadSpecId) -> Self {
        spec.into_eth_spec()
    }
}

impl From<SpecId> for MonadSpecId {
    /// Converts an Ethereum [`SpecId`] to the corresponding [`MonadSpecId`].
    ///
    /// This maps Ethereum hardforks to their Monad equivalents:
    /// - PRAGUE and earlier → MonadSpecId::MonadEight (Monad's genesis is PRAGUE-based)
    ///
    /// When new Monad hardforks are added, update this mapping:
    /// ```ignore
    /// match spec {
    ///     SpecId::OSAKA | SpecId::AMSTERDAM.. => MonadSpecId::MonadNine,
    ///     _ => MonadSpecId::MonadEight,
    /// }
    /// ```
    fn from(spec: SpecId) -> Self {
        // Currently Monad only has one hardfork (MonadEight), which is PRAGUE-based.
        // When future Monad forks are added, map newer Ethereum specs accordingly.
        match spec {
            // Future: Add mappings for newer Monad hardforks here
            // SpecId::OSAKA | SpecId::AMSTERDAM.. => MonadSpecId::MonadNine,
            _ => MonadSpecId::MonadEight,
        }
    }
}

impl FromStr for MonadSpecId {
    type Err = UnknownHardfork;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            name::MONAD_EIGHT => Ok(MonadSpecId::MonadEight),
            _ => Err(UnknownHardfork),
        }
    }
}

impl From<MonadSpecId> for &'static str {
    fn from(spec_id: MonadSpecId) -> Self {
        match spec_id {
            MonadSpecId::MonadEight => name::MONAD_EIGHT,
        }
    }
}

/// String identifiers for Monad hardforks
pub mod name {
    /// Mainnet launch spec name.
    pub const MONAD_EIGHT: &str = "MonadEight";
}
