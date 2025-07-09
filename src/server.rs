use axum::{Router, routing::get_service};
use std::io;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

/// Serve static files from the given directory on the specified port.
pub async fn serve_web(directory_to_serve: &str, port_number: u16) -> io::Result<()> {
    // Check if the directory exists
    if !std::path::Path::new(directory_to_serve).exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Directory '{}' does not exist", directory_to_serve),
        ));
    }

    // Create a service to serve static files
    let static_file_service = get_service(ServeDir::new(directory_to_serve));

    // Set up the Axum router to serve files at the root path
    let app_router = Router::new().nest_service("/", static_file_service);

    // Build the socket address for localhost and the given port
    let server_address = SocketAddr::from(([127, 0, 0, 1], port_number));
    println!(
        "Server started. The '{}' folder is being served at http://{}",
        directory_to_serve, server_address
    );

    // Bind the TCP listener to the address
    let tcp_listener = tokio::net::TcpListener::bind(server_address).await.map_err(|error| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to bind to address: {}", error),
        )
    })?;

    // Start the Axum server
    axum::serve(tcp_listener, app_router)
        .await
        .map_err(|error| io::Error::new(io::ErrorKind::Other, format!("Server error: {}", error)))?;

    Ok(())
}