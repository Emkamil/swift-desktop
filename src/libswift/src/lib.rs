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

pub mod widgets;
pub use widgets::swift_accent_button::SwiftAccentButton;

use gtk4::prelude::*;
use gtk4::CssProvider;
use zbus::dbus_proxy;

// The macro generates 'ConfigProxy' from the 'Config' trait
#[dbus_proxy(
    interface = "org.swift.Config.Theme",
    default_service = "org.swift.Config",
    default_path = "/org/swift/Config"
)]
trait Config {
    #[dbus_proxy(property)]
    fn accent_color(&self) -> zbus::Result<String>;
}

pub fn apply_swift_theme(accent_hex: &str) {
    let provider = CssProvider::new();
    let css = format!(
        "@define-color accent_bg_color {0};
         @define-color accent_fg_color #ffffff;

         .swift-accent-button {{
            background-color: @accent_bg_color;
            color: @accent_fg_color;
            border-radius: 8px;
            padding: 6px 16px;
            border: none;
            font-weight: 600;
            transition: all 0.2s cubic-bezier(0.25, 0.46, 0.45, 0.94);
         }}

         .swift-accent-button:hover {{
            filter: brightness(1.1);
            box-shadow: 0 4px 12px rgba(0,0,0,0.15);
         }}",
        accent_hex
    );

    provider.load_from_data(&css);

    if let Some(display) = gtk4::gdk::Display::default() {
        // Fix: Use the non-deprecated global function and correct priority constant
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

pub async fn init_libswift() -> Result<(), Box<dyn std::error::Error>> {
    let connection = zbus::Connection::session().await?;
    
    // The trait was named 'Config', so zbus generates 'ConfigProxy'
    let proxy = ConfigProxy::new(&connection).await?;

    // D-Bus properties return owned Strings, fixing the Sized/str error
    if let Ok(color) = proxy.accent_color().await {
        apply_swift_theme(&color);
    }

    Ok(())
}