pub mod redb_kv;

use enum_dispatch::enum_dispatch;
use serde::{de::DeserializeOwned, Serialize};
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
    async fn read_metadata_raw(&self, key: String) -> Result<Option<String>, KVError>;
    async fn write(&self, key: String, value: String) -> Result<(), KVError>;
    async fn write_metadata_raw(&self, key: String, metadata: String) -> Result<(), KVError>;

    async fn read_json_metadata<T: DeserializeOwned>(
        &self,
        key: String,
    ) -> Result<Option<T>, KVError> {
        let Some(metadata) = self.read_metadata_raw(key).await? else {
            return Ok(None);
        };
        serde_json::from_str(&metadata).map_err(Into::into)
    }

    async fn write_json_metadata<T: Serialize>(
        &self,
        key: String,
        metadata: T,
    ) -> Result<(), KVError> {
        let metadata = serde_json::to_string(&metadata).map_err(KVError::Deser)?;
        self.write_metadata_raw(key, metadata).await
    }
}

#[derive(Clone)]
#[enum_dispatch(KVStore)]
pub enum KVStoreImpl {
    ReDB(redb_kv::ReDBKV),
}
