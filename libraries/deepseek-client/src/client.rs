use std::sync::Arc;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use reqwest::{Client, ClientBuilder};
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use anyhow::{Result, Context};
use tokio::sync::Mutex;
use crate::models::{ApiRequest, ApiResponse, ChatMessage};

#[derive(Clone, Debug)]
pub struct DeepSeekConfig {
    pub api_key: String,
    pub model: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub timeout_seconds: u64,
    pub max_retries: u32,
}

impl Default for DeepSeekConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            model: "deepseek-chat".to_string(),
            max_tokens: 1024,
            temperature: 0.3,
            timeout_seconds: 30,
            max_retries: 3,
        }
    }
}

pub struct DeepSeekClient {
    config: DeepSeekConfig,
    http_client: Client,
    rate_limiter: Arc<Mutex<RateLimiter>>,
}

struct RateLimiter {
    requests_per_minute: usize,
    current_count: usize,
    window_start: std::time::Instant,
}

impl DeepSeekClient {
    pub fn new(config: DeepSeekConfig) -> Result<Self> {
        let headers = Self::build_headers(&config.api_key)?;
        
        let http_client = ClientBuilder::new()
            .default_headers(headers)
            .timeout(Duration::from_secs(config.timeout_seconds))
            .connect_timeout(Duration::from_secs(10))
            .pool_max_idle_per_host(10)
            .pool_idle_timeout(Duration::from_secs(30))
            .build()
            .context("Failed to build HTTP client")?;

        Ok(Self {
            config,
            http_client,
            rate_limiter: Arc::new(Mutex::new(RateLimiter {
                requests_per_minute: 60,
                current_count: 0,
                window_start: std::time::Instant::now(),
            })),
        })
    }

    fn build_headers(api_key: &str) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        
        let auth_value = format!("Bearer {}", api_key);
        headers.insert(AUTHORIZATION, auth_value.parse()?);
        headers.insert(CONTENT_TYPE, "application/json".parse()?);
        
        Ok(headers)
    }

    pub async fn chat_completion(
        &self,
        messages: Vec<ChatMessage>,
    ) -> Result<ApiResponse> {
        self.acquire_rate_limit().await;

        let request_body = ApiRequest {
            model: self.config.model.clone(),
            messages,
            max_tokens: self.config.max_tokens,
            temperature: self.config.temperature,
            stream: false,
        };

        let mut retries = 0;
        let mut last_error = None;

        while retries <= self.config.max_retries {
            match self.send_request(&request_body).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    last_error = Some(e);
                    retries += 1;
                    
                    if retries <= self.config.max_retries {
                        tokio::time::sleep(Duration::from_millis(
                            500 * (2_u64.pow(retries - 1))
                        )).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| 
            anyhow::anyhow!("Max retries exceeded")
        ))
    }

    async fn send_request(
        &self,
        request: &ApiRequest
    ) -> Result<ApiResponse> {
        let response = self.http_client
            .post("https://api.deepseek.com/v1/chat/completions")
            .json(request)
            .send()
            .await
            .context("Failed to send request")?;

        if !response.status().is_success() {
            let error_body = response.text().await?;
            return Err(anyhow::anyhow!(
                "API error ({}): {}", 
                response.status(), 
                error_body
            ));
        }

        let response_body: ApiResponse = response
            .json()
            .await
            .context("Failed to parse response")?;

        Ok(response_body)
    }

    async fn acquire_rate_limit(&self) {
        let mut limiter = self.rate_limiter.lock().await;
        
        let elapsed = limiter.window_start.elapsed();
        if elapsed > Duration::from_secs(60) {
            limiter.current_count = 0;
            limiter.window_start = std::time::Instant::now();
        }

        if limiter.current_count >= limiter.requests_per_minute {
            let wait_time = Duration::from_secs(60) - elapsed;
            tokio::time::sleep(wait_time).await;
            limiter.current_count = 0;
            limiter.window_start = std::time::Instant::now();
        }

        limiter.current_count += 1;
    }
}
