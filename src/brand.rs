use base::config::keys;
use hbb_common::config;

const APP_NAME: &str = "SamDesk";

// Defaults only: each device can still change them in Settings > Security.
// Out of the box nobody can connect until the owner sets a permanent password,
// and there is no click-to-accept prompt a stranger could talk a user into.
const DEFAULT_SETTINGS: &[(&str, &str)] = &[
    (keys::OPTION_VERIFICATION_METHOD, "use-permanent-password"),
    (keys::OPTION_APPROVE_MODE, "password"),
];

pub fn apply() {
    *config::APP_NAME.write().unwrap() = APP_NAME.to_owned();
    let mut defaults = config::DEFAULT_SETTINGS.write().unwrap();
    for (k, v) in DEFAULT_SETTINGS {
        defaults.insert((*k).to_owned(), (*v).to_owned());
    }
}
