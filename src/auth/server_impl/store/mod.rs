pub mod redb_kv;

use enum_dispatch::enum_dispatch;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KVError {
    #[error("deserialization err: {0}")]
    Deser(#[from] serde_json::Error),
    #[error(transparent)]
    ReDB(#[from] redb::Error),
}

#[enum_dispatch]
pub(crate) trait KVStore: Send {
    async fn read(&self, key: String) -> Result<Option<String>, KVError>;
    async fn write(&self, key: String, value: String) -> Result<(), KVError>;
}

#[derive(Clone)]
#[enum_dispatch(KVStore)]
pub enum KVStoreImpl {
    ReDB(redb_kv::ReDBKV),
}
