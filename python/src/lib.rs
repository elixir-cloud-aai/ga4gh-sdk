use ::ga4gh_sdk::clients::serviceinfo::ServiceInfo;
use ga4gh_sdk::clients::tes::model::ListTasksParams;
use ga4gh_sdk::clients::tes::models::TesListTasksResponse;
use ga4gh_sdk::clients::tes::{Task, TES};
use ::ga4gh_sdk::utils::configuration::Configuration;
use ::ga4gh_sdk::utils::transport::Transport;
use pyo3::prelude::*;
use tokio::runtime::Runtime;
use url::Url;

/// Expose the Task struct to Python
#[pyclass(name = "Task", module = "ga4gh")]
struct PyTask {
    inner: Task,
}

#[pymethods]
impl PyTask {
    #[new]
    pub fn new(id: String, transport: &PyTransport) -> PyResult<Self> {
        Ok(PyTask {
            inner: Task::new(id, transport.inner.clone()),
        })
    }

    pub fn status(&self) -> PyResult<String> {
        let rt = Runtime::new().unwrap();
        match rt.block_on(self.inner.status()) {
            Ok(state) => Ok(format!("{:?}", state)),
            Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
                "Failed to get task status: {}",
                e
            ))),
        }
    }

    pub fn cancel(&self) -> PyResult<String> {
        let rt = Runtime::new().unwrap();
        match rt.block_on(self.inner.cancel()) {
            Ok(response) => Ok(response.to_string()),
            Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
                "Failed to cancel task: {}",
                e
            ))),
        }
    }
}

/// Expose the TES struct to Python
#[pyclass(unsendable, name = "TES", module = "ga4gh")]
struct PyTES {
    inner: TES,
}

#[pymethods]
impl PyTES {
    #[new]
    pub fn new(config: &PyConfiguration) -> PyResult<Self> {
        let rt = Runtime::new().unwrap();
        match rt.block_on(TES::new(&config.inner)) {
            Ok(instance) => Ok(PyTES { inner: instance }),
            Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
                "Failed to create TES instance: {}",
                e
            ))),
        }
    }

    pub fn create(&self, task: &PyTesTask) -> PyResult<PyTask> {
        let rt = Runtime::new().unwrap();
        match rt.block_on(self.inner.create(task.inner.clone())) {
            Ok(task) => Ok(PyTask { inner: task }),
            Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
                "Failed to create task: {}",
                e
            ))),
        }
    }

    pub fn get(&self, view: &str, id: &str) -> PyResult<PyTesTask> {
        let rt = Runtime::new().unwrap();
        match rt.block_on(self.inner.get(view, id)) {
            Ok(task) => Ok(PyTesTask { inner: task }),
            Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
                "Failed to get task: {}",
                e
            ))),
        }
    }
}

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

    pub fn get_base_path(&mut self) -> String {
        self.inner.base_path.to_string()
    }
    
}

// Expose the ServiceInfo struct to Python
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

// Expose the Transport struct to Python
#[pyclass(name = "Transport", module = "ga4gh")]
struct PyTransport {
    inner: Transport,
}

#[pymethods]
impl PyTransport {
    #[new]
    pub fn new(py_config: &PyConfiguration) -> PyResult<Self> {
        let transport = Transport::new(&py_config.inner);
        Ok(PyTransport { inner: transport })
    }

    pub fn get(&self, endpoint: String, params: Option<String>) -> PyResult<String> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let params_value = params.map(|p| serde_json::from_str(&p).unwrap());
        let result = rt.block_on(self.inner.get(&endpoint, params_value));

        match result {
            Ok(response) => Ok(response),
            Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
                "GET request failed: {}",
                e
            ))),
        }
    }

    pub fn post(&self, endpoint: String, data: Option<String>) -> PyResult<String> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let data_value = data.map(|d| serde_json::from_str(&d).unwrap());
        let result = rt.block_on(self.inner.post(&endpoint, data_value));

        match result {
            Ok(response) => Ok(response),
            Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
                "POST request failed: {}",
                e
            ))),
        }
    }

    pub fn put(&self, endpoint: String, data: String) -> PyResult<String> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let data_value = serde_json::from_str(&data).unwrap();
        let result = rt.block_on(self.inner.put(&endpoint, data_value));

        match result {
            Ok(response) => Ok(response),
            Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
                "PUT request failed: {}",
                e
            ))),
        }
    }

    pub fn delete(&self, endpoint: String) -> PyResult<String> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(self.inner.delete(&endpoint));

        match result {
            Ok(response) => Ok(response),
            Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
                "DELETE request failed: {}",
                e
            ))),
        }
    }
}


#[pyclass(name = "PyTesTask", module = "ga4gh")]
struct PyTesTask {
    inner: ga4gh_sdk::clients::tes::models::TesTask,
}

#[pyclass(name = "ListTasksParams", module = "ga4gh")]
struct PyListTasksParams {
    _inner: ListTasksParams,
}

pub struct PyTesListTasksResponse {
    _inner: TesListTasksResponse,
}


// The Python module initialization
#[pymodule]
fn ga4gh(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyConfiguration>()?;
    m.add_class::<PyServiceInfo>()?;
    m.add_class::<PyTransport>()?;
    m.add_class::<PyTES>()?;
    m.add_class::<PyTesTask>()?;
    Ok(())
}