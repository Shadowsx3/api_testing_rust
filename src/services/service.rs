macro_rules! generate_service {
    ($service_name:ident, $endpoint:expr, $additional_data:ty) => {
        pub struct $service_name<'a, D> {
            client: &'a ClientWithMiddleware,
            path: String,
            data: D,
        }

        impl<'a, D> $service_name<'a, D> {
            pub fn new(client: &'a ClientWithMiddleware, data: D) -> Self {
                Self {
                    client,
                    path: format!("{}{}", base_url(), $endpoint),
                    data,
                }
            }

            pub fn from_url(client: &'a ClientWithMiddleware, url: &'a str, data: D) -> Self {
                Self {
                    client,
                    path: url.to_string(),
                    data,
                }
            }

            pub fn set_client(&mut self, client: &'a ClientWithMiddleware) {
                self.client = client;
            }

            pub fn get_path(&self) -> &str {
                &self.path
            }

            pub fn get_data(&self) -> &D {
                &self.data
            }
        }
    };
    ($service_name:ident, $endpoint:expr) => {
        pub struct $service_name<'a> {
            client: &'a ClientWithMiddleware,
            path: String,
        }

        impl<'a> $service_name<'a> {
            pub fn new(client: &'a ClientWithMiddleware) -> Self {
                Self {
                    client,
                    path: format!("{}{}", base_url(), $endpoint),
                }
            }

            pub fn from_url(client: &'a ClientWithMiddleware, url: &'a str) -> Self {
                Self {
                    client,
                    path: url.to_string(),
                }
            }

            pub fn set_client(&mut self, client: &'a ClientWithMiddleware) {
                self.client = client;
            }

            pub fn get_path(&self) -> &str {
                &self.path
            }
        }
    };
}

pub(crate) use generate_service;