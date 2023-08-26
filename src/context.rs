use std::sync::OnceLock;

use crate::AllowedDnsNames;
use crate::AllowedDnsServers;
use crate::OldFunctions;

static CONTEXT: OnceLock<Context> = OnceLock::new();

pub(crate) struct Context {
    pub(crate) old_functions: OldFunctions,
    pub(crate) allowed_names: AllowedDnsNames,
    pub(crate) allowed_servers: AllowedDnsServers,
}

impl Context {
    fn new() -> Self {
        Self {
            old_functions: OldFunctions::new(),
            allowed_names: AllowedDnsNames::new(),
            allowed_servers: AllowedDnsServers::new(),
        }
    }

    pub(crate) fn get() -> &'static Self {
        CONTEXT.get_or_init(Self::new)
    }
}
