//! Builder for custom [`OpChainSpec`].

#![cfg(feature = "test-utils")]

use alloy_chains::Chain;
use alloy_genesis::Genesis;
use alloy_primitives::U256;
use reth_chainspec::{ChainSpec, ChainSpecBuilder};
use reth_ethereum_forks::{EthereumHardfork, ForkCondition, OptimismHardfork};

use crate::{OpChainSpec, OP_MAINNET};

/// A helper to build custom chain specs
#[derive(Debug, Default, Clone)]
pub struct OpChainSpecBuilder {
    inner: ChainSpecBuilder,
}

impl OpChainSpecBuilder {
    /// Construct a new builder from the mainnet chain spec.
    pub fn mainnet() -> Self {
        Self {
            inner: ChainSpecBuilder {
                chain: Some(OP_MAINNET.chain),
                genesis: Some(OP_MAINNET.genesis.clone()),
                hardforks: OP_MAINNET.hardforks.clone(),
            },
        }
    }

    /// Set the chain ID
    pub const fn chain(mut self, chain: Chain) -> Self {
        self.inner.chain = Some(chain);
        self
    }

    /// Set the genesis block.
    pub fn genesis(mut self, genesis: Genesis) -> Self {
        self.inner.genesis = Some(genesis);
        self
    }

    /// Add the given fork with the given activation condition to the spec.
    pub fn with_fork(mut self, fork: EthereumHardfork, condition: ForkCondition) -> Self {
        self.inner.hardforks.insert(fork, condition);
        self
    }

    /// Remove the given fork from the spec.
    pub fn without_fork(mut self, fork: EthereumHardfork) -> Self {
        self.inner.hardforks.remove(fork);
        self
    }

    /// Enable the Paris hardfork at the given TTD.
    ///
    /// Does not set the merge netsplit block.
    pub fn paris_at_ttd(mut self, ttd: U256) -> Self {
        self.inner = self.inner.with_fork(
            EthereumHardfork::Paris,
            ForkCondition::TTD { total_difficulty: ttd, fork_block: None },
        );
        self
    }

    /// Enable Frontier at genesis.
    pub fn frontier_activated(mut self) -> Self {
        self.inner.hardforks.insert(EthereumHardfork::Frontier, ForkCondition::Block(0));
        self
    }

    /// Enable Homestead at genesis.
    pub fn homestead_activated(mut self) -> Self {
        self.inner = self.inner.frontier_activated();
        self.inner.hardforks.insert(EthereumHardfork::Homestead, ForkCondition::Block(0));
        self
    }

    /// Enable Tangerine at genesis.
    pub fn tangerine_whistle_activated(mut self) -> Self {
        self.inner = self.inner.homestead_activated();
        self.inner.hardforks.insert(EthereumHardfork::Tangerine, ForkCondition::Block(0));
        self
    }

    /// Enable Spurious Dragon at genesis.
    pub fn spurious_dragon_activated(mut self) -> Self {
        self.inner = self.inner.tangerine_whistle_activated();
        self.inner.hardforks.insert(EthereumHardfork::SpuriousDragon, ForkCondition::Block(0));
        self
    }

    /// Enable Byzantium at genesis.
    pub fn byzantium_activated(mut self) -> Self {
        self.inner = self.inner.spurious_dragon_activated();
        self.inner.hardforks.insert(EthereumHardfork::Byzantium, ForkCondition::Block(0));
        self
    }

    /// Enable Constantinople at genesis.
    pub fn constantinople_activated(mut self) -> Self {
        self.inner = self.inner.byzantium_activated();
        self.inner.hardforks.insert(EthereumHardfork::Constantinople, ForkCondition::Block(0));
        self
    }

    /// Enable Petersburg at genesis.
    pub fn petersburg_activated(mut self) -> Self {
        self.inner = self.inner.constantinople_activated();
        self.inner.hardforks.insert(EthereumHardfork::Petersburg, ForkCondition::Block(0));
        self
    }

    /// Enable Istanbul at genesis.
    pub fn istanbul_activated(mut self) -> Self {
        self.inner = self.inner.petersburg_activated();
        self.inner.hardforks.insert(EthereumHardfork::Istanbul, ForkCondition::Block(0));
        self
    }

