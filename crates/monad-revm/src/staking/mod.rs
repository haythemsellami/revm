//! Monad staking precompile implementation.
//!
//! Implements read-only view methods for the staking contract at address 0x1000.
//! Storage layout and ABI encoding match the C++ implementation.
//!
//! ## Implemented Methods
//!
//! | Method | Selector | Gas |
//! |--------|----------|-----|
//! | getEpoch | 0x757991a8 | 16,200 |
//! | getProposerValId | 0xfbacb0be | 100 |
//! | getValidator | 0x2b6d639a | 97,200 |
//! | getDelegator | 0x573c1ce0 | 184,900 |
//! | getWithdrawalRequest | 0x56fa2045 | 24,300 |

pub mod abi;
pub mod storage;
pub mod types;

use abi::selectors;
use revm::{
    context_interface::{ContextTr, JournalTr, LocalContextTr},
    interpreter::{CallInputs, Gas, InstructionResult, InterpreterResult},
    precompile::PrecompileError,
    primitives::{Address, Bytes, U256},
};
use storage::{
    delegator_key, delegator_offsets, global_slots, validator_key, validator_offsets,
    withdrawal_key, withdrawal_offsets, STAKING_ADDRESS,
};
use types::{Delegator, EpochInfo, Validator, WithdrawalRequest};

/// Run the staking precompile.
///
/// Returns `None` if the address is not the staking precompile.
/// Returns `Some(result)` with the execution result.
pub fn run_staking_precompile<CTX: ContextTr>(
    context: &mut CTX,
    inputs: &CallInputs,
) -> Result<Option<InterpreterResult>, String> {
    // Check if this is the staking precompile address
    if inputs.bytecode_address != STAKING_ADDRESS {
        return Ok(None);
    }

    // Get input bytes
    let input_bytes: Vec<u8> = match &inputs.input {
        revm::interpreter::CallInput::SharedBuffer(range) => context
            .local()
            .shared_memory_buffer_slice(range.clone())
            .map(|slice| slice.to_vec())
            .unwrap_or_default(),
        revm::interpreter::CallInput::Bytes(bytes) => bytes.0.to_vec(),
    };

    // Decode selector
    let selector = abi::decode_selector(&input_bytes).ok_or("Invalid input: missing selector")?;

    // Dispatch to appropriate handler
    let result = match selector {
        selectors::GET_EPOCH => handle_get_epoch(context, &input_bytes, inputs.gas_limit),
        selectors::GET_PROPOSER_VAL_ID => {
            handle_get_proposer_val_id(context, &input_bytes, inputs.gas_limit)
        }
        selectors::GET_VALIDATOR => handle_get_validator(context, &input_bytes, inputs.gas_limit),
        selectors::GET_DELEGATOR => handle_get_delegator(context, &input_bytes, inputs.gas_limit),
        selectors::GET_WITHDRAWAL_REQUEST => {
            handle_get_withdrawal_request(context, &input_bytes, inputs.gas_limit)
        }
        _ => Err(PrecompileError::Other(
            format!("Unknown selector: {selector:#x}").into(),
        )),
    };

    // Convert result to InterpreterResult
    match result {
        Ok((gas_used, output)) => {
            let mut interpreter_result = InterpreterResult {
                result: InstructionResult::Return,
                gas: Gas::new(inputs.gas_limit),
                output,
            };
            if !interpreter_result.gas.record_cost(gas_used) {
                interpreter_result.result = InstructionResult::PrecompileOOG;
            }
            Ok(Some(interpreter_result))
        }
        Err(e) => Ok(Some(InterpreterResult {
            result: if e.is_oog() {
                InstructionResult::PrecompileOOG
            } else {
                InstructionResult::PrecompileError
            },
            gas: Gas::new(inputs.gas_limit),
            output: Bytes::new(),
        })),
    }
}

