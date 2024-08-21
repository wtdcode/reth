pub struct Env {
    // required
    current_coinbase: Address,
    current_gas_limit: u64,
    current_number: u64,
    current_timestamp: u64,
    withdrawals: Vec<Withdrawal>,

    // optional
    current_difficulty: U256,
    current_random: U256,
    current_base_fee: U256,
    parent_gas_used: u64,
    parent_gas_limit: u64,
    parent_timestamp: u64,
    block_hashes: Vec<B256>,
    parent_uncle_hash: B256,
    ommers: Vec<Ommer>,
}

pub struct Ommer {
    delta: u64,
    address: Address,
}
