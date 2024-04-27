use rusoto_ce::{CostExplorer, CostExplorerClient, GetAnomalyMonitorsRequest};
use rusoto_core::credential::{ChainProvider, ProvideAwsCredentials};
use rusoto_core::{HttpClient, Region, RusotoError};
// use rusoto_credential::StaticProvider;
use serde::Deserialize;
use serde_yaml;
use std::fs;
use tokio;

mod monitor;

#[allow(dead_code)]
#[derive(Debug)]
struct AnomalyMonitor {
    name: String,
    threshold: Option<f64>,
}

impl From<rusoto_ce::AnomalyMonitor> for AnomalyMonitor {
    fn from(monitor: rusoto_ce::AnomalyMonitor) -> Self {
        Self {
            name: monitor.monitor_name,
            threshold: None,
        }
    }
}

async fn check_anomaly_monitors(
    client: &CostExplorerClient,
) -> Result<Vec<AnomalyMonitor>, RusotoError<rusoto_ce::GetAnomalyMonitorsError>> {
    let request = GetAnomalyMonitorsRequest::default();
    let response = client.get_anomaly_monitors(request).await?;
    Ok(response
        .anomaly_monitors
        .into_iter()
        .map(AnomalyMonitor::from)
        .collect())
}

#[derive(Debug, Deserialize)]
struct Config {
    monitor_name: String,
    monitor_type: String,
    // region: String,
    // anomaly_threshold: f64,
}

fn load_config_from_file(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let config: Config = serde_yaml::from_str(&contents)?;
    Ok(config)
}

#[tokio::main]
async fn main() {
    let config_path = "config/config.yaml";
    let config = match load_config_from_file(config_path) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            return;
        }
    };

    let provider = ChainProvider::new();
    match provider.credentials().await {
        Ok(_) => println!("Successfully authenticated with AWS."),
        Err(e) => {
            eprintln!("Failed to authenticate with AWS: {}", e);
            return;
        }
    }

    let client = CostExplorerClient::new_with(
        HttpClient::new().expect("Failed to create HTTP client"),
        provider,
        Region::default(),
    );

    if let Err(e) = monitor::create_anomaly_monitor_from_config(&client, &config).await {
        eprintln!("Error creating anomaly monitor: {:?}", e);
    }

    match check_anomaly_monitors(&client).await {
        Ok(anomaly_monitors) => {
            for monitor in anomaly_monitors {
                println!("Anomaly Monitor: {:?}", monitor);
            }
        }
        Err(err) => eprintln!("Error checking anomaly monitors: {:?}", err),
    }
}

// fn setup_fake_client() -> CostExplorerClient {
//     let credentials = StaticProvider::new_minimal("fakeAccessKeyId".to_string(), "fakeSecretAccessKey".to_string());
//     CostExplorerClient::new_with(HttpClient::new().expect("Failed to create HTTP client"), credentials, Region::UsEast1)
// }

// #[tokio::test]
// async fn test_anomaly_monitor_creation() {
//     let client = setup_fake_client();
//     let config = Config {
//         monitor_name: "Test Monitor".to_string(),
//         monitor_type: "COST".to_string(),
//     };
//     assert!(create_anomaly_monitor_from_config(&client, &config).await.is_ok());
// }
