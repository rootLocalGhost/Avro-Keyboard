mod engine;

use engine::AvroEngine;
use std::env;
use zbus::connection::Builder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    smol::block_on(async {
        let _conn = match env::var("IBUS_ADDRESS") {
            Ok(addr) if !addr.trim().is_empty() => {
                Builder::address(addr.as_str())?
                    .serve_at("/org/freedesktop/IBus/Engine/AvroRust", AvroEngine::new())?
                    .build()
                    .await?
            }
            _ => {
                Builder::session()?
                    .name("org.freedesktop.IBus.AvroRust")?
                    .serve_at("/org/freedesktop/IBus/Engine/AvroRust", AvroEngine::new())?
                    .build()
                    .await?
            }
        };

        println!("Avro Rust IBus engine started.");

        // Wait forever
        std::future::pending::<()>().await;

        Ok(())
    })
}
