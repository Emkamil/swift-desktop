Name:           swift-desktop
Version:        1.0.4
Release:        1%{?dist}
Summary:        SWIFT Desktop Environment Components
License:        GPL-3.0-or-later
URL:            https://github.com/Emkamil/swift-desktop

# Using rpkg to pack the entire workspace repository
Source:         {{{ git_dir_pack }}}

BuildRequires:  cargo
BuildRequires:  rust
BuildRequires:  pkgconfig(gtk4)
BuildRequires:  pkgconfig(libadwaita-1)
BuildRequires:  gettext

%description
Primary components and libraries for the SWIFT Desktop Environment.

# --- SUBPACKAGE: swift-about ---
%package -n swift-about
Summary:        Swift Desktop About Dialog
%description -n swift-about
About dialog for the Swift Desktop environment.

# --- SUBPACKAGE: swift-cfg (Background Service) ---
%package -n swift-cfg
Summary:        SWIFT Desktop Configuration Service
%description -n swift-cfg
Background service handling system configuration via DBus interface. 
Includes default settings management.

# --- SUBPACKAGE: swift-ctl (CLI Tool) ---
%package -n swift-ctl
Summary:        SWIFT Desktop Control Tool
%description -n swift-ctl
Command-line interface for manual configuration and management of 
Swift desktop settings.

%prep
# Standard setup for rpkg-based workspace
%setup -q -n %{name}

%build
# Build all workspace members at once
cargo build --release

%install
# Create directory structure
mkdir -p %{buildroot}%{_bindir}
mkdir -p %{buildroot}%{_datadir}/swift/licenses
mkdir -p %{buildroot}%{_datadir}/locale/pl/LC_MESSAGES

# --- Install: swift-about ---
install -m 0755 target/release/swift-about %{buildroot}%{_bindir}/swift-about
install -D -m 0644 src/swift-about/res/swift-about.svg %{buildroot}%{_datadir}/icons/hicolor/scalable/apps/swift-about.svg
install -D -m 0644 src/swift-about/res/swift-about-symbolic.svg %{buildroot}%{_datadir}/icons/hicolor/symbolic/apps/swift-about-symbolic.svg
install -D -m 0644 src/swift-about/res/swift-about.desktop %{buildroot}%{_datadir}/applications/swift-about.desktop
msgfmt src/swift-about/po/pl.po -o %{buildroot}%{_datadir}/locale/pl/LC_MESSAGES/swift-about.mo

# --- Install: swift-cfg (Background service & defaults) ---
install -m 0755 target/release/swift-cfg %{buildroot}%{_bindir}/swift-cfg
# defaults.toml is intentionally NOT marked as %config to ensure it updates every time
install -D -m 0644 src/swift-cfg/res/defaults.toml %{buildroot}%{_datadir}/swift/defaults.toml

# --- Install: swift-ctl (CLI tool) ---
install -m 0755 target/release/swift-ctl %{buildroot}%{_bindir}/swift-ctl

# --- Shared resources ---
install -p -m 0644 src/swift-about/res/licenses/*.txt %{buildroot}%{_datadir}/swift/licenses/

%find_lang swift-about

# --- Files: swift-about ---
%files -n swift-about -f swift-about.lang
%{_bindir}/swift-about
%{_datadir}/icons/hicolor/scalable/apps/swift-about.svg
%{_datadir}/icons/hicolor/symbolic/apps/swift-about-symbolic.svg
%{_datadir}/applications/swift-about.desktop
%{_datadir}/swift/licenses/*.txt

# --- Files: swift-cfg ---
%files -n swift-cfg
%{_bindir}/swift-cfg
%{_datadir}/swift/defaults.toml

# --- Files: swift-ctl ---
%files -n swift-ctl
%{_bindir}/swift-ctl

%changelog
* Sun Apr 19 2026 Kamil <emkamil@example.com> - 1.0.4-1
- Migrated to Cargo Workspace unified build system
- Integrated swift-about, swift-cfg, and swift-ctl into a single SPEC
- Ensured defaults.toml is overwritten on every update for swift-cfg