/// Handle getEpoch() => (uint64 epoch, bool inDelayPeriod)
fn handle_get_epoch<CTX: ContextTr>(
    context: &mut CTX,
    _input: &[u8],
    gas_limit: u64,
) -> Result<(u64, Bytes), PrecompileError> {
    if gas_limit < abi::gas::GET_EPOCH {
        return Err(PrecompileError::OutOfGas);
    }

    let epoch_info = read_epoch_info(context)?;

    Ok((
        abi::gas::GET_EPOCH,
        abi::encode_get_epoch_result(epoch_info.epoch, epoch_info.in_delay_period),
    ))
}

/// Handle getProposerValId() => uint64
fn handle_get_proposer_val_id<CTX: ContextTr>(
    context: &mut CTX,
    _input: &[u8],
    gas_limit: u64,
) -> Result<(u64, Bytes), PrecompileError> {
    if gas_limit < abi::gas::GET_PROPOSER_VAL_ID {
        return Err(PrecompileError::OutOfGas);
    }

    let val_id = read_storage_u64(context, global_slots::PROPOSER_VAL_ID)?;

    Ok((
        abi::gas::GET_PROPOSER_VAL_ID,
        abi::encode_get_proposer_val_id_result(val_id),
    ))
}

/// Handle getValidator(uint64 valId) => (...)
fn handle_get_validator<CTX: ContextTr>(
    context: &mut CTX,
    input: &[u8],
    gas_limit: u64,
) -> Result<(u64, Bytes), PrecompileError> {
    if gas_limit < abi::gas::GET_VALIDATOR {
        return Err(PrecompileError::OutOfGas);
    }

    // Decode val_id from input (after 4-byte selector)
    let val_id = abi::decode_u64(input, 4)
        .ok_or_else(|| PrecompileError::Other("Invalid val_id".into()))?;

    let validator = read_validator(context, val_id)?;

    // Read consensus and snapshot stakes
    let consensus_stake = read_storage_u256(context, storage::consensus_view_key(val_id, 0))?;
    let snapshot_stake = read_storage_u256(context, storage::snapshot_view_key(val_id, 0))?;

    Ok((
        abi::gas::GET_VALIDATOR,
        abi::encode_get_validator_result(
            &validator.auth_address,
            validator.flags,
            validator.stake,
            validator.accumulated_reward_per_token,
            validator.commission,
            validator.unclaimed_rewards,
            consensus_stake,
            snapshot_stake,
            &validator.secp_pubkey,
            &validator.bls_pubkey,
        ),
    ))
}

/// Handle getDelegator(uint64 valId, address delegator) => (...)
fn handle_get_delegator<CTX: ContextTr>(
    context: &mut CTX,
    input: &[u8],
    gas_limit: u64,
) -> Result<(u64, Bytes), PrecompileError> {
    if gas_limit < abi::gas::GET_DELEGATOR {
        return Err(PrecompileError::OutOfGas);
    }

    // Decode parameters
    let val_id = abi::decode_u64(input, 4)
        .ok_or_else(|| PrecompileError::Other("Invalid val_id".into()))?;
    let delegator_addr = abi::decode_address(input, 36)
        .ok_or_else(|| PrecompileError::Other("Invalid delegator address".into()))?;

    let delegator = read_delegator(context, val_id, &delegator_addr)?;

    Ok((
        abi::gas::GET_DELEGATOR,
        abi::encode_get_delegator_result(
            delegator.stake,
            delegator.accumulated_reward_per_token,
            delegator.rewards,
            delegator.delta_stake,
            delegator.next_delta_stake,
            delegator.delta_epoch,
            delegator.next_delta_epoch,
        ),
    ))
}

