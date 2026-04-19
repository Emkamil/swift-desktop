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

use crate::store::Store;
use std::sync::Arc;
use zbus::{SignalContext, interface};

pub struct SwiftConfigInterface {
    pub store: Arc<Store>,
}

#[interface(name = "org.swift.Config")]
impl SwiftConfigInterface {
    async fn get_property(&self, key: &str) -> String {
        let db = self.store.data.read().unwrap();
        db.settings.get(key).cloned().unwrap_or_default()
    }

    async fn set_property(
        &self,
        #[zbus(signal_context)] ctxt: SignalContext<'_>,
        key: String,
        value: String,
    ) -> zbus::fdo::Result<()> {
        {
            let mut db = self.store.data.write().unwrap();
            db.settings.insert(key.clone(), value.clone());
        }

        self.store
            .save()
            .map_err(|e| zbus::fdo::Error::Failed(e.to_string()))?;

        Self::notify(ctxt, &key, &value)
            .await
            .map_err(|e| zbus::fdo::Error::Failed(e.to_string()))?;

        Ok(())
    }

    #[zbus(signal)]
    pub async fn notify(ctxt: SignalContext<'_>, key: &str, value: &str) -> zbus::Result<()>;
}
