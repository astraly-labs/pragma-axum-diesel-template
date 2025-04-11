use anyhow::Result;
use serde_json::to_string_pretty;
use std::path::PathBuf;
use utoipa::OpenApi;
use utoipauto::utoipauto;

#[utoipauto(paths = "./crates/pragma-api/src/")]
#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "pragma-bin", description = "pragma, auth service")
    )
)]
pub struct ApiDoc;

impl ApiDoc {
    #[allow(dead_code)]
    pub fn generate_openapi_json(output_path: PathBuf) -> Result<()> {
        let openapi = Self::openapi();
        let json = to_string_pretty(&openapi)?;

        let file_path = output_path.join("openapi.json");

        tracing::info!("Saving OpenAPI specs to {}...", file_path.display());

        std::fs::write(&file_path, json)?;
        tracing::info!("OpenAPI specs saved!");
        Ok(())
    }
}
