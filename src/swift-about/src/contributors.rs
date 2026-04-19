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


pub struct ContributorInfo {
    pub name: &'static str,
    pub email: &'static str,
}

pub struct ContributorGroup {
    pub name: &'static str,
    pub contributors: &'static [ContributorInfo],
}

pub static SWIFT_CONTRIBUTORS_CORE: &[ContributorInfo] = &[ContributorInfo {
    name: "Kamil Machowski",
    email: "machowskikamil@proton.me",
}];
/*
pub static SWIFT_CONTRIBUTORS_ACTIVE: &[ContributorInfo] = &[];
pub static SWIFT_CONTRIBUTORS_SERVER: &[ContributorInfo] = &[];
pub static SWIFT_CONTRIBUTORS_TRANSLATORS_SUPERVISION: &[ContributorInfo] = &[];
pub static SWIFT_CONTRIBUTORS_DOCUMENTATION_SUPERVISION: &[ContributorInfo] = &[];
pub static SWIFT_CONTRIBUTORS_PREVIOUS: &[ContributorInfo] = &[];
*/

pub static SWIFT_CONTRIBUTORS: &[ContributorGroup] = &[
    ContributorGroup {
        name: "Core developers",
        contributors: SWIFT_CONTRIBUTORS_CORE,
    }, /*ContributorGroup {
           name: "Active contributors",
           contributors: SWIFT_CONTRIBUTORS_ACTIVE,
       },
       ContributorGroup {
           name: "Servers maintained by",
           contributors: SWIFT_CONTRIBUTORS_SERVER,
       },
       ContributorGroup {
           name: "Translations supervision",
           contributors: SWIFT_CONTRIBUTORS_TRANSLATORS_SUPERVISION,
       },
       ContributorGroup {
           name: "Documentation supervision",
           contributors: SWIFT_CONTRIBUTORS_DOCUMENTATION_SUPERVISION,
       },
       ContributorGroup {
           name: "Previous contributors",
           contributors: SWIFT_CONTRIBUTORS_PREVIOUS,
       },*/
];
