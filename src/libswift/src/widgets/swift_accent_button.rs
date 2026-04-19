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

use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use gtk4::glib;

glib::wrapper! {
    pub struct SwiftAccentButton(ObjectSubclass<imp::SwiftAccentButton>)
        @extends gtk4::Button, gtk4::Widget,
        @implements gtk4::Accessible, gtk4::Actionable, gtk4::Buildable, gtk4::ConstraintTarget;
}

impl SwiftAccentButton {
    pub fn new(label: &str) -> Self {
        glib::Object::builder()
            .property("label", label)
            .build()
    }
}

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct SwiftAccentButton;

    #[glib::object_subclass]
    impl ObjectSubclass for SwiftAccentButton {
        const NAME: &'static str = "SwiftAccentButton";
        type Type = super::SwiftAccentButton;
        type ParentType = gtk4::Button;
    }

    impl ObjectImpl for SwiftAccentButton {
        fn constructed(&self) {
            self.parent_constructed();
            // Class used for CSS targeting
            self.obj().add_css_class("swift-accent-button");
        }
    }

    impl WidgetImpl for SwiftAccentButton {}
    impl ButtonImpl for SwiftAccentButton {}
}