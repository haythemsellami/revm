//! ABI encoding/decoding helpers for staking precompile.
//!
//! Matches Solidity ABI encoding used by the staking contract interface.

use revm::primitives::{Address, Bytes, U256};

/// Function selectors for staking precompile methods.
pub mod selectors {
    // View methods
    /// getEpoch() => (uint64, bool)
    pub const GET_EPOCH: u32 = 0x757991a8;
    /// getProposerValId() => uint64
    pub const GET_PROPOSER_VAL_ID: u32 = 0xfbacb0be;
    /// getValidator(uint64) => (...)
    pub const GET_VALIDATOR: u32 = 0x2b6d639a;
    /// getDelegator(uint64, address) => (...)
    pub const GET_DELEGATOR: u32 = 0x573c1ce0;
    /// getWithdrawalRequest(uint64, address, uint8) => (...)
    pub const GET_WITHDRAWAL_REQUEST: u32 = 0x56fa2045;
    /// getConsensusValidatorSet(uint32) => (...)
    pub const GET_CONSENSUS_VALIDATOR_SET: u32 = 0xfb29b729;
    /// getSnapshotValidatorSet(uint32) => (...)
    pub const GET_SNAPSHOT_VALIDATOR_SET: u32 = 0xde66a368;
    /// getExecutionValidatorSet(uint32) => (...)
    pub const GET_EXECUTION_VALIDATOR_SET: u32 = 0x7cb074df;
    /// getDelegations(address, uint64) => (...)
    pub const GET_DELEGATIONS: u32 = 0x4fd66050;
    /// getDelegators(uint64, address) => (...)
    pub const GET_DELEGATORS: u32 = 0xa0843a26;

    // Write methods
    /// addValidator(...) => uint64
    pub const ADD_VALIDATOR: u32 = 0xf145204c;
    /// delegate(uint64) payable => bool
    pub const DELEGATE: u32 = 0x84994fec;
    /// undelegate(uint64, uint256, uint8) => bool
    pub const UNDELEGATE: u32 = 0x5cf41514;
    /// withdraw(uint64, uint8) => bool
    pub const WITHDRAW: u32 = 0xaed2ee73;
    /// compound(uint64) => bool
    pub const COMPOUND: u32 = 0xb34fea67;
    /// claimRewards(uint64) => bool
    pub const CLAIM_REWARDS: u32 = 0xa76e2ca5;
    /// changeCommission(uint64, uint256) => bool
    pub const CHANGE_COMMISSION: u32 = 0x9bdcc3c8;
    /// externalReward(uint64) payable => bool
    pub const EXTERNAL_REWARD: u32 = 0xe4b3303b;

    // Syscalls (internal)
    /// syscallOnEpochChange(bytes)
    pub const SYSCALL_ON_EPOCH_CHANGE: u32 = 0x1d4e9f02;
    /// syscallReward(bytes)
    pub const SYSCALL_REWARD: u32 = 0x791bdcf3;
    /// syscallSnapshot(bytes)
    pub const SYSCALL_SNAPSHOT: u32 = 0x157eeb21;
}

/// Gas costs for staking operations.
pub mod gas {
    /// getEpoch view
    pub const GET_EPOCH: u64 = 16_200;
    /// getProposerValId view (MONAD_FIVE+)
    pub const GET_PROPOSER_VAL_ID: u64 = 100;
    /// getValidator view
    pub const GET_VALIDATOR: u64 = 97_200;
    /// getDelegator view (pulls up to date)
    pub const GET_DELEGATOR: u64 = 184_900;
    /// getWithdrawalRequest view
    pub const GET_WITHDRAWAL_REQUEST: u64 = 24_300;
}

/// Decode function selector from input.
pub fn decode_selector(input: &[u8]) -> Option<u32> {
    if input.len() < 4 {
        return None;
    }
    Some(u32::from_be_bytes([input[0], input[1], input[2], input[3]]))
}

