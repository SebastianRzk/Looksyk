use crate::migration::model::ApplicationVersion;

pub fn get_current_application_version() -> ApplicationVersion {
    ApplicationVersion::new(env!("CARGO_PKG_VERSION"))
}
