use gob_cloudflare::{api::kv::KvNamespace, CloudflareAuth};

use super::{KVError, KVStore};

#[derive(Clone)]
pub struct CloudflareKV {
    client: CloudflareAuth,
    namespace: KvNamespace,
}

impl CloudflareKV {
    pub fn new(client: CloudflareAuth, namespace: KvNamespace) -> Self {
        Self { client, namespace }
    }
}

impl KVStore for CloudflareKV {
    async fn read(&self, key: String) -> Result<Option<String>, KVError> {
        let req = self.namespace.read_kv(key);
        let res = match self.client.send_auth(req).await {
            Ok(res) => Some(res),
            Err(gob_cloudflare::Error::Cloudflare(e)) if e[0].code == 10009 => None,
            Err(e) => return Err(e.into()),
        };
        Ok(res)
    }

    async fn write(&self, key: String, value: String) -> Result<(), KVError> {
        let req = self.namespace.write_kv(key).value(value).metadata(&()).unwrap();
        self.client.send_auth_multipart(req).await?;
        Ok(())
    }
}
