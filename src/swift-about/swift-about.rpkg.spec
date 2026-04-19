Name:           swift-about
Version:        1.0.4
Release:        2%{?dist}
Summary:        SWIFT-desktop About Dialog
License:        GPL-3.0-or-later
URL:            https://github.com/Emkamil/swift-about

Source:         {{{ git_dir_pack }}}

BuildRequires:  cargo
BuildRequires:  rust
BuildRequires:  pkgconfig(gtk4)
BuildRequires:  pkgconfig(libadwaita-1)
BuildRequires:  gettext

%description
Modern about dialog for Swift desktop.

%prep
%setup -q -n %{name}

%build
cargo build --release

%install
# Binary
install -D -m 0755 target/release/swift-about %{buildroot}%{_bindir}/swift-about

# Icons
install -D -m 0644 res/swift-about.svg %{buildroot}%{_datadir}/icons/hicolor/scalable/apps/swift-about.svg
install -D -m 0644 res/swift-about-symbolic.svg %{buildroot}%{_datadir}/icons/hicolor/symbolic/apps/swift-about-symbolic.svg

# Desktop
install -D -m 0644 res/swift-about.desktop %{buildroot}%{_datadir}/applications/swift-about.desktop

# Translates
mkdir -p %{buildroot}%{_datadir}/locale/pl/LC_MESSAGES
msgfmt po/pl.po -o %{buildroot}%{_datadir}/locale/pl/LC_MESSAGES/swift-about.mo

mkdir -p %{buildroot}%{_datadir}/swift/licenses
install -p -m 0644 res/licenses/*.txt %{buildroot}%{_datadir}/swift/licenses/

%find_lang swift-about

%files -f swift-about.lang
%{_bindir}/swift-about
%{_datadir}/icons/hicolor/scalable/apps/swift-about.svg
%{_datadir}/icons/hicolor/symbolic/apps/swift-about-symbolic.svg
%{_datadir}/applications/swift-about.desktop

%{_datadir}/swift/licenses/*.txt

%license res/licenses/*.txt

%changelog
* Sun Apr 05 2026 Kamil - 1.0.4
- added global licenses path in /usr/share/swift/licenses/
- fixed icon and .desktop files installation