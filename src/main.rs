fn main() {
}

#[cfg(test)]
mod test {
    use leptos::SerializationError;
    use leptos_reqwest::{send_and_parse, HttpMethod, LeptosReqwestError};
    use reqwest::{header, Error};
    use serde::{Serialize, Deserialize};
    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct CustomErrors {
        pub errors: Vec<CustomError>,
    }
    impl LeptosReqwestError for CustomErrors {
        fn deserialization_error(e: SerializationError) -> Self {
            CustomErrors {
                errors: vec![CustomError {message: e.to_string(), extensions: ErrorExtension { code: String::from("500"), reason: None }}],
            }
        }
    
        fn read_error(e: Error) -> Self {
            CustomErrors {
                errors: vec![CustomError {message: e.to_string(), extensions: ErrorExtension { code: String::from("500"), reason: None }}],
            }
        }
    }
    impl Default for CustomErrors {
        fn default() -> Self {
            CustomErrors {
                errors: vec![CustomError {message: String::from("System Error"), extensions: ErrorExtension { code: String::from("500"), reason: None }}],
            }
        }
    }

    #[derive(Debug, Clone, Deserialize, Default, Serialize)]
    pub struct CustomError {
        pub message: String,
        pub extensions: ErrorExtension,
    }

    #[derive(Debug, Clone, Deserialize, Default, Serialize)]
    pub struct ErrorExtension {
        pub code: String,
        pub reason: Option<String>
    }

    #[derive(Debug, Clone, Serialize, Default)]
    pub struct AuthenticationRequest {
        pub email: String,
        pub password: String,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct AuthenticationResponse {
        pub data: AuthenticationResponsePayload,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct AuthenticationResponsePayload {
        pub expires: u64,
        pub refresh_token: String,
        pub access_token: String,
    }
   
   #[tokio::test]
    async fn test_login() {
        let url = String::from("https://example.com");

        let request = AuthenticationRequest {
            email: String::from("mark@kodecraft.dev"),
            password: String::from("******"),
        };

        let headers = header::HeaderMap::new();

        let send = send_and_parse::<AuthenticationRequest, AuthenticationResponse, CustomErrors>(Some(request), url, headers, HttpMethod::Post).await;
        match send {
            Ok(response) => {
                println!("{:#?}", response);
            },
            Err(e) => {
                println!("{:#?}", e);
            }
        }
    }
}

