use serde::Serialize;

pub trait Discovery: Serialize {
    const COMPONENT: &'static str;
    fn unique_id(&self) -> &str;
    fn topic(&self, discovery_prefix: &str, node_id: &str) -> String {
        let component = Self::COMPONENT;
        let unique_id = self.unique_id();
        // https://www.home-assistant.io/integrations/mqtt#discovery-messages
        format!("{discovery_prefix}/{component}/{node_id}/{unique_id}/config")
    }
}
