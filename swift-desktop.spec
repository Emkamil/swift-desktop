Name:           swift-desktop
Version:        1.0.4
Release:        4%{?dist}
Summary:        SWIFT Desktop Environment Components
License:        GPL-3.0-or-later
URL:            https://github.com/Emkamil/swift-desktop

Source:         {{{ git_dir_pack }}}

BuildRequires:  cargo
BuildRequires:  rust
BuildRequires:  pkgconfig(gtk4)
BuildRequires:  gettext
BuildRequires:  systemd-rpm-macros

%description
Primary components and libraries for the Swift Desktop Environment.

# --- Package: swift-about ---
%package -n swift-about
Summary:        Swift Desktop About Dialog
%description -n swift-about
A modern about dialog for the Swift desktop environment.

# --- Package: swift-cfg ---
%package -n swift-cfg
Summary:        Swift Desktop Configuration Service
%{?systemd_requires}

%description -n swift-cfg
Background service handling system configuration via DBus interface.
Includes default settings management and user-session systemd integration.

# --- Package: swift-ctl ---
%package -n swift-ctl
Summary:        Swift Desktop Control Tool
%description -n swift-ctl
Command-line interface for managing Swift desktop settings.

%prep
%setup -q -n %{name}

%build
cargo build --release

%install
# Create directory structure
mkdir -p %{buildroot}%{_bindir}
mkdir -p %{buildroot}%{_datadir}/swift/licenses
mkdir -p %{buildroot}%{_datadir}/swift/defaults
mkdir -p %{buildroot}%{_userunitdir}
mkdir -p %{buildroot}%{_userpresetdir}

# --- Install: swift-about ---
install -m 0755 target/release/swift-about %{buildroot}%{_bindir}/swift-about
install -D -m 0644 src/swift-about/res/swift-about.svg %{buildroot}%{_datadir}/icons/hicolor/scalable/apps/swift-about.svg
install -D -m 0644 src/swift-about/res/swift-about-symbolic.svg %{buildroot}%{_datadir}/icons/hicolor/symbolic/apps/swift-about-symbolic.svg
install -D -m 0644 src/swift-about/res/swift-about.desktop %{buildroot}%{_datadir}/applications/swift-about.desktop

# Translations for swift-about
mkdir -p %{buildroot}%{_datadir}/locale/pl/LC_MESSAGES
msgfmt src/swift-about/po/pl.po -o %{buildroot}%{_datadir}/locale/pl/LC_MESSAGES/swift-about.mo

# --- Install: swift-cfg ---
install -m 0755 target/release/swift-cfg %{buildroot}%{_bindir}/swift-cfg
install -m 0644 src/swift-cfg/res/defaults.toml %{buildroot}%{_datadir}/swift/defaults.toml

# Install systemd USER unit
install -p -m 0644 src/swift-cfg/res/swift-cfg.service %{buildroot}%{_userunitdir}/swift-cfg.service

# Install preset to enable the service by default for all users
echo "enable swift-cfg.service" > %{buildroot}%{_userpresetdir}/90-swift-cfg.preset

# --- Install: swift-ctl ---
install -m 0755 target/release/swift-ctl %{buildroot}%{_bindir}/swift-ctl

# --- Shared resources ---
install -p -m 0644 src/swift-about/res/licenses/*.txt %{buildroot}%{_datadir}/swift/licenses/

%find_lang swift-about

# --- Scripts: swift-cfg (USER UNIT) ---
%post -n swift-cfg
%systemd_user_post swift-cfg.service

%preun -n swift-cfg
%systemd_user_preun swift-cfg.service

%postun -n swift-cfg
%systemd_user_postun_with_restart swift-cfg.service

# --- Files: swift-about ---
%files -n swift-about -f swift-about.lang
%{_bindir}/swift-about
%{_datadir}/icons/hicolor/scalable/apps/swift-about.svg
%{_datadir}/icons/hicolor/symbolic/apps/swift-about-symbolic.svg
%{_datadir}/applications/swift-about.desktop
%dir %{_datadir}/swift
%dir %{_datadir}/swift/licenses
%{_datadir}/swift/licenses/*.txt

# --- Files: swift-cfg ---
%files -n swift-cfg
%{_bindir}/swift-cfg
%dir %{_datadir}/swift/defaults
%{_datadir}/swift/defaults.toml
%{_userunitdir}/swift-cfg.service
%{_userpresetdir}/90-swift-cfg.preset

# --- Files: swift-ctl ---
%files -n swift-ctl
%{_bindir}/swift-ctl

%changelog
* Sun Apr 19 2026 Kamil Machowski <kamil@fedora> - 1.0.4-4
- Refactor swift-cfg to use multi-section TOML support
- Migrate systemd service to user session unit
- Add systemd user presets for automatic activation