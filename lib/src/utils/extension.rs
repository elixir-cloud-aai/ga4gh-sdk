// use serde::{Deserialize, Serialize};
// use std::io::Result;
use log::{info, debug};
use libloading::{Library, Symbol};
use std::collections::HashMap;
use serde_json::Value;
use crate::utils::configuration::InstalledExtension;
use std::fs::File;
use std::io::Read;
use std::error::Error;

type ExtensionInitFunction = unsafe extern "Rust" fn(Value) -> Vec<&'static [&'static str]>;
type ExtensionMethodFunction = unsafe extern "Rust" fn(Value) -> Value;

#[derive(Debug, Clone)]
pub struct ExtensionMethod {
    pub extension_name: String,
    pub unified_name: String,
    pub internal_name: String,
    pub method: Symbol<'static, ExtensionMethodFunction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Extension {
    pub name: String,
    pub version: String,
    pub path: Option<String>,
    pub description: Option<String>,

    #[serde(skip_serializing, default)]
    pub enabled: bool,
    #[serde(skip_serializing, default)]
    pub loaded: bool,
    #[serde(skip)]
    library: Option<Library>,
    #[serde(skip, default)]
    pub methods: HashMap<String, Vec<ExtensionMethod>>,
}


impl Clone for Extension {
    fn clone(&self) -> Self {
        Extension {
            name: self.name.clone(),
            version: self.version.clone(),
            path: self.path.clone(),
            description: self.description.clone(),
            enabled: self.enabled,
            loaded: self.loaded,
            library: None, // Exclude the library field from cloning
            methods: self.methods.clone(),
        }
    }
}

impl Extension {
    pub fn new(config: InstalledExtension) -> Result<Self, Box<dyn Error>> {
        let mut file = File::open(&config.path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let extension_definition_json: Value = serde_json::from_str(&contents)?;
        if !extension_definition_json.is_object() {
            return Err(format!("Extension definition file is not a valid JSON object: {}", config.path).into());
        }
        
        let mut extension: Extension = serde_json::from_value(extension_definition_json)?;

        extension.enabled = config.enabled;
        extension.loaded = false;
        extension.library = None;
        extension.methods = HashMap::new();

        Ok(extension)
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn load(&mut self, service_config: Value) {
        if self.enabled {
            debug!("Loading extension: {} v{}", self.name, self.version);
            if let Some(path) = &self.path {
                info!("Path: {}", path);
                unsafe {
                    self.library = Some(Library::new(path).expect("Failed to load extension shared library"));
                    debug!("Library loaded successfully");

                    if let Some(lib) = &self.library {
                        debug!("Getting init function");
                        let init_func: Symbol<ExtensionInitFunction> = lib.get(b"init").expect("Failed to load symbol");
                        debug!("Calling init function to obtain extension methods");

                        // let service_config_json = to_string(&service_config).expect("Failed to serialize service_config");
                        // let service_config_value: Value = json!(service_config_json);
                
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
            }
        } else {
            debug!("Extension {} is disabled", self.name);
        }
    }

    pub fn unload(&mut self) {
        if self.loaded {
            debug!("Unloading extension: {} v{}", self.name, self.version);
            if let Some(path) = &self.path {
                info!("Path: {}", path);
                unsafe {
                    let lib = Library::new(path).expect("Failed to load extension shared library");
                    let deinit_func: Symbol<unsafe extern "C" fn()> = lib.get(b"deinit").expect("Failed to load symbol");
                    debug!("Calling deinit function");
                    deinit_func();
                    self.loaded = false;
                    debug!("Deinit function called successfully");
                }
            }
        } else {
            debug!("Extension {} is disabled", self.name);
        }
    }
}

