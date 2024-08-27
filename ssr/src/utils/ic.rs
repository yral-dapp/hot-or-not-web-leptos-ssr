use std::sync::Arc;

use ic_agent::{agent::AgentBuilder, Agent, Identity};

use crate::consts::AGENT_URL;

#[derive(Clone)]
pub struct AgentWrapper(Agent);

impl AgentWrapper {
    pub fn build(builder_func: impl FnOnce(AgentBuilder) -> AgentBuilder) -> Self {
        let mut builder = Agent::builder().with_url(AGENT_URL);
        builder = builder_func(builder);
        Self(builder.build().unwrap())
    }

    pub async fn get_agent(&self) -> &Agent {
        let agent = &self.0;
        #[cfg(any(feature = "local-bin", feature = "local-lib"))]
        {
            agent
                .fetch_root_key()
                .await
                .expect("AGENT: fetch_root_key failed");
        }
        agent
    }

    pub fn set_arc_id(&mut self, id: Arc<impl Identity + 'static>) {
        self.0.set_arc_identity(id);
    }
}