    /// Enable Berlin at genesis.
    pub fn berlin_activated(mut self) -> Self {
        self.inner = self.inner.istanbul_activated();
        self.inner.hardforks.insert(EthereumHardfork::Berlin, ForkCondition::Block(0));
        self
    }

    /// Enable London at genesis.
    pub fn london_activated(mut self) -> Self {
        self.inner = self.inner.berlin_activated();
        self.inner.hardforks.insert(EthereumHardfork::London, ForkCondition::Block(0));
        self
    }

    /// Enable Paris at genesis.
    pub fn paris_activated(mut self) -> Self {
        self.inner = self.inner.london_activated();
        self.inner.hardforks.insert(
            EthereumHardfork::Paris,
            ForkCondition::TTD { fork_block: Some(0), total_difficulty: U256::ZERO },
        );
        self
    }

    /// Enable Shanghai at genesis.
    pub fn shanghai_activated(mut self) -> Self {
        self.inner = self.inner.paris_activated();
        self.inner.hardforks.insert(EthereumHardfork::Shanghai, ForkCondition::Timestamp(0));
        self
    }

    /// Enable Cancun at genesis.
    pub fn cancun_activated(mut self) -> Self {
        self.inner = self.inner.shanghai_activated();
        self.inner.hardforks.insert(EthereumHardfork::Cancun, ForkCondition::Timestamp(0));
        self
    }

    /// Enable Prague at genesis.
    pub fn prague_activated(mut self) -> Self {
        self.inner = self.inner.cancun_activated();
        self.inner.hardforks.insert(EthereumHardfork::Prague, ForkCondition::Timestamp(0));
        self
    }

    /// Enable Bedrock at genesis
    pub fn bedrock_activated(mut self) -> Self {
        self.inner = self.inner.paris_activated();
        self.inner.hardforks.insert(OptimismHardfork::Bedrock, ForkCondition::Block(0));
        self
    }

    /// Enable Regolith at genesis
    pub fn regolith_activated(mut self) -> Self {
        self = self.bedrock_activated();
        self.inner.hardforks.insert(OptimismHardfork::Regolith, ForkCondition::Timestamp(0));
        self
    }

    /// Enable Canyon at genesis
    pub fn canyon_activated(mut self) -> Self {
        self = self.regolith_activated();
        // Canyon also activates changes from L1's Shanghai hardfork
        self.inner.hardforks.insert(EthereumHardfork::Shanghai, ForkCondition::Timestamp(0));
        self.inner.hardforks.insert(OptimismHardfork::Canyon, ForkCondition::Timestamp(0));
        self
    }

    /// Enable Ecotone at genesis
    pub fn ecotone_activated(mut self) -> Self {
        self = self.canyon_activated();
        self.inner.hardforks.insert(EthereumHardfork::Cancun, ForkCondition::Timestamp(0));
        self.inner.hardforks.insert(OptimismHardfork::Ecotone, ForkCondition::Timestamp(0));
        self
    }

    /// Enable Fjord at genesis
    pub fn fjord_activated(mut self) -> Self {
        self = self.ecotone_activated();
        self.inner.hardforks.insert(OptimismHardfork::Fjord, ForkCondition::Timestamp(0));
        self
    }

    /// Enable Granite at genesis
    pub fn granite_activated(mut self) -> Self {
        self = self.fjord_activated();
        self.inner.hardforks.insert(OptimismHardfork::Granite, ForkCondition::Timestamp(0));
        self
    }

    /// Build the resulting [`ChainSpec`].
    ///
    /// # Panics
    ///
    /// This function panics if the chain ID and genesis is not set ([`Self::chain`] and
    /// [`Self::genesis`])
    pub fn build(self) -> OpChainSpec {
        let paris_block_and_final_difficulty = {
            self.inner.hardforks.get(EthereumHardfork::Paris).and_then(|cond| {
                if let ForkCondition::TTD { fork_block, total_difficulty } = cond {
                    fork_block.map(|fork_block| (fork_block, total_difficulty))
                } else {
                    None
                }
            })
        };
        OpChainSpec {
            inner: ChainSpec {
                chain: self.inner.chain.expect("The chain is required"),
                genesis: self.inner.genesis.expect("The genesis is required"),
                genesis_hash: None,
                hardforks: self.inner.hardforks,
                paris_block_and_final_difficulty,
                deposit_contract: None,
                ..Default::default()
            },
        }
    }
}