/// Handle getWithdrawalRequest(uint64 valId, address delegator, uint8 withdrawalId) => (...)
fn handle_get_withdrawal_request<CTX: ContextTr>(
    context: &mut CTX,
    input: &[u8],
    gas_limit: u64,
) -> Result<(u64, Bytes), PrecompileError> {
    if gas_limit < abi::gas::GET_WITHDRAWAL_REQUEST {
        return Err(PrecompileError::OutOfGas);
    }

    // Decode parameters
    let val_id = abi::decode_u64(input, 4)
        .ok_or_else(|| PrecompileError::Other("Invalid val_id".into()))?;
    let delegator_addr = abi::decode_address(input, 36)
        .ok_or_else(|| PrecompileError::Other("Invalid delegator address".into()))?;
    let withdrawal_id = abi::decode_u8(input, 68)
        .ok_or_else(|| PrecompileError::Other("Invalid withdrawal_id".into()))?;

    let request = read_withdrawal_request(context, val_id, &delegator_addr, withdrawal_id)?;

    Ok((
        abi::gas::GET_WITHDRAWAL_REQUEST,
        abi::encode_get_withdrawal_request_result(request.amount, request.accumulator, request.epoch),
    ))
}

// ═══════════════════════════════════════════════════════════════════════════════
// Storage Read Helpers
// ═══════════════════════════════════════════════════════════════════════════════

/// Read epoch info from storage.
fn read_epoch_info<CTX: ContextTr>(context: &mut CTX) -> Result<EpochInfo, PrecompileError> {
    let epoch = read_storage_u64(context, global_slots::EPOCH)?;
    let in_delay_raw = read_storage_u256(context, global_slots::IN_BOUNDARY)?;
    let in_delay_period = in_delay_raw != U256::ZERO;

    Ok(EpochInfo {
        epoch,
        in_delay_period,
    })
}

/// Read a validator from storage.
fn read_validator<CTX: ContextTr>(
    context: &mut CTX,
    val_id: u64,
) -> Result<Validator, PrecompileError> {
    // Read all 8 slots
    let stake = read_storage_u256(context, validator_key(val_id, validator_offsets::STAKE))?;
    let accumulated_reward_per_token = read_storage_u256(
        context,
        validator_key(val_id, validator_offsets::ACCUMULATED_REWARD_PER_TOKEN),
    )?;
    let commission =
        read_storage_u256(context, validator_key(val_id, validator_offsets::COMMISSION))?;

    // Keys span 3 slots (slots 3, 4, 5)
    // secp_pubkey (33 bytes) + bls_pubkey (48 bytes) = 81 bytes across 96 bytes (3 slots)
    let keys_slot_0 =
        read_storage_u256(context, validator_key(val_id, validator_offsets::KEYS))?.to_be_bytes::<32>();
    let keys_slot_1 = read_storage_u256(context, validator_key(val_id, validator_offsets::KEYS + 1))?
        .to_be_bytes::<32>();
    let keys_slot_2 = read_storage_u256(context, validator_key(val_id, validator_offsets::KEYS + 2))?
        .to_be_bytes::<32>();

    // Concatenate all 3 slots then slice out the keys
    // Layout: secp (33 bytes at offset 0) + bls (48 bytes at offset 33)
    let mut keys_concat = [0u8; 96];
    keys_concat[0..32].copy_from_slice(&keys_slot_0);
    keys_concat[32..64].copy_from_slice(&keys_slot_1);
    keys_concat[64..96].copy_from_slice(&keys_slot_2);

    let mut secp_pubkey = [0u8; 33];
    let mut bls_pubkey = [0u8; 48];
    secp_pubkey.copy_from_slice(&keys_concat[0..33]);
    bls_pubkey.copy_from_slice(&keys_concat[33..81]);

    // Address + flags (slot 6)
    let address_flags_raw =
        read_storage_u256(context, validator_key(val_id, validator_offsets::ADDRESS_FLAGS))?
            .to_be_bytes::<32>();
    // Layout: address (20 bytes) + flags (8 bytes) = 28 bytes, left-aligned
    let auth_address = Address::from_slice(&address_flags_raw[0..20]);
    let flags = u64::from_be_bytes(address_flags_raw[20..28].try_into().unwrap());

    let unclaimed_rewards = read_storage_u256(
        context,
        validator_key(val_id, validator_offsets::UNCLAIMED_REWARDS),
    )?;

    Ok(Validator {
        stake,
        accumulated_reward_per_token,
        commission,
        secp_pubkey,
        bls_pubkey,
        auth_address,
        flags,
        unclaimed_rewards,
    })
}

