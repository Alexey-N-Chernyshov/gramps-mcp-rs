// Copyright 2026 Alexey Chernyshov
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod app_error;
mod auth;
mod client;
mod config;
mod models;
mod server;
mod tools;

use app_error::AppError;
use rmcp::{transport::io::stdio, ServiceExt};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    init_tracing();

    let config = config::Config::from_env()?;

    tracing::info!(
        "Starting gramps-web-mcp-rs, API base: {}",
        config.gramps_api_url
    );

    run(config).await
}

fn init_tracing() {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(std::io::stderr))
        .with(EnvFilter::from_default_env())
        .init();
}

async fn run(config: config::Config) -> Result<(), AppError> {
    let mcp_server = server::GrampsMcpServer::new(config)?;
    let transport = stdio();
    let server = mcp_server
        .serve(transport)
        .await
        .map_err(|e| AppError::ServerInit(Box::new(e)))?;

    tracing::info!("gramps-web-mcp-rs started on stdio");

    server.waiting().await?;

    Ok(())
}
