use testcontainers::{
    core::{ContainerPort, IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    ContainerAsync, GenericImage, Image, ImageExt,
};
use yral_testcontainers::{
    backend::{self, YralBackend},
    metadata::{self, YralMetadata},
};

type MaybeContainer<I> = Option<ContainerAsync<I>>;

/// Holds all the containers that are started for local testing
/// we are required to hold these as ContainerAsync stops the container
/// on drop
#[derive(Default)]
pub struct TestContainers {
    redis: MaybeContainer<GenericImage>,
    metadata: MaybeContainer<YralMetadata>,
    backend: MaybeContainer<YralBackend>,
}

impl TestContainers {
    async fn start_image<I: Image>(image: I, port: impl Into<ContainerPort>) -> ContainerAsync<I> {
        let port = port.into();
        image
            .with_mapped_port(port.as_u16(), port)
            .start()
            .await
            .map_err(|e| format!("Failed to start container: {}", e))
            .unwrap()
    }
    pub async fn start_redis(&mut self) {
        let redis_im = GenericImage::new("redis", "7.2.4")
            .with_exposed_port(6379.tcp())
            .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"));
        self.redis = Some(Self::start_image(redis_im, 6379.tcp()).await);
    }

    pub async fn start_metadata(&mut self) {
        self.metadata = Some(Self::start_image(YralMetadata, metadata::REST_PORT).await);
    }

    pub async fn start_backend(&mut self) {
        self.backend = Some(Self::start_image(YralBackend, backend::AGENT_PORT).await);
    }
}
