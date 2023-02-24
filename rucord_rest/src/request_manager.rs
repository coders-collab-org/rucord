use std::collections::HashMap;

use reqwest::{header::AUTHORIZATION, Client, Method, Response};
use rucord_api_types::{routes, GatewayBotObject, GatewayObject};
use serde::Serialize;

#[derive(Serialize)]
pub struct Dummy;
pub struct RequestManagerOptions {
    pub global_rate_limit: i32,
}

pub struct RequestOptions<T: Serialize = Dummy> {
    url: String,

    method: Method,

    body: Option<T>,

    extra_headers: Option<HashMap<String, String>>,
}

impl<T: Serialize> RequestOptions<T> {
    #[inline]
    pub fn get(url: String, extra_headers: Option<HashMap<String, String>>) -> Self {
        Self {
            url,
            method: Method::GET,
            body: None,
            extra_headers,
        }
    }

    #[inline]
    pub fn post(
        url: String,
        body: Option<T>,
        extra_headers: Option<HashMap<String, String>>,
    ) -> Self {
        Self {
            url,
            method: Method::POST,
            body,
            extra_headers,
        }
    }
}

#[derive(Default)]
pub struct RequestManager {
    pub options: RequestManagerOptions,
    pub token: Option<String>,

    // TODO: Use handler for every route id.
    client: Client,
}

impl RequestManager {
    pub fn new(options: RequestManagerOptions) -> Self {
        Self {
            options,
            ..Default::default()
        }
    }

    pub fn new_with_token(options: RequestManagerOptions, token: String) -> Self {
        Self {
            options,
            token: Some(token),
            ..Default::default()
        }
    }

    #[inline]
    fn api(route: String) -> String {
        format!("https://discord.com/api/v{v}{route}", v = 10)
    }
}

impl RequestManager {
    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }

    pub async fn request<T: Serialize>(
        &self,
        options: RequestOptions<T>,
    ) -> Result<Response, reqwest::Error> {
        let RequestOptions {
            url,
            method,
            body,
            extra_headers,
        } = options;

        let mut builder = self.client.request(method, url);

        if let Some(ref token) = self.token {
            builder = builder.header(AUTHORIZATION, format!("Bot {}", token));
        }

        if let Some(extra_headers) = extra_headers {
            for (k, v) in extra_headers {
                builder = builder.header(k, v);
            }
        }

        if let Some(ref body) = body {
            builder = builder.json(body);
        }

        self.client.execute(builder.build()?).await
    }
}

impl RequestManager {
    pub async fn get_gateway(&self) -> Result<GatewayObject, reqwest::Error> {
        let options = RequestOptions::<Dummy>::get(Self::api(routes::gateway()), None);
        self.request(options).await?.json().await
    }

    pub async fn get_gateway_bot(&self) -> Result<GatewayBotObject, reqwest::Error> {
        let options = RequestOptions::<Dummy>::get(Self::api(routes::gateway_bot()), None);
        self.request(options).await?.json().await
    }
}

impl Default for RequestManagerOptions {
    fn default() -> Self {
        Self {
            global_rate_limit: 50,
        }
    }
}
