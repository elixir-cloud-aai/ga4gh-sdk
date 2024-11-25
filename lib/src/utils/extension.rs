//! Extension Module
//!
//! This module provides structures and functions to manage extensions, including loading, initializing, and invoking methods from extensions.
//!
//! # Example
//!
//! ```rust
//! use extension_manager::InstalledExtension;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Load an extension from a file
//!     let extension = InstalledExtension::from_file("/path/to/extension.json")?;
//!     println!("Loaded extension: {:?}", extension);
//!     Ok(())
//! }
//! ```

use log::{info, debug};
use libloading::{Library, Symbol};
use std::collections::HashMap;
use serde_json::Value;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::error::Error;

type ExtensionInitFunction = unsafe extern "Rust" fn(Value) -> Vec<&'static [&'static str]>;
type ExtensionMethodFunction = unsafe extern "Rust" fn(Value) -> Value;

/// Represents a method provided by an extension.
#[derive(Debug, Clone)]
pub struct ExtensionMethod {
    /// The name of the extension providing this method.
    pub extension_name: String,
    /// The unified name of the method.
    pub unified_name: String,
    /// The internal name of the method.
    pub internal_name: String,
    /// The function pointer to the method implementation.
    pub method: Symbol<'static, ExtensionMethodFunction>,
}

/// Represents an extension with its metadata.
#[derive(Debug, Serialize, Deserialize)]
pub struct Extension {
    /// The name of the extension.
    pub name: String,
    /// The version of the extension.
    pub version: String,
    /// The file path to the extension.
    pub path: String,
    /// SHA512 checksum of the extension binary.
    pub checksum: String,
    /// An optional description of the extension.
    pub description: Option<String>,
}

/// Represents an installed extension with its runtime state.
#[derive(Debug, Serialize, Deserialize)]
pub struct InstalledExtension {
    /// The name of the extension.
    pub name: String,
    /// The version of the extension.
    pub version: String,
    /// The path to the extension's definition file.
    #[serde(rename = "definition-path")]
    pub definition_path: String,
    /// The path to the extension's library file.
    #[serde(rename = "library-path")]
    pub library_path: String,

    /// Indicates whether the extension is enabled.
    #[serde(skip_serializing, default)]
    pub enabled: bool,
    /// Indicates whether the extension is loaded.
    #[serde(skip_serializing, default)]
    pub loaded: bool,
    /// The loaded library of the extension.
    #[serde(skip)]
    pub library: Option<Library>,
    /// The methods provided by the extension.
    #[serde(skip, default)]
    pub methods: HashMap<String, Vec<ExtensionMethod>>,
}

impl InstalledExtension {
    /// Loads an `InstalledExtension` from a JSON file.
    ///
    /// # Arguments
    ///
    /// * `file_path` - A string slice that holds the path to the JSON file.
    ///
    /// # Returns
    ///
    /// * `Result<Self, Box<dyn Error>>` - Returns an `InstalledExtension` on success, or an error on failure.
    ///
    /// # Example
    ///
    /// ```rust
    /// let extension = InstalledExtension::from_file("/path/to/extension.json")?;
    /// println!("Loaded extension: {:?}", extension);
    /// ```
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let json: Value = serde_json::from_str(&contents)?;
        if !json.is_object() {
            return Err(format!("Extension definition file is not a valid JSON object: {}", file_path).into());
        }
        let mut extension: InstalledExtension = serde_json::from_value(json)?;

        extension.loaded = false;
        extension.library = None;
        extension.methods = HashMap::new();

