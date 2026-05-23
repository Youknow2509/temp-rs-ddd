use std::time::Duration;

use anyhow::{bail, Context, Result};
use aws_credential_types::Credentials;
use aws_sdk_s3::config::retry::RetryConfig;
use aws_sdk_s3::config::timeout::TimeoutConfig;
use aws_sdk_s3::config::{Builder as S3ConfigBuilder, Region};
use aws_sdk_s3::Client;

use domain::config::ObjectStorageSetting;

pub type S3Client = Client;

pub fn create_client(setting: &ObjectStorageSetting) -> Result<S3Client> {
    if setting.r#type != "s3" {
        bail!(
            "unsupported object storage type '{}'; only 's3' is implemented",
            setting.r#type
        );
    }

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("building S3 runtime")?;

    rt.block_on(build_client(setting))
}

async fn build_client(setting: &ObjectStorageSetting) -> Result<S3Client> {
    let session_token = if setting.session_token.is_empty() {
        None
    } else {
        Some(setting.session_token.clone())
    };

    let credentials = Credentials::new(
        &setting.access_key_id,
        &setting.secret_access_key,
        session_token,
        None,
        "static",
    );

    let retry_config = RetryConfig::standard()
        .with_max_attempts(setting.retry.max_retries + 1);

    let timeout_config = TimeoutConfig::builder()
        .connect_timeout(Duration::from_millis(setting.timeouts.connect_timeout_ms))
        .read_timeout(Duration::from_millis(setting.timeouts.request_timeout_ms))
        .operation_attempt_timeout(Duration::from_millis(
            setting.timeouts.operation_timeout_ms,
        ))
        .build();

    let mut config_builder = S3ConfigBuilder::new()
        .credentials_provider(credentials)
        .region(Region::new(setting.region.clone()))
        .retry_config(retry_config)
        .timeout_config(timeout_config)
        .force_path_style(setting.force_path_style);

    if !setting.endpoint.is_empty() {
        config_builder = config_builder.endpoint_url(&setting.endpoint);
    }

    let client = Client::from_conf(config_builder.build());

    client
        .head_bucket()
        .bucket(&setting.bucket_name)
        .send()
        .await
        .context("S3 health-check failed at startup")?;

    Ok(client)
}
