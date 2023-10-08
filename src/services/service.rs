macro_rules! generate_service {
    ($service_name:ident, $endpoint:expr, $additional_data:ty) => {
        pub struct $service_name<'a, D> {
            client: &'a ClientWithMiddleware,
            cookie_jar: Arc<Jar>,
            base_url: &'a str,
            path: String,
            data: D,
        }

        impl<'a, D> $service_name<'a, D> {
            pub fn new(cookie_client: &'a CookieClient, data: D) -> Self {
                Self {
                    client: &cookie_client.client,
                    cookie_jar: cookie_client.cookie_jar.clone(),
                    base_url: &base_url(),
                    path: format!("{}{}", base_url(), $endpoint),
                    data,
                }
            }

            pub fn from_url(cookie_client: &'a CookieClient, base_url: &'a str, data: D) -> Self {
                Self {
                    client: &cookie_client.client,
                    cookie_jar: cookie_client.cookie_jar.clone(),
                    base_url,
                    path: format!("{}{}", base_url, $endpoint),
                    data,
                }
            }

            pub fn set_client(&mut self, cookie_client: &'a CookieClient) {
                self.client = &cookie_client.client;
                self.cookie_jar = cookie_client.cookie_jar.clone();
            }

            pub fn get_base_url(&self) -> Url {
                Url::parse(&self.base_url).unwrap()
            }

            pub fn get_path(&self) -> &str {
                &self.path
            }

            pub fn get_data(&self) -> &D {
                &self.data
            }

            pub fn set_data(&mut self, data: D) {
                self.data = data;
            }
        }
    };
    ($service_name:ident, $endpoint:expr) => {
        pub struct $service_name<'a> {
            client: &'a ClientWithMiddleware,
            cookie_jar: Arc<Jar>,
            base_url: &'a str,
            path: String,
        }

        impl<'a> $service_name<'a> {
            pub fn new(cookie_client: &'a CookieClient) -> Self {
                Self {
                    client: &cookie_client.client,
                    cookie_jar: cookie_client.cookie_jar.clone(),
                    base_url: &base_url(),
                    path: format!("{}{}", base_url(), $endpoint),
                }
            }

            pub fn from_url(cookie_client: &'a CookieClient, base_url: &'a str) -> Self {
                Self {
                    client: &cookie_client.client,
                    cookie_jar: cookie_client.cookie_jar.clone(),
                    base_url,
                    path: format!("{}{}", base_url, $endpoint),
                }
            }

            pub fn set_client(&mut self, cookie_client: &'a CookieClient) {
                self.client = &cookie_client.client;
                self.cookie_jar = cookie_client.cookie_jar.clone();
            }

            pub fn get_base_url(&self) -> Url {
                Url::parse(&self.base_url).unwrap()
            }

            pub fn get_path(&self) -> &str {
                &self.path
            }
        }
    };
}
pub(crate) use generate_service;