use reth_db_api::database::Database;
use reth_storage_api::{BlockNumReader, HeaderProvider};
use reth_storage_errors::provider::ProviderResult;

/// A read-only database provider.
pub trait DBProviderRO<TX>: BlockNumReader + HeaderProvider + 'static {
    /// Provides a reference to underlying transaction.
    fn tx_ref(&self) -> &TX;
}

/// Database provider factory.
pub trait DatabaseProviderFactory<DB: Database> {
    /// Read-only database provider.
    type ProviderRO: DBProviderRO<DB::TX>;

    /// Create new read-only database provider.
    fn database_provider_ro(&self) -> ProviderResult<Self::ProviderRO>;
}
