use rusoto_ce::{
    AnomalyMonitor, CostExplorer, CostExplorerClient, CreateAnomalyMonitorError,
    CreateAnomalyMonitorRequest, DimensionValues, Expression,
};
use rusoto_core::RusotoError;
// use rusoto_core::{Region, HttpClient}
// use rusoto_credential::StaticProvider;

use crate::Config;
pub async fn create_anomaly_monitor_from_config(
    client: &CostExplorerClient,
    config: &Config,
) -> Result<(), RusotoError<CreateAnomalyMonitorError>> {
    let request = CreateAnomalyMonitorRequest {
        anomaly_monitor: AnomalyMonitor {
            monitor_name: config.monitor_name.clone(),
            monitor_type: config.monitor_type.clone(),
            monitor_specification: Some(Expression {
                dimensions: Some(DimensionValues {
                    key: Some("LINKED_ACCOUNT".to_string()),
                    values: Some(vec!["aws_account".to_string()]),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        },
    };

    match client.create_anomaly_monitor(request).await {
        Ok(response) => {
            println!("Monitor Created: {:?}", response);
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to create anomaly monitor: {:?}", e);
            Err(e)
        }
    }
}

// #[tokio::test]
// async fn test_anomaly_monitor_creation() {
//     let client = setup_fake_client();
//     let config = Config {
//         monitor_name: "Test Monitor".to_string(),
//         monitor_type: "COST".to_string(),
//     };
//     assert!(create_anomaly_monitor_from_config(&client, &config).await.is_ok());
// }
// fn setup_fake_client() -> CostExplorerClient {
//     let credentials = StaticProvider::new_minimal("fakeAccessKeyId".to_string(), "fakeSecretAccessKey".to_string());
//     CostExplorerClient::new_with(HttpClient::new().expect("Failed to create HTTP client"), credentials, Region::UsEast1)
// }
