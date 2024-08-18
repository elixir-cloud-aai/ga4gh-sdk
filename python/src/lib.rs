use pyo3::prelude::*;
use tokio::runtime::Runtime;
use crate::utils::configuration::Configuration;
use crate::utils::transport::Transport;
use crate::models;

#[pyclass]
struct PyServiceInfo {
    inner: ServiceInfo,
    rt: Runtime,
}

#[pymethods]
impl PyServiceInfo {
    #[new]
    fn new(config_url: &str) -> PyResult<Self> {
        let rt = Runtime::new().unwrap();
        let config = Configuration::new(url::Url::parse(config_url).unwrap());
        let inner = ServiceInfo::new(&config).unwrap();
        Ok(PyServiceInfo { inner, rt })
    }

    fn get(&self) -> PyResult<String> {
        let result = self.rt.block_on(self.inner.get());
        match result {
            Ok(service) => Ok(serde_json::to_string(&service).unwrap()),
            Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(e.to_string())),
        }
    }
}

#[pymodule]
fn ga4gh_sdk(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyServiceInfo>()?;
    Ok(())
}