/// Decode a uint64 from ABI-encoded input at given offset.
pub fn decode_u64(input: &[u8], offset: usize) -> Option<u64> {
    if input.len() < offset + 32 {
        return None;
    }
    // ABI encodes uint64 as 32 bytes, right-aligned
    let bytes = &input[offset..offset + 32];
    // Check high bytes are zero
    if bytes[..24] != [0u8; 24] {
        return None;
    }
    Some(u64::from_be_bytes(bytes[24..32].try_into().ok()?))
}

/// Decode a uint8 from ABI-encoded input at given offset.
pub fn decode_u8(input: &[u8], offset: usize) -> Option<u8> {
    if input.len() < offset + 32 {
        return None;
    }
    // ABI encodes uint8 as 32 bytes, right-aligned
    let bytes = &input[offset..offset + 32];
    // Check high bytes are zero
    if bytes[..31] != [0u8; 31] {
        return None;
    }
    Some(bytes[31])
}

/// Decode an address from ABI-encoded input at given offset.
pub fn decode_address(input: &[u8], offset: usize) -> Option<Address> {
    if input.len() < offset + 32 {
        return None;
    }
    // ABI encodes address as 32 bytes, right-aligned (12 zero bytes + 20 address bytes)
    let bytes = &input[offset..offset + 32];
    // Check high bytes are zero
    if bytes[..12] != [0u8; 12] {
        return None;
    }
    Some(Address::from_slice(&bytes[12..32]))
}

/// Encode a bool to ABI format (32 bytes).
pub fn encode_bool(value: bool) -> [u8; 32] {
    let mut result = [0u8; 32];
    result[31] = if value { 1 } else { 0 };
    result
}

/// Encode a uint64 to ABI format (32 bytes).
pub fn encode_u64(value: u64) -> [u8; 32] {
    let mut result = [0u8; 32];
    result[24..32].copy_from_slice(&value.to_be_bytes());
    result
}

/// Encode a uint256 to ABI format (32 bytes).
pub fn encode_u256(value: U256) -> [u8; 32] {
    value.to_be_bytes::<32>()
}

/// Encode an address to ABI format (32 bytes).
pub fn encode_address(value: &Address) -> [u8; 32] {
    let mut result = [0u8; 32];
    result[12..32].copy_from_slice(value.as_slice());
    result
}

/// Encode bytes to ABI dynamic bytes format.
pub fn encode_bytes(value: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    // Length prefix
    result.extend_from_slice(&encode_u256(U256::from(value.len())));
    // Data, padded to 32 bytes
    result.extend_from_slice(value);
    let padding = (32 - (value.len() % 32)) % 32;
    result.extend_from_slice(&vec![0u8; padding]);
    result
}

/// Build ABI-encoded output for getEpoch().
pub fn encode_get_epoch_result(epoch: u64, in_delay_period: bool) -> Bytes {
    let mut result = Vec::with_capacity(64);
    result.extend_from_slice(&encode_u64(epoch));
    result.extend_from_slice(&encode_bool(in_delay_period));
    result.into()
}

/// Build ABI-encoded output for getProposerValId().
pub fn encode_get_proposer_val_id_result(val_id: u64) -> Bytes {
    encode_u64(val_id).to_vec().into()
}

/// Build ABI-encoded output for getValidator().
///
/// Returns: (auth_address, flags, stake, accumulated_reward_per_token, commission,
///           unclaimed_rewards, consensus_stake, snapshot_stake, secp_pubkey, bls_pubkey)
#[allow(clippy::too_many_arguments)]
pub fn encode_get_validator_result(
    auth_address: &Address,
    flags: u64,
    stake: U256,
    accumulated_reward_per_token: U256,
    commission: U256,
    unclaimed_rewards: U256,
    consensus_stake: U256,
    snapshot_stake: U256,
    secp_pubkey: &[u8; 33],
    bls_pubkey: &[u8; 48],
) -> Bytes {
    let mut result = Vec::with_capacity(384);

    // Fixed-size values first
    result.extend_from_slice(&encode_address(auth_address));
    result.extend_from_slice(&encode_u64(flags));
    result.extend_from_slice(&encode_u256(stake));
    result.extend_from_slice(&encode_u256(accumulated_reward_per_token));
    result.extend_from_slice(&encode_u256(commission));
    result.extend_from_slice(&encode_u256(unclaimed_rewards));
    result.extend_from_slice(&encode_u256(consensus_stake));
    result.extend_from_slice(&encode_u256(snapshot_stake));

    // Dynamic bytes offsets (2 dynamic fields: secp_pubkey, bls_pubkey)
    let base_offset = 10 * 32; // 10 fixed fields × 32 bytes
    result.extend_from_slice(&encode_u256(U256::from(base_offset)));
    result.extend_from_slice(&encode_u256(U256::from(
        base_offset + 32 + ((33 + 31) / 32) * 32,
    )));

    // Dynamic bytes data
    result.extend_from_slice(&encode_bytes(secp_pubkey));
    result.extend_from_slice(&encode_bytes(bls_pubkey));

    result.into()
}

