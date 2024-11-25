//! Extension Manager
//!
//! This module provides functionality to manage extensions, including adding, removing, enabling, and disabling extensions.
//! It handles reading and writing the extensions configuration file, managing the extension files and directories, and updating the status of extensions.
//!
//! # Example
//!
//! ```rust
//! use extension_manager::ExtensionManager;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut manager = ExtensionManager::default();
//!
//!     // Initialize the manager with a configuration path
//!     manager.init("/path/to/config.json")?;
//!
//!     // Add a new extension
//!     manager.add_extension("/path/to/extension-file.json").await?;
//!
//!     // Enable an extension by name
//!     manager.enable_extension("extension-name")?;
//!
//!     // Disable an extension by name
//!     manager.disable_extension("extension-name")?;
//!
//!     // Remove an extension by name
//!     manager.remove_extension("extension-name")?;
//!
//!     Ok(())
//! }
//! ```

use crate::utils::extension::Extension;
use crate::utils::extension::ExtensionMethod;
use crate::utils::extension::InstalledExtension;
use crate::utils::configuration::ServiceExtensionsConfiguration;
use log::{debug, info, warn, error};
use std::error::Error;
use std::fs;
use crate::utils::expand_path_with_home_dir;
use std::io::Read;
use serde_json::Value;
use std::collections::HashMap;

type InstalledExtensions = Vec<InstalledExtension>;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionManager {
    extensions: InstalledExtensions,
    #[serde(skip)]
    config_path: String,
}

