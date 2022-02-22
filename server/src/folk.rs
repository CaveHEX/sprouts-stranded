use agent::Agent;
use identity::Identity;

pub struct Sprout {
    pub name: String,
    pub agent: Agent,
}

pub impl Identity for Sprout {
    pub fn name() -> String {
        return self.name;
    }
}
