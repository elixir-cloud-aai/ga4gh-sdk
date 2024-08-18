use ::ga4gh_sdk::clients::serviceinfo::ServiceInfo;
use ::ga4gh_sdk::utils::configuration::Configuration;
use pyo3::prelude::*;
use url::Url;

// Expose the Configuration struct to Python
#[pyclass(name = "Configuration", module = "ga4gh")]
struct PyConfiguration {
    inner: Configuration,
}

#[pymethods]
impl PyConfiguration {
    #[new]
    pub fn new(base_path: String) -> PyResult<Self> {
        let config = Configuration::new(Url::parse(&base_path).unwrap());
        Ok(PyConfiguration { inner: config })
    }

    pub fn set_base_path(&mut self, base_path: String) -> PyResult<()> {
        self.inner.set_base_path(Url::parse(&base_path).unwrap());
        Ok(())
    }
}

#[pyclass(name = "ServiceInfo", module = "ga4gh")]
struct PyServiceInfo {
    inner: ServiceInfo,
}

#[pymethods]
impl PyServiceInfo {
    #[new]
    pub fn new(py_config: &PyConfiguration) -> PyResult<Self> {
        let service_info = ServiceInfo::new(&py_config.inner).unwrap();
        Ok(PyServiceInfo {
            inner: service_info,
        })
    }

    pub fn get_service_info(&self) -> PyResult<String> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(self.inner.get());

        match result {
            Ok(service_info) => Ok(format!("Service: {:?}", service_info)),
            Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
                "Failed to get service info: {}",
                e
            ))),
        }
    }
}

// The Python module initialization
#[pymodule]
fn ga4gh(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyConfiguration>()?;
    m.add_class::<PyServiceInfo>()?;
    Ok(())
}
