use crate::io::fs::version::save_graph_version;
use crate::migration::migration_1_10_2::migriere_1_10_2;
use crate::migration::migration_1_14_0::migriere_1_14_0;
use crate::migration::model::ApplicationVersion;
use crate::state::application_state::GraphRootLocation;

pub fn run_migrations(
    current_version: &ApplicationVersion,
    data_state_version: &ApplicationVersion,
    user_application_directory: &GraphRootLocation,
) -> MigrationResult {
    println!("Running migrations from version {data_state_version} to {current_version}");

    if data_state_version > current_version {
        panic!(
            "Data state version {data_state_version} is newer than current version {current_version}. This is not supported. Please update the application."
        );
    }

    if data_state_version == current_version {
        println!("No migration needed. Current version is already up to date: {current_version}");
        return MigrationResult::EverythingUpToDate;
    }

    let version_1_10_2 = ApplicationVersion::new("1.10.2");
    if data_state_version < &version_1_10_2 {
        print_running_migration(version_1_10_2);
        migriere_1_10_2(user_application_directory);
    }

    let version_1_14_0 = ApplicationVersion::new("1.14.0");
    if data_state_version < &version_1_14_0 {
        print_running_migration(version_1_14_0);
        migriere_1_14_0(user_application_directory);
    }

    save_graph_version(user_application_directory, current_version);
    println!("Migrated from version {data_state_version} to {current_version}");
    MigrationResult::MigratedSomething
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MigrationResult {
    MigratedSomething,
    EverythingUpToDate,
}

fn print_running_migration(migration: ApplicationVersion) {
    println!("Running migration: {migration}");
}