/// Build ABI-encoded output for getDelegator().
///
/// Returns: (stake, accumulated_reward_per_token, rewards, delta_stake,
///           next_delta_stake, delta_epoch, next_delta_epoch)
pub fn encode_get_delegator_result(
    stake: U256,
    accumulated_reward_per_token: U256,
    rewards: U256,
    delta_stake: U256,
    next_delta_stake: U256,
    delta_epoch: u64,
    next_delta_epoch: u64,
) -> Bytes {
    let mut result = Vec::with_capacity(224);
    result.extend_from_slice(&encode_u256(stake));
    result.extend_from_slice(&encode_u256(accumulated_reward_per_token));
    result.extend_from_slice(&encode_u256(rewards));
    result.extend_from_slice(&encode_u256(delta_stake));
    result.extend_from_slice(&encode_u256(next_delta_stake));
    result.extend_from_slice(&encode_u64(delta_epoch));
    result.extend_from_slice(&encode_u64(next_delta_epoch));
    result.into()
}

/// Build ABI-encoded output for getWithdrawalRequest().
///
/// Returns: (amount, accumulator, epoch)
pub fn encode_get_withdrawal_request_result(amount: U256, accumulator: U256, epoch: u64) -> Bytes {
    let mut result = Vec::with_capacity(96);
    result.extend_from_slice(&encode_u256(amount));
    result.extend_from_slice(&encode_u256(accumulator));
    result.extend_from_slice(&encode_u64(epoch));
    result.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_selector() {
        let input = [0x75, 0x79, 0x91, 0xa8, 0x00, 0x00];
        assert_eq!(decode_selector(&input), Some(selectors::GET_EPOCH));

        let short = [0x75, 0x79];
        assert_eq!(decode_selector(&short), None);
    }

    #[test]
    fn test_decode_u64() {
        let mut input = [0u8; 36];
        input[28..36].copy_from_slice(&42u64.to_be_bytes());

        assert_eq!(decode_u64(&input, 4), Some(42));

        // With non-zero high bytes (invalid)
        input[4] = 1;
        assert_eq!(decode_u64(&input, 4), None);
    }

    #[test]
    fn test_decode_address() {
        let mut input = [0u8; 36];
        let addr = Address::new([0x11; 20]);
        input[16..36].copy_from_slice(addr.as_slice());

        assert_eq!(decode_address(&input, 4), Some(addr));
    }

    #[test]
    fn test_encode_u64() {
        let encoded = encode_u64(42);
        assert_eq!(encoded[..24], [0u8; 24]);
        assert_eq!(&encoded[24..], &42u64.to_be_bytes());
    }

    #[test]
    fn test_encode_bool() {
        let true_encoded = encode_bool(true);
        assert_eq!(true_encoded[31], 1);
        assert_eq!(true_encoded[..31], [0u8; 31]);

        let false_encoded = encode_bool(false);
        assert_eq!(false_encoded[31], 0);
    }

    #[test]
    fn test_get_epoch_result() {
        let result = encode_get_epoch_result(100, true);
        assert_eq!(result.len(), 64);

        // Check epoch
        assert_eq!(decode_u64(&result, 0), Some(100));
        // Check bool (at offset 32)
        assert_eq!(result[63], 1);
    }
}