impl ExtensionManager {
    /// Initializes the `ExtensionManager` by reading the extensions configuration file.
    ///
    /// This method reads the configuration file specified by `extensions_config_path` and initializes
    /// the extensions. It also optionally takes a `ServiceExtensionsConfiguration` for additional
    /// configuration of the extensions.
    ///
    /// Example configuration JSON of the globally installed extensions:
    /// ```json
    /// {
    ///     "extensions": [
    ///       {
    ///         "name": "extension-name",
    ///         "path": "/full/path/to/extension-name.ga4gh-sdk-extension.json",
    ///         "enabled": true
    ///       }
    ///     ]
    /// }
    /// ```
    ///
    /// # Arguments
    ///
    /// * `extensions_config_path` - A string slice that holds the path to the extensions configuration file.
    /// * `service_config` - An optional `ServiceExtensionsConfiguration` that provides additional configuration for the extensions.
    ///
    /// # Returns
    ///
    /// * `Result<Self, Box<dyn Error>>` - Returns an `ExtensionManager` instance on success, or an error on failure.
    pub fn init(extensions_config_path: &str, service_config: Option<ServiceExtensionsConfiguration>) -> Result<Self, Box<dyn Error>> {
        debug!("Reading extensions configuration file: {}", extensions_config_path);
        let mut file = crate::utils::open_or_create_file(extensions_config_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let mut installed_extensions: ExtensionManager = if contents.is_empty() {
            let default_extensions = ExtensionManager::default();
            let default_json = serde_json::to_string(&default_extensions)?;
            serde_json::to_writer_pretty(&mut file, &default_extensions)?;
            default_extensions
        } else {
            serde_json::from_str(&contents).unwrap_or_else(|_| ExtensionManager::default())
        };  

        for instaslled_extension in &mut installed_extensions.extensions {
            if let Some(service_config) = &service_config {
                instaslled_extension.load(service_config.get_extension_config(&instaslled_extension.name).clone());
            }
        }

        installed_extensions.config_path = extensions_config_path.to_string();

        Ok(installed_extensions)
    }

    /// Returns a reference to the list of installed extensions.
    ///
    /// # Returns
    ///
    /// * `&Vec<InstalledExtension>` - A reference to the vector of installed extensions.
    pub fn get_extensions(&self) -> &Vec<InstalledExtension> {
        &self.extensions
    }

    /// Registers a new extension by adding it to the list of installed extensions.
    ///
    /// # Arguments
    ///
    /// * `extension` - An `InstalledExtension` instance to be added to the list of installed extensions.
    pub fn register_extension(&mut self, extension: InstalledExtension) {
        self.extensions.push(extension);
    }

    /// Looks up extension methods by their unified method name.
    ///
    /// # Arguments
    ///
    /// * `unified_method_name` - A string slice that holds the unified method name to look up.
    ///
    /// # Returns
    ///
    /// * `Vec<&ExtensionMethod>` - A vector of references to `ExtensionMethod` instances that match the unified method name.
    pub fn lookup_extension_methods(&self, unified_method_name: &str) -> Vec<&ExtensionMethod> {
        debug!("Looking up extension methods for '{}'", unified_method_name);
        self.extensions
            .iter()
            .filter(|e| e.enabled)
            .flat_map(|e| e.methods.values().flatten())
            .filter(|m| m.unified_name == unified_method_name)
            .collect::<Vec<_>>()
    }

    /// Adds a new extension by reading the extension file, copying it to the appropriate directory,
    /// and updating the extensions configuration file.
    ///
    /// # Arguments
    ///
    /// * `extension_file` - A string slice that holds the path to the extension file to be added.
    ///
    /// # Returns
    ///
    /// * `Result<(), Box<dyn std::error::Error>>` - Returns `Ok(())` on success, or an error on failure.
    ///
    /// # Example
    ///
    /// ```rust
    /// let manager = ExtensionManager::default();
    /// manager.add_extension("/path/to/extension-file.json").await?;
    /// ```
    pub async fn add_extension(&self, extension_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        info!("Adding extension record for file: {}", extension_file);
        let extension: Extension = Extension::from_file(extension_file)?;
        
        let mut file = crate::utils::open_or_create_file(self.config_path.as_str())?;
        let mut extensions_json_content = String::new();
        file.read_to_string(&mut extensions_json_content)?;
        debug!("Reading extensions configuration file: {}", self.config_path);
        let mut extensions_json: ExtensionManager = serde_json::from_str(&extensions_json_content)?;

        if extensions_json.extensions.iter().any(|ext| ext.name == extension.name) {
            warn!("Extension '{}' already exists in the configuration.", extension.name);
            return Ok(());
        }

        let extension_lib_filename = crate::utils::extract_filename_from_url(extension.path.as_str()).unwrap();
        let extension_folder_path = expand_path_with_home_dir(format!(".ga4gh/extensions/{}/", extension.name).as_str());
        if let Err(e) = fs::create_dir_all(&extension_folder_path) {
            error!("Failed to create directory: {}", e);
        };
        let local_extension_lib_path = format!("{}/{}", extension_folder_path, extension_lib_filename);
        debug!("Copying extension file to {}", extension_folder_path);
        crate::utils::copy_file_to_folder(extension_file, &extension_folder_path)?;
        debug!("Downloading extension library from {} to {}", extension.path.as_str(), local_extension_lib_path);
        crate::utils::download_file(&extension.path, local_extension_lib_path.as_str()).await;

        let installed_definition_file_path = expand_path_with_home_dir(format!(".ga4gh/extensions/{}/{}.ga4gh-sdk-extension.json", extension.name, extension.name).as_str());
        let full_definition_file_path = fs::canonicalize(&installed_definition_file_path)?.to_str().unwrap().to_string();
        let new_extension_record = InstalledExtension {
            name: extension.name.clone(),
            version: extension.version.clone(),
            definition_path: full_definition_file_path,
            library_path: local_extension_lib_path,
            enabled: false,
            loaded: false,
            library: None,
            methods: HashMap::new(),
        };
        debug!("Adding extension record: {:?}", new_extension_record);
        extensions_json.extensions.push(new_extension_record); // TODO: should the struct be updated?
        debug!("Added extension '{}' to the configuration.", extension_file);
    
        // Write the updated JSON back to the file
        fs::write(&self.config_path, serde_json::to_string_pretty(&extensions_json)?)?;
    
        Ok(())
    }

    /// Removes an extension by its name from the configuration file and deletes its directory.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the extension to be removed.
    ///
    /// # Returns
    ///
    /// * `Result<(), Box<dyn std::error::Error>>` - Returns `Ok(())` on success, or an error on failure.
    ///
    /// # Example
    ///
    /// ```rust
    /// let manager = ExtensionManager::default();
    /// manager.remove_extension("extension-name")?;
    /// ```
    pub fn remove_extension(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        info!("Removing extension: {}", name);
        let contents = fs::read_to_string(&self.config_path)?;
        let mut extensions_json: Value = serde_json::from_str(&contents)?;
        if let Some(extensions) = extensions_json["extensions"].as_array_mut() {
            extensions.retain(|extension| extension["name"] != name);
        }
        fs::write(&self.config_path, serde_json::to_string_pretty(&extensions_json)?)?;

        let extension_folder_path = expand_path_with_home_dir(format!(".ga4gh/extensions/{}/", name).as_str());
        if fs::metadata(&extension_folder_path).is_ok() {
            fs::remove_dir_all(&extension_folder_path)?;
        }

        Ok(())
    }

    /// Updates the status of an extension (enabled or disabled) in the configuration file.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the extension to be updated.
    /// * `enabled` - A boolean indicating whether the extension should be enabled (`true`) or disabled (`false`).
    ///
    /// # Returns
    ///
    /// * `Result<(), Box<dyn std::error::Error>>` - Returns `Ok(())` on success, or an error on failure.
    fn update_extension_status(&self, name: &str, enabled: bool) -> Result<(), Box<dyn std::error::Error>> {
        let status = if enabled { "Enabling" } else { "Disabling" };
        info!("{} extension: {}", status, name);

        let contents = fs::read_to_string(&self.config_path)?;
        let mut extensions_json: Value = serde_json::from_str(&contents)?;

        if let Some(extensions) = extensions_json["extensions"].as_array_mut() {
            for extension in extensions.iter_mut() {
                if extension["name"] == name {
                    extension["enabled"] = enabled.into();
                }
            }
        }

        fs::write(&self.config_path, serde_json::to_string_pretty(&extensions_json)?)?;
        Ok(())
    }

    /// Enables an extension by its name in the configuration file.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the extension to be enabled.
    ///
    /// # Returns
    ///
    /// * `Result<(), Box<dyn std::error::Error>>` - Returns `Ok(())` on success, or an error on failure.
    ///
    /// # Example
    ///
    /// ```rust
    /// let manager = ExtensionManager::default();
    /// manager.enable_extension("extension-name")?;
    /// ```
    pub fn enable_extension(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.update_extension_status(name, true)
    }

    /// Disables an extension by its name in the configuration file.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the extension to be disabled.
    ///
    /// # Returns
    ///
    /// * `Result<(), Box<dyn std::error::Error>>` - Returns `Ok(())` on success, or an error on failure.
    ///
    /// # Example
    ///
    /// ```rust
    /// let manager = ExtensionManager::default();
    /// manager.disable_extension("extension-name")?;
    /// ```
    pub fn disable_extension(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.update_extension_status(name, false)
    }
}
