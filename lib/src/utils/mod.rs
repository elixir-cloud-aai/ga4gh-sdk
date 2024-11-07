pub mod configuration;
pub mod transport;
pub mod test_utils;
pub mod extension;
pub mod extension_manager;

pub fn expand_path_with_home_dir(path: &str) -> String {
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let full_path = home_dir.join(path);
    full_path.to_str().expect("Failed to convert path to string").to_string()
}