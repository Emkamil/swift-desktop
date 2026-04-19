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

mod interface;
mod store;

use crate::interface::SwiftConfigInterface;
use crate::store::Store;
use std::sync::Arc;
use zbus::connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = Arc::new(Store::new());

    let interface = SwiftConfigInterface { store };

    let _conn = connection::Builder::session()?
        .name("org.swift.Config")?
        .serve_at("/org/swift/Config", interface)?
        .build()
        .await?;

    println!("Swift Config Daemon: Active.");

    std::future::pending::<()>().await;
    Ok(())
}
