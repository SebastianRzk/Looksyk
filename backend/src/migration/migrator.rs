use crate::io::fs::version::save_user_data_version;
use crate::migration::migration_1_10_2::migriere_1_10_2;
use crate::migration::model::ApplicationVersion;
use crate::state::application_state::GraphRootLocation;

pub fn run_migrations(
    current_version: ApplicationVersion,
    data_state_version: ApplicationVersion,
    user_application_directory: &GraphRootLocation,
) {
    println!(
        "Running migrations from version {} to {}",
        data_state_version, current_version
    );

    if data_state_version > current_version {
        panic!("Der Datenbestand ist neuer als die aktuelle Version. Bitte aktualisieren Sie die Anwendung.");
    }

    if data_state_version == current_version {
        println!("Keine Migration erforderlich.");
        return;
    }

    let version_1_10_2 = ApplicationVersion::new("1.10.2");
    if data_state_version < version_1_10_2 {
        print_running_migration(version_1_10_2);
        migriere_1_10_2(user_application_directory)
    }

    save_user_data_version(user_application_directory, &current_version);
    println!(
        "Migrated from version {} to {}",
        data_state_version, current_version
    );
}

fn print_running_migration(migration: ApplicationVersion) {
    println!("Running migration: {}", migration);
}