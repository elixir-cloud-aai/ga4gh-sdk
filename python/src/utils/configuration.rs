use pyo3::prelude::*;
use ga4gh_sdk::utils::configuration::Configuration;
use crate::PyServiceType;
use url::Url;

#[pyclass(name = "Configuration", module = "GA4GH")]
pub struct PyConfiguration {
    pub inner: Configuration,
}

#[pymethods]
impl PyConfiguration {
    pub fn set_base_path(&mut self, base_path: String) -> PyResult<()> {
        self.inner.set_base_path(Url::parse(&base_path).unwrap());
        Ok(())
    }

    pub fn get_base_path(&mut self) -> String {
        self.inner.base_path.to_string()
    }

    pub fn from_file(&mut self, service_type: PyServiceType, service_config_path: String, extensions_config_path: String) -> PyResult<()> {
        self.inner = Configuration::from_file(
            service_type.into(),
            &service_config_path,
            &extensions_config_path
        ).map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(format!("{}", e)))?;
        Ok(())
    }
}