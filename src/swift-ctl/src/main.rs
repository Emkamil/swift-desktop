// Copyright (C) 2026  Kamil Machowski
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use zbus::{Connection, proxy};
use clap::{Parser, Subcommand};
use futures_util::StreamExt;

// Updated proxy to match the new multi-section daemon interface
#[proxy(
    interface = "org.swift.Config",
    default_service = "org.swift.Config",
    default_path = "/org/swift/Config"
)]
trait SwiftConfig {
    // Now functions take 'section' and 'key'
    async fn get_value(&self, section: &str, key: &str) -> zbus::Result<String>;
    async fn set_value(&self, section: &str, key: &str, value: &str) -> zbus::Result<()>;
    
    #[zbus(signal)]
    fn notify(&self, section: &str, key: &str, value: &str) -> zbus::Result<()>;
}

#[derive(Parser)]
#[command(name = "swift-ctl", about = "Swift session main controller", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get value: swift-ctl get <section> <key>
    Get { 
        section: String, 
        key: String 
    },
    /// Set value: swift-ctl set <section> <key> <value>
    Set { 
        section: String, 
        key: String, 
        value: String 
    },
    /// Monitor property changes in real-time
    Monitor,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let conn = Connection::session().await?;
    let proxy = SwiftConfigProxy::new(&conn).await?;

    match &cli.command {
        Commands::Get { section, key } => {
            let val = proxy.get_value(section, key).await?;
            if val.is_empty() {
                println!("Value for '{}/{}' is not set or empty.", section, key);
            } else {
                println!("{}", val);
            }
        }
        Commands::Set { section, key, value } => {
            proxy.set_value(section, key, value).await?;
            println!("Successfully set: [{}] {} = {}", section, key, value);
        }
        Commands::Monitor => {
            println!("--- Swift Monitor: Waiting for signals (Multi-section) ---");
            let mut signals = proxy.receive_notify().await?;
            while let Some(sig) = signals.next().await {
                let args = sig.args()?;
                println!("CHANGE: Section: [{}], Key: '{}' -> New Value: '{}'", 
                    args.section, args.key, args.value);
            }
        }
    }

    Ok(())
}