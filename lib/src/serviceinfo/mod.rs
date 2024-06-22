mod models;

use crate::transport::Transport;
use crate::configuration::Configuration;

#[derive(Clone)]
pub struct ServiceInfo {
    transport: Transport,
}

impl ServiceInfo {
    pub fn new(transport: Transport)-> Self{
        Self { transport }
    }
    pub async fn get_service_info(&self) -> Result<models::Service, Box<dyn std::error::Error>> {
        
        let configuration = &self.transport.config;

        let url = format!("{}/service-info", configuration.base_path);
        let response = self.transport.get(&url,None).await;
        match response {
            Ok(response_body) => {
                match serde_json::from_str::<models::Service>(&response_body) {
                    Ok(tes_create_task_response) => Ok(tes_create_task_response),
                    Err(e) => {
                        log::error!("Failed to deserialize response: {}", e);
                        Err("Failed to deserialize response".into())
                    },
                }
            },
            Err(e) => {
                log::error!("Error: {}", e);
                Err(e)
            },
        }
    }

}



#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr::eq;
    use mockall::predicate::*;
    use tokio;

    #[tokio::test]
    async fn test_get_service_info() {
        // let mut mock_transport = MockTransport::new();

        // // Set up the mock to return a specific value when `get` is called
        // mock_transport.expect_get()
        //     .with(eq("http://localhost/service-info"), eq(None))
        //     .returning(|_, _| Ok(String::from("{\"id\": \"test\", \"name\": \"test\"}")));

        // let service_info = ServiceInfo::new(mock_transport);
        // let result = service_info.get_service_info().await;

        // assert!(result.is_ok());
        // assert_eq!(result.unwrap().id, "test");
        // assert_eq!(result.unwrap().name, "test");
    }
     #[tokio::test]
    async fn test_get_service_info_from_funnel() {
        // Initialize the Transport struct to point to your local Funnel server
        let config = Configuration::new("http://localhost:8000".to_string(), None, None);
        let transport = Transport::new(&config);

        // Create a ServiceInfo instance using the local Transport
        let service_info = ServiceInfo::new(transport);

        // Call get_service_info and print the result
        match service_info.get_service_info().await {
            Ok(service) => {
                println!("Service Info: {:?}", service);
            },
            Err(e) => {
                println!("Failed to get service info: {}", e);
            },
        }
    }
}

// CHECK WHAT ALL ARE REQUIRED

// use std::error;
// use std::fmt;

// #[derive(Debug, Clone)]
// pub struct ResponseContent<T> {
//     pub status: reqwest::StatusCode,
//     pub content: String,
//     pub entity: Option<T>,
// }

// #[derive(Debug)]
// pub enum Error<T> {
//     Reqwest(reqwest::Error),
//     Serde(serde_json::Error),
//     Io(std::io::Error),
//     ResponseError(ResponseContent<T>),
// }

// impl <T> fmt::Display for Error<T> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let (module, e) = match self {
//             Error::Reqwest(e) => ("reqwest", e.to_string()),
//             Error::Serde(e) => ("serde", e.to_string()),
//             Error::Io(e) => ("IO", e.to_string()),
//             Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
//         };
//         write!(f, "error in {}: {}", module, e)
//     }
// }

// impl <T: fmt::Debug> error::Error for Error<T> {
//     fn source(&self) -> Option<&(dyn error::Error + 'static)> {
//         Some(match self {
//             Error::Reqwest(e) => e,
//             Error::Serde(e) => e,
//             Error::Io(e) => e,
//             Error::ResponseError(_) => return None,
//         })
//     }
// }

// impl <T> From<reqwest::Error> for Error<T> {
//     fn from(e: reqwest::Error) -> Self {
//         Error::Reqwest(e)
//     }
// }

// impl <T> From<serde_json::Error> for Error<T> {
//     fn from(e: serde_json::Error) -> Self {
//         Error::Serde(e)
//     }
// }

// impl <T> From<std::io::Error> for Error<T> {
//     fn from(e: std::io::Error) -> Self {
//         Error::Io(e)
//     }
// }

// pub fn urlencode<T: AsRef<str>>(s: T) -> String {
//     ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
// }

// pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
//     if let serde_json::Value::Object(object) = value {
//         let mut params = vec![];

//         for (key, value) in object {
//             match value {
//                 serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
//                     &format!("{}[{}]", prefix, key),
//                     value,
//                 )),
//                 serde_json::Value::Array(array) => {
//                     for (i, value) in array.iter().enumerate() {
//                         params.append(&mut parse_deep_object(
//                             &format!("{}[{}][{}]", prefix, key, i),
//                             value,
//                         ));
//                     }
//                 },
//                 serde_json::Value::String(s) => params.push((format!("{}[{}]", prefix, key), s.clone())),
//                 _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
//             }
//         }

//         return params;
//     }

//     unimplemented!("Only objects are supported with style=deepObject")
// }

