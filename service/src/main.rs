use futures::StreamExt;
use r2r::std_srvs::srv::SetBool;
use r2r::{Context, Node, QosProfile};
use std::sync::{Arc, Mutex};
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Initialize ROS2 context
    let ctx = Context::create()?;
    let mut node = Node::create(ctx, "service_server_rs", "")?;

    log::info!("ROS2 node 'service_server_rs' created successfully!");

    // Shared state for the service
    let bool_state = Arc::new(Mutex::new(false));

    // Create service server
    let service = node.create_service::<SetBool::Service>("demo/set_bool", QosProfile::default())?;
    let bool_state_clone = bool_state.clone();

    // Spawn service handling task
    tokio::spawn(async move {
        service
            .for_each(|req| {
                log::info!("Received SetBool request: data = {}", req.message.data);

                // Update shared state
                *bool_state_clone.lock().unwrap() = req.message.data;

                // Create response
                let response = SetBool::Response {
                    success: true,
                    message: format!("Successfully set bool to: {}", req.message.data),
                };

                log::info!(
                    "Sending response: success = {}, message = '{}'",
                    response.success,
                    response.message
                );

                // Send response back
                req.respond(response).expect("Failed to send response");

                futures::future::ready(())
            })
            .await;
    });

    log::info!("Service '/demo/set_bool' advertised successfully!");

    // Spawn state monitoring task
    let bool_state_monitor = bool_state.clone();
    tokio::spawn(async move {
        loop {
            let current_value = *bool_state_monitor.lock().unwrap();
            log::info!("Current bool state: {}", current_value);
            sleep(Duration::from_secs(2)).await;
        }
    });

    // Handle Ctrl+C gracefully
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        log::info!("Received Ctrl+C, shutting down...");
        std::process::exit(0);
    });

    log::info!("Service is running at '/demo/set_bool'. Press Ctrl+C to stop.");

    // Main event loop
    loop {
        node.spin_once(Duration::from_millis(100));
        sleep(Duration::from_millis(10)).await;
    }
}
