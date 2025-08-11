use crate::state::application_state::GraphRootLocation;
use crate::sync::git::application_port::git_sync_application_port::write_default_disabled_config_to_disk;

pub fn migriere_1_14_0(graph_root_location: &GraphRootLocation) {
    write_default_disabled_config_to_disk(graph_root_location)
}
