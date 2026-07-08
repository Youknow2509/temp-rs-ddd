use anyhow::{Context, Result};
use domain::config::QdrantSettingRepository;
use qdrant_client::{
    Qdrant,
    config::{CompressionEncoding, QdrantConfig},
    qdrant::{CreateCollectionBuilder, Distance, VectorParamsBuilder},
};
use std::time::Duration;

pub type QdrantClient = Qdrant;

/// Create qdrant client connection
pub fn create_qdrant_client(setting: &QdrantSettingRepository) -> Result<QdrantClient> {
    let url = if setting.tls.is_enabled {
        return Err(anyhow::anyhow!(
            "TLS is not supported for Qdrant connection"
        ));
    } else {
        format!("http://{}:{}", setting.host, setting.port)
    };
    let qdrant_compression = match setting.compression.as_deref() {
        Some("gzip") => Some(CompressionEncoding::Gzip),
        Some("none") | None => None,
        Some(other) => {
            return Err(anyhow::anyhow!(
                "unknown qdrant compression '{}'; expected none|gzip",
                other
            ));
        }
    };
    let config = QdrantConfig {
        uri: url,
        api_key: Some(setting.api_key.clone()),
        timeout: Duration::from_millis(setting.timeout_ms),
        connect_timeout: Duration::from_millis(setting.connect_timeout_ms),
        keep_alive_while_idle: setting.keep_alive_interval,
        custom_headers: setting.custom_headers.clone(),
        pool_size: setting.pool_size,
        check_compatibility: setting.check_compatibility,
        compression: qdrant_compression,
    };
    let client = Qdrant::new(config)
        .inspect_err(|e| tracing::error!(error = %e, "Qdrant create client error"))
        .context("Failed to create Qdrant client")?;

    // Check connection with ensure collection existence
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            if let Err(e) = ensure_qdrant_collection(&client, setting).await {
                return Err(anyhow::anyhow!("Failed to ensure Qdrant collection: {}", e));
            }
            Ok(())
        })
    })?;

    Ok(client)
}

/// Check collection existence and create if not exists
pub async fn ensure_qdrant_collection(
    client: &Qdrant,
    setting: &QdrantSettingRepository,
) -> Result<()> {
    let exists = client
        .collection_exists(&setting.collection)
        .await
        .inspect_err(|e| {
            tracing::error!(error = %e, "Qdrant check collection existence error");
        })
        .context("Error when check collection existence")?;
    if !exists {
        let req = CreateCollectionBuilder::new(&setting.collection).vectors_config(
            VectorParamsBuilder::new(setting.dimension, Distance::Cosine),
        );
        client
            .create_collection(req)
            .await
            .inspect_err(|e| {
                tracing::error!(error = %e, "Qdrant create collection error");
            })
            .context("Error when create collection")?;
    }
    Ok(())
}
