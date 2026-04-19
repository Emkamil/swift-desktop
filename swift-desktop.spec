Name:           swift-desktop
Version:        1.0.4
Release:        3%{?dist}
Summary:        SWIFT Desktop Environment Components
License:        GPL-3.0-or-later
URL:            https://github.com/Emkamil/swift-desktop

Source:         {{{ git_dir_pack }}}

BuildRequires:  cargo
BuildRequires:  rust
BuildRequires:  pkgconfig(gtk4)
BuildRequires:  pkgconfig(libadwaita-1)
BuildRequires:  gettext
# Required for systemd macros
BuildRequires:  systemd-rpm-macros

%description
Primary components and libraries for the SWIFT Desktop Environment.

%package -n swift-about
Summary:        SWIFT Desktop About Dialog
%description -n swift-about
A modern about dialog for the SWIFT desktop environment.

%package -n swift-cfg
Summary:        SWIFT Desktop Configuration Service
%{?systemd_requires}

%description -n swift-cfg
Background service handling system configuration via DBus interface.
Includes default settings management and systemd service integration.

%package -n swift-ctl
Summary:        SWIFT Desktop Control Tool
%description -n swift-ctl
Command-line interface for managing SWIFT desktop settings.

%prep
%setup -q -n %{name}

%build
cargo build --release

%install
mkdir -p %{buildroot}%{_bindir}
mkdir -p %{buildroot}%{_datadir}/swift/licenses
mkdir -p %{buildroot}%{_unitdir}

# --- Install: swift-about ---
install -m 0755 target/release/swift-about %{buildroot}%{_bindir}/swift-about
install -D -m 0644 src/swift-about/res/swift-about.svg %{buildroot}%{_datadir}/icons/hicolor/scalable/apps/swift-about.svg
install -D -m 0644 src/swift-about/res/swift-about-symbolic.svg %{buildroot}%{_datadir}/icons/hicolor/symbolic/apps/swift-about-symbolic.svg
install -D -m 0644 src/swift-about/res/swift-about.desktop %{buildroot}%{_datadir}/applications/swift-about.desktop
mkdir -p %{buildroot}%{_datadir}/locale/pl/LC_MESSAGES
msgfmt src/swift-about/po/pl.po -o %{buildroot}%{_datadir}/locale/pl/LC_MESSAGES/swift-about.mo

# --- Install: swift-cfg (Service & Defaults) ---
install -m 0755 target/release/swift-cfg %{buildroot}%{_bindir}/swift-cfg
install -D -m 0644 src/swift-cfg/res/defaults.toml %{buildroot}%{_datadir}/swift/defaults.toml
# Installing the systemd service unit
install -p -m 0644 src/swift-cfg/res/swift-cfg.service %{buildroot}%{_unitdir}/swift-cfg.service

# --- Install: swift-ctl ---
install -m 0755 target/release/swift-ctl %{buildroot}%{_bindir}/swift-ctl

# --- Shared resources ---
install -p -m 0644 src/swift-about/res/licenses/*.txt %{buildroot}%{_datadir}/swift/licenses/

%find_lang swift-about

# --- Systemd post-install scripts for swift-cfg ---
%post -n swift-cfg
%systemd_post swift-cfg.service

%preun -n swift-cfg
%systemd_preun swift-cfg.service

%postun -n swift-cfg
%systemd_postun_with_restart swift-cfg.service

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
%{_unitdir}/swift-cfg.service

# --- Files: swift-ctl ---
%files -n swift-ctl
%{_bindir}/swift-ctl