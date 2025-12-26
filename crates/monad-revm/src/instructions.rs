use crate::MonadSpecId;
use revm::{
    handler::instructions::EthInstructions,
    interpreter::{
        gas::params::{GasId, GasParams},
        instructions::instruction_table_gas_changes_spec,
        interpreter::EthInterpreter,
        Host,
    },
};

/// Type alias for Monad instructions.
pub type MonadInstructions<CTX> = EthInstructions<EthInterpreter, CTX>;

/// Monad-specific gas parameters for a given hardfork.
/// Override Ethereum defaults with Monad's gas costs.
///
/// Monad increases cold access costs to account for the relatively higher cost
/// of state reads from disk. See: https://docs.monad.xyz/developer-essentials/opcode-pricing#cold-access-cost
///
/// | Access Type | Ethereum | Monad |
/// |-------------|----------|-------|
/// | Account     | 2600     | 10100 |
/// | Storage     | 2100     | 8100  |
///
/// Warm access costs (100 gas) remain the same as Ethereum.
pub fn monad_gas_params(spec: MonadSpecId) -> GasParams {
    let eth_spec = spec.into_eth_spec();
    let mut params = GasParams::new_spec(eth_spec);

    if MonadSpecId::Monad.is_enabled_in(spec) {
        params.override_gas([
            // SSTORE uses full cold storage cost
            (GasId::cold_storage_cost(), COLD_SLOAD_COST),
            // SLOAD uses additional cost (cold - warm)
            (
                GasId::cold_storage_additional_cost(),
                COLD_SLOAD_COST - WARM_STORAGE_READ_COST,
            ),
            // Account access opcodes (BALANCE, EXTCODESIZE, EXTCODECOPY, EXTCODEHASH,
            // CALL, CALLCODE, DELEGATECALL, STATICCALL, SELFDESTRUCT) use additional cost
            (
                GasId::cold_account_additional_cost(),
                COLD_ACCOUNT_ACCESS_COST - WARM_STORAGE_READ_COST,
            ),
        ]);
    }

    params
}

// Create Monad instructions table with custom gas costs.
/// This function combines:
/// 1. Standard instruction table for the underlying Ethereum spec
/// 2. Monad-specific gas parameters for the hardfork
/// 3. Any custom Monad opcodes (future)
pub fn monad_instructions<CTX: Host>(spec: MonadSpecId) -> MonadInstructions<CTX> {
    let eth_spec = spec.into_eth_spec();
    let instructions = EthInstructions::new(
        instruction_table_gas_changes_spec(eth_spec),
        monad_gas_params(spec),
        eth_spec,
    );

    instructions
}

/// Monad cold storage access cost (SLOAD, SSTORE).
/// Ethereum: 2100, Monad: 8100
pub const COLD_SLOAD_COST: u64 = 8100;

/// Monad cold account access cost (BALANCE, EXTCODE*, CALL*, SELFDESTRUCT).
/// Ethereum: 2600, Monad: 10100
pub const COLD_ACCOUNT_ACCESS_COST: u64 = 10100;

/// Warm storage read cost - same as Ethereum.
pub const WARM_STORAGE_READ_COST: u64 = 100;