        Ok(extension)
    }

    /// Enables the extension.
    ///
    /// This method sets the `enabled` flag to `true`.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut extension = InstalledExtension::from_file("/path/to/extension.json")?;
    /// extension.enable();
    /// assert!(extension.enabled);
    /// ```
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disables the extension.
    ///
    /// This method sets the `enabled` flag to `false`.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut extension = InstalledExtension::from_file("/path/to/extension.json")?;
    /// extension.disable();
    /// assert!(!extension.enabled);
    /// ```
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Loads the extension if it is enabled.
    ///
    /// This method attempts to load the extension's shared library and initialize it with the provided service configuration.
    /// It retrieves the extension's methods and stores them in the `methods` map.
    ///
    /// # Arguments
    ///
    /// * `service_config` - A `serde_json::Value` containing the service configuration to pass to the extension's initialization function.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it involves loading a shared library and calling external functions.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut extension = InstalledExtension::from_file("/path/to/extension.json")?;
    /// let service_config = serde_json::json!({});
    /// extension.load(service_config);
    /// assert!(extension.loaded);
    /// ```
    pub fn load(&mut self, service_config: Value) {
        if self.enabled {
            debug!("Loading extension lib: {} v{}", self.name, self.version);
            info!("Path: {}", self.library_path);
            unsafe {
                self.library = Some(Library::new(&self.library_path).expect("Failed to load extension shared library"));
                debug!("Library loaded successfully");

                if let Some(lib) = &self.library {
                    debug!("Getting init function");
                    let init_func: Symbol<ExtensionInitFunction> = lib.get(b"init").expect("Failed to load symbol");
                    debug!("Calling init function to obtain extension methods");
            
                    let methods = init_func(service_config);
                    debug!("Init function called successfully, methods obtained:");
                    for method in methods {
                        let unified_method_name = method[0];
                        let internal_method_name = method[1];
                        let symbol_name = method[2];
                        debug!("category={}, method={}, symbol_name={}", unified_method_name, internal_method_name, symbol_name);
                        debug!("Getting `{}` function symbol", symbol_name);
                        let symbol: Symbol<ExtensionMethodFunction> = lib.get(symbol_name.as_bytes()).expect("Failed to load symbol");
                        self.methods.entry(unified_method_name.to_string()).or_insert_with(Vec::new).push(ExtensionMethod {
                            extension_name: self.name.clone(),
                            unified_name: unified_method_name.to_string(),
                            internal_name: internal_method_name.to_string(),
                            method: std::mem::transmute::<Symbol<ExtensionMethodFunction>, Symbol<'static, ExtensionMethodFunction>>(symbol),
                        });
                        debug!("Symbol loaded successfully");
                        info!("Method loaded: {}", internal_method_name);
                    }
                    self.loaded = true;
                }
            }
        } else {
            debug!("Extension {} is disabled", self.name);
        }
    }

    /// Unloads the extension by calling its `deinit` function.
    ///
    /// This method checks if the extension is currently loaded. If it is, it attempts to load the extension's shared library
    /// and retrieve the `deinit` function symbol. It then calls the `deinit` function to properly unload the extension.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it involves loading a shared library and calling an external function.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut extension = InstalledExtension::from_file("/path/to/extension.json")?;
    /// extension.load(serde_json::json!({}));
    /// extension.unload();
    /// assert!(!extension.loaded);
    /// ```
    pub fn unload(&mut self) {
        if self.loaded {
            debug!("Unloading extension: {} v{}", self.name, self.version);
            info!("Path: {}", self.library_path);
            unsafe {
                let lib = Library::new(&self.library_path).expect("Failed to load extension shared library");
                let deinit_func: Symbol<unsafe extern "C" fn()> = lib.get(b"deinit").expect("Failed to load symbol");
                debug!("Calling deinit function");
                deinit_func();
                self.loaded = false;
                debug!("Deinit function called successfully");
            }
        } else {
            debug!("Extension {} is disabled", self.name);
        }
    }
}

impl Clone for InstalledExtension {
    /// Creates a clone of the `InstalledExtension`.
    ///
    /// This method creates a new `InstalledExtension` instance with the same values for all fields except for the `library` field,
    /// which is set to `None` in the cloned instance. This is because the `library` field represents a loaded shared library,
    /// which cannot be safely cloned.
    fn clone(&self) -> Self {
        InstalledExtension {
            name: self.name.clone(),
            version: self.version.clone(),
            definition_path: self.definition_path.clone(),
            library_path: self.library_path.clone(),
            enabled: self.enabled,
            loaded: self.loaded,
            library: None, // Exclude the library field from cloning
            methods: self.methods.clone(),
        }
    }
}

impl Extension {
    /// Loads an `Extension` from a JSON file.
    ///
    /// This method reads the contents of the specified JSON file and deserializes it into an `Extension` instance.
    ///
    /// # Arguments
    ///
    /// * `file` - A string slice that holds the path to the JSON file.
    ///
    /// # Returns
    ///
    /// * `Result<Self, Box<dyn std::error::Error>>` - Returns an `Extension` on success, or an error on failure.
    ///
    /// # Example
    ///
    /// ```rust
    /// let extension = Extension::from_file("/path/to/extension.json")?;
    /// println!("Loaded extension: {:?}", extension);
    /// ```
    pub fn from_file(file: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(file)?;
        let extension: Extension = serde_json::from_str(&contents)?;
        Ok(extension)
    }
}