/// Read a delegator from storage.
fn read_delegator<CTX: ContextTr>(
    context: &mut CTX,
    val_id: u64,
    delegator_addr: &Address,
) -> Result<Delegator, PrecompileError> {
    let stake = read_storage_u256(
        context,
        delegator_key(val_id, delegator_addr, delegator_offsets::STAKE),
    )?;
    let accumulated_reward_per_token = read_storage_u256(
        context,
        delegator_key(
            val_id,
            delegator_addr,
            delegator_offsets::ACCUMULATED_REWARD_PER_TOKEN,
        ),
    )?;
    let rewards = read_storage_u256(
        context,
        delegator_key(val_id, delegator_addr, delegator_offsets::REWARDS),
    )?;
    let delta_stake = read_storage_u256(
        context,
        delegator_key(val_id, delegator_addr, delegator_offsets::DELTA_STAKE),
    )?;
    let next_delta_stake = read_storage_u256(
        context,
        delegator_key(val_id, delegator_addr, delegator_offsets::NEXT_DELTA_STAKE),
    )?;

    // Epochs (packed u64 + u64)
    let epochs_raw = read_storage_u256(
        context,
        delegator_key(val_id, delegator_addr, delegator_offsets::EPOCHS),
    )?
    .to_be_bytes::<32>();
    // Layout: delta_epoch (8 bytes) + next_delta_epoch (8 bytes) = 16 bytes, left-aligned
    let delta_epoch = u64::from_be_bytes(epochs_raw[0..8].try_into().unwrap());
    let next_delta_epoch = u64::from_be_bytes(epochs_raw[8..16].try_into().unwrap());

    Ok(Delegator {
        stake,
        accumulated_reward_per_token,
        rewards,
        delta_stake,
        next_delta_stake,
        delta_epoch,
        next_delta_epoch,
    })
}

/// Read a withdrawal request from storage.
fn read_withdrawal_request<CTX: ContextTr>(
    context: &mut CTX,
    val_id: u64,
    delegator_addr: &Address,
    withdrawal_id: u8,
) -> Result<WithdrawalRequest, PrecompileError> {
    let amount = read_storage_u256(
        context,
        withdrawal_key(val_id, delegator_addr, withdrawal_id, withdrawal_offsets::AMOUNT),
    )?;
    let accumulator = read_storage_u256(
        context,
        withdrawal_key(
            val_id,
            delegator_addr,
            withdrawal_id,
            withdrawal_offsets::ACCUMULATOR,
        ),
    )?;

    // Epoch is u64 in first 8 bytes of the slot
    let epoch_raw = read_storage_u256(
        context,
        withdrawal_key(val_id, delegator_addr, withdrawal_id, withdrawal_offsets::EPOCH),
    )?
    .to_be_bytes::<32>();
    let epoch = u64::from_be_bytes(epoch_raw[0..8].try_into().unwrap());

    Ok(WithdrawalRequest {
        amount,
        accumulator,
        epoch,
    })
}

/// Read a U256 from storage.
fn read_storage_u256<CTX: ContextTr>(context: &mut CTX, key: U256) -> Result<U256, PrecompileError> {
    context
        .journal_mut()
        .sload(STAKING_ADDRESS, key)
        .map(|r| r.data)
        .map_err(|e| PrecompileError::Other(format!("Storage read failed: {e:?}").into()))
}

/// Read a u64 from storage (stored as U256, take lower 64 bits).
fn read_storage_u64<CTX: ContextTr>(context: &mut CTX, key: U256) -> Result<u64, PrecompileError> {
    let value = read_storage_u256(context, key)?;
    // For simple u64 values stored as U256, take lower 64 bits
    Ok(value.as_limbs()[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staking_address_constant() {
        assert_eq!(STAKING_ADDRESS, storage::STAKING_ADDRESS);
    }
}
