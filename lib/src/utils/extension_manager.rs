use crate::utils::extension::Extension;
use crate::utils::extension::ExtensionMethod;
use crate::utils::configuration::InstalledExtensions;
// use crate::utils::configuration::Configuration;
use crate::utils::configuration::ServiceExtensionsConfiguration;
use log::{debug, info, warn};
use std::error::Error;
use std::fs;
use crate::utils::expand_path_with_home_dir;
use serde_json::Value;
use std::path::Path;

#[derive(Default, Debug, Clone)]
pub struct ExtensionManager {
    extensions: Vec<Extension>,
}

impl ExtensionManager {
    pub fn new(installed_extensions: InstalledExtensions, service_config: Option<ServiceExtensionsConfiguration>) -> Result<Self, Box<dyn Error>> {
        let mut extensions = Vec::new();

        for extension_global_config in installed_extensions.extensions {
            let mut extension = Extension::new(extension_global_config)?;
            if let Some(service_config) = &service_config {
                extension.load(service_config.get_extension_config(&extension.name).clone());
            }
            extensions.push(extension);
        }

        Ok(ExtensionManager { extensions })
    }

    pub fn get_extensions(&self) -> &Vec<Extension> {
        &self.extensions
    }

    pub fn register_extension(&mut self, extension: Extension) {
        self.extensions.push(extension);
    }

    // pub fn load_extensions(&mut self) {
    //     if !self.extensions.is_empty() {
    //         debug!("Loading extensions");
    //         for extension in &mut self.extensions {
    //             extension.load();
    //         }
    //     }
    // }

    // pub fn unload_extensions(&mut self) {
    //     for extension in &mut self.extensions {
    //         extension.unload();
    //     }
    // }

    pub fn lookup_extension_methods(&self, unified_method_name: &str) -> Vec<&ExtensionMethod> {
        debug!("Looking up extension methods for '{}'", unified_method_name);
        self.extensions
            .iter()
            .filter(|e| e.enabled)
            .flat_map(|e| e.methods.values().flatten())
            .filter(|m| m.unified_name == unified_method_name)
            .collect()
    }

    pub fn add_extension_record(&self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        info!("Adding extension record for file: {}", file);
        let config_file_path = expand_path_with_home_dir(".ga4gh-cli/extensions.json");

        // Read the existing JSON file
        let mut config_json: Value = if Path::new(&config_file_path).exists() {
            let contents = fs::read_to_string(&config_file_path)?;
            serde_json::from_str(&contents)?
        } else {
            serde_json::json!({"extensions": {}})
        };
    
        // Convert the extensions to a HashMap
        let extensions = config_json["extensions"].as_object_mut().ok_or("Invalid JSON format")?;

        // Check if the extension already exists
        if extensions.contains_key(file) {
            warn!("Extension '{}' already exists in the configuration.", file);
            return Ok(());
        }

        // Add or update the extension record
        let new_extension_record = serde_json::json!({
            "enabled": false
        });
        extensions.insert(file.to_string(), new_extension_record);
        debug!("Added extension '{}' to the configuration.", file);
    
        // Write the updated JSON back to the file
        fs::write(config_file_path, serde_json::to_string_pretty(&config_json)?)?;
    
        Ok(())
    }

    pub fn enable_extension(&self, _name: &str) -> Result<(), Box<dyn std::error::Error>> {
        // info!("Enabling extension: {}", name);
        // let config_file_path = Configuration::get_config_path(".ga4gh-cli/extensions.json".to_string())?;

        // // Read the existing JSON file
        // let mut config_json: Value = if config_file_path.exists() {
        //     let contents = fs::read_to_string(&config_file_path)?;
        //     serde_json::from_str(&contents)?
        // } else {
        //     serde_json::json!({"extensions": {}})
        // };
    
        // // Convert the extensions to a HashMap
        // let extensions = config_json["extensions"].as_object_mut().ok_or("Invalid JSON format")?;

        // // Check if the extension exists
        // if !extensions.contains_key(name) {
        //     warn!("Extension '{}' does not exist in the configuration.", name);
        //     return Ok(());
        // }

        // // Enable the extension
        // extensions[name]["enabled"] = serde_json::Value::Bool(true);
        // debug!("Enabled extension '{}'.", name);
    
        // // Write the updated JSON back to the file
        // fs::write(config_file_path, serde_json::to_string_pretty(&config_json)?)?;
    
        Ok(())
    }
}


        // // read extensions configuration
        // let config_json = Configuration::get_configuration_from_file(".ga4gh-cli/extensions.json".to_string())?;
        // for extension in config_json.get("extensions").and_then(|v| v.as_array()).unwrap_or(&Vec::new()) {
        //     if let Some(file) = extension.get("name").and_then(|v| v.as_str()) {
        //         if let Some(enabled) = extension.get("enabled").and_then(|v| v.as_bool()) {
        //             if enabled {
        //                 let extension_path = Configuration::get_config_path(".ga4gh-cli/extensions/".to_owned() + file)?;
        //                 debug!("Loading extension definition file: {}", extension_path.as_path().to_str().unwrap_or_default());
        //                 let mut extension = Extension::from_file(extension_path.as_path().to_str().unwrap_or_default())?;
        //                 extension.enable();
        //                 info!("Registering extension: {}", file);
        //                 config.extensions_manager.register_extension(extension);
        //             }
        //         } else {
        //             warn!("extensions.json file record is missing a 'name' field");
        //         }
        //     } else {
        //         warn!("extensions.json file record is missing a 'name' field");
        //     }
        //     config.extensions_manager.load_extensions();
        // }