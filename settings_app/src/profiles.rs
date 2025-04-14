use shared::config::Profile;

#[allow(dead_code)]
pub fn get_profile_list(profiles: &[Profile]) -> Vec<String> {
    profiles.iter().map(|p| p.name.clone()).collect()
}
