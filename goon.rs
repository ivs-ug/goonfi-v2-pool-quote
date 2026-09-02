#[derive(Debug, Clone, Copy)]
pub struct FeeTier {
    pub ticks_atb: [u64; TICK_COUNT],
    pub ticks_bta: [u64; TICK_COUNT],
    pub capacity_mult: u64,
}

#[derive(Debug, Clone)]
pub struct Pool {
    pub version: u64,
    pub available_capacity_a: u64,
    pub available_capacity_b: u64,
    pub last_update_slot_a: u64,
    pub last_update_slot_b: u64,

    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub vault_a: Pubkey,
    pub vault_b: Pubkey,
    pub oracle: Pubkey,

    pub fee_tiers: [FeeTier; TIER_COUNT],
    pub global_ticks_atb: [u64; TICK_COUNT],
    pub global_ticks_bta: [u64; TICK_COUNT],

    pub slippage_param: u64,
    pub theo_base_a: u64,
    pub theo_base_b: u64,
    pub theo_max_decay: u64,
    pub theo_skew_a: u64,
    pub theo_skew_b: u64,
    pub theo_last_ts: i64,

    pub timeout_byte: u8,
    pub reserve_byte: u8,
    pub fee_tier_idx_exact_out: u8,
    pub fee_tier_idx_exact_in: u8,
    pub fee_selector: u8,
    pub paused: bool,
}

impl Pool {
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        if data.len() < ACCOUNT_MIN_LEN {
            return Err(eyre!(
                "account too short: {} < 0x{ACCOUNT_MIN_LEN:X}",
                data.len()
            ));
        }

        let mut fee_tiers = [FeeTier {
            ticks_atb: [0; TICK_COUNT],
            ticks_bta: [0; TICK_COUNT],
            capacity_mult: 0,
        }; TIER_COUNT];

        for i in 0..TIER_COUNT {
            let b = OFF_FEE_TIER_TABLE + i * TIER_STRIDE;
            for j in 0..TICK_COUNT {
                fee_tiers[i].ticks_atb[j] = rd_u64(data, b + (j * 8));
                fee_tiers[i].ticks_bta[j] = rd_u64(data, b + 0x50 + (j * 8));
            }
            fee_tiers[i].capacity_mult = rd_u64(data, b + 0xA0);
        }

        let mut global_ticks_atb = [0; TICK_COUNT];
        let mut global_ticks_bta = [0; TICK_COUNT];
        for i in 0..TICK_COUNT {
            global_ticks_atb[i] = rd_u64(data, OFF_GLOBAL_TICKS_A + i * 8);
            global_ticks_bta[i] = rd_u64(data, OFF_GLOBAL_TICKS_B + i * 8);
        }

        Ok(Pool {
            version: rd_u64(data, OFF_VERSION),
            available_capacity_a: rd_u64(data, OFF_AVAIL_CAP_A),
            available_capacity_b: rd_u64(data, OFF_AVAIL_CAP_B),
            last_update_slot_a: rd_u64(data, OFF_LAST_SLOT_A),
            last_update_slot_b: rd_u64(data, OFF_LAST_SLOT_B),
            mint_a: rd_pubkey(data, OFF_MINT_A),
            mint_b: rd_pubkey(data, OFF_MINT_B),
            vault_a: rd_pubkey(data, OFF_VAULT_A),
            vault_b: rd_pubkey(data, OFF_VAULT_B),
            oracle: rd_pubkey(data, OFF_ORACLE),
            fee_tiers,
            global_ticks_atb,
            global_ticks_bta,
            slippage_param: rd_u64(data, OFF_SLIPPAGE_PARAM),
            theo_base_a: rd_u64(data, OFF_THEO_BASE_A),
            theo_base_b: rd_u64(data, OFF_THEO_BASE_B),
            theo_max_decay: rd_u64(data, OFF_THEO_MAX_DECAY),
            theo_skew_a: rd_u64(data, OFF_THEO_SKEW_A),
            theo_skew_b: rd_u64(data, OFF_THEO_SKEW_B),
            theo_last_ts: rd_i64(data, OFF_THEO_LAST_TS),
            timeout_byte: rd_u8(data, OFF_TIMEOUT_BYTE),
            reserve_byte: rd_u8(data, OFF_RESERVE_BYTE),
            fee_tier_idx_exact_out: rd_u8(data, OFF_FEE_TIER_IDX_BASE + 1),
            fee_tier_idx_exact_in: rd_u8(data, OFF_FEE_TIER_IDX_BASE + 3),
            fee_selector: rd_u8(data, OFF_FEE_SELECTOR),
            paused: rd_u8(data, OFF_PAUSED) != 0,
        })
    }
}

pub fn quote(
    amount_in: u64,
    oracle_data: &[u8],
    a_to_b: bool,
    pool: &Pool,
    current_slot: u64,
    vault_reserve: u64,
) -> Option<u64> {
  // ...
}
