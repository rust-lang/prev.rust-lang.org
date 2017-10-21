---
layout: de-DE/default
title: Andere Installationsmethoden &middot; The Rust Programming Language
---

# Andere Installationsmethoden für Rust

- [Welchen Installer sollte ich benutzen?](#which)
- [Andere Wege, `rustup` zu installieren](#more-rustup)
- [Standalone Installer](#standalone)
- [Quellcode](#source)

## Welchen Installer sollte ich benutzen?
<span id="which"></span>

Rust läuft auf vielen Plattformen, und es gibt viele Wege, es zu installieren.
Wenn du Rust möglichst einfach installieren willst, dann folge den Schritten
auf der [Installations-Hauptseite].

Diese Seite beschreibt Installation via [`rustup`], einem Werkzeug, welches
mehrere Rust-Toolchains auf konsistente Art für alle von Rust unterstützten
Plattformen verwaltet. Warum sollte man Rust _nicht_ mit [`rustup`]
installieren wollen?

- Offline-Installation. `rustup` lädt Komponenten je nach Bedarf aus dem
  Internet nach. Wenn du Rust ohne Internetzugriff installieren willst, dann
  ist `rustup` ungeeignet.
- Vorzug des Paketmanagers. Insbesondere auf Linux, aber mit [Homebrew] auch
  auf macOS, und auf Windows mit [Chocolatey], wollen manche Entwickler Rust
  lieber mit dem jeweiligen Paketmanager installieren.
- Vermeidung von `curl | sh`. Auf Unix installieren wir `rustup` normalerweise,
  indem wir per `curl` ein Shell-Skript ausführen. Manche haben Bedenken
  zur Sicherheit dieser Methode und würden den Installer lieber selber laden
  und ausführen.
- Validierung von Signaturen. Auch wenn `rustup` alle Downloads mit HTTPS
  durchführt, können die Signaturen des Installers momentan nur über die
  Verwendung der alleinstehenden Installer durchgeführt werden.
- GUI-Installation und Integration mit "Programme hinzufügen/entfernen" auf
  Windows. `rustup` läuft in der Konsole und registriert seine Installation
  nicht wie andere Windows-Programme. Wenn du Rust lieber mithilfe einer typische
  GUI installieren willst, nutze die für Windows erhältlichen standalone
  `.msi`-Installer. Zukünftig wird `rustup` für Windows einen GUI-Installer
  anbieten.

Die Plattformunterstützung von Rust ist in [drei Stufen] unterteilt, welche
eng mit den verfügbaren Installationsmethoden korrespondieren:
im allgemeinen bietet das Rust Projekt ausführbare Dateien für Plattformen mit
Stufen 1 und 2, und alle sind über `rustup` installierbar. Manche Plattformen
zweiter Stufe bieten allerdings nur die Standardbibliotheken und nicht den
Compiler an; das bedeutet, dass diese Plattformen lediglich Ziele für die
Cross-Compilation sind. Rust läuft auf diesen Plattformen, obwohl der
Compiler selbst nicht auf ihnen läuft.
Diese Ziele können mit dem Befehl `rustup target add` installiert werden.

## Andere Wege, `rustup` zu installieren
<span id="rustup"></span>

Dies hängt stark von der verwendeten Plattform ab:

* Auf Unix, führe in deiner Shell `curl https://sh.rustup.rs -sSf | sh`
  aus. Dies wird eine Datei [`rustup-init.sh`] herunterladen und ausführen.
  Dieses Skript wiederum wird die korrekte Version der ausführbaren
  Datei `rustup-init` herunterladen und ausführen.
* Auf Windows, lade [`rustup-init.exe`] herunter und führe die Datei aus.

`rustup-init` kann interaktiv konfiguriert werden, und alle Optionen können
zusätzlich mit Kommandozeilenargumenten angegeben werden, welche durch das
Shellskript übergeben werden können. Gib das Argument `--help` an `rustup-init`
folgendermaßen weiter, um die unterstützten Befehle anzuzeigen:

```
curl https://sh.rustup.rs -sSf | sh -s -- --help
```

Wenn du lieber kein Shellskript verwenden willst, kannst du direkt
`rustup-init` für eine Plattform deiner Wahl herunterladen:

<div class="rustup-init-table">
  {% for column in site.data.platforms.rustup %}
  <div>
    {% for target in column %}
    {% if target contains 'windows' %}
    <a href="https://static.rust-lang.org/rustup/dist/{{ target }}/rustup-init.exe">
      {{ target }}
    </a>
    {% else %}
    <a href="https://static.rust-lang.org/rustup/dist/{{ target }}/rustup-init">
      {{ target }}
    </a>
    {% endif %}
    {% endfor %}
  </div>
  {% endfor %}
</div>

## Standalone-Installer
<span id="standalone"></span>

Die offiziellen standalone-Installer enthalten eine einzige Version von Rust,
und sind für eine Installation ohne Internet geeignet. Sie sind als tarball,
welche in jeder Unix-artigen Umgebung laufen sollten (Erweiterung `.tar.gz`),
Windows Installer (`.msi`), und Mac-Installer (`.pkg`). Diese Installer
kommen mit `rustc`, `cargo`, `rustdoc`, der Standardbibliothek, und der
Standarddokumentation, bieten aber im Gegensatz zu `rustup` keinen Zugriff
auf weitere cross-targets.

Die häufigsten Gründe, diese zu benutzen, sind:

- Offline-Installation
- Vorzug der Nutzung eines Plattform-Integrierten, graphischen Installers
  auf Windows

Jede dieser Binaries sind mit dem [Rust signing key] signiert, welcher
auf [keybase.io] verfügbar ist. Die Rust build Infrastruktur signiert
alle Binaries automatisch mit GPG. In der folgenden Tabelle sind
die Signaturen als `.asc`-Dateien gelistet.

Vergangene Veröffentlichungen können in [den Archiven] gefunden werden.

{% for channel in site.channels %}

### {{ channel.name | capitalize }} ({{ channel.vers }})
<span id="{{ channel.name }}"></span>

<div class="installer-table {{ channel.name }}">
  {% for column in site.data.platforms[channel.name] %}
  <div>
    {% for target in column %}
    <div>
      <span>{{ target }}</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-{{ target }}.tar.gz">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-{{ target }}.tar.gz.asc">.asc</a>
    </div>
    {% if target contains 'windows' %}
    <div>
      <span>{{ target }}</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-{{ target }}.msi">.msi</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-{{ target }}.msi.asc">.asc</a>
    </div>
    {% elsif target contains 'darwin' %}
    <div>
      <span>{{ target }}</span>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-{{ target }}.pkg">.pkg</a>
      <a href="https://static.rust-lang.org/dist/rust-{{ channel.package }}-{{ target }}.pkg.asc">.asc</a>
    </div>
    {% endif %}
    {% endfor %}
  </div>
  {% endfor %}
</div>

{% endfor %}

## Quelltext
<span id="source"></span>

<div class="installer-table">
  <div>
    <div>
      <span>Stable</span>
      <a href="https://static.rust-lang.org/dist/rustc-{{ site.stable }}-src.tar.gz">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rustc-{{ site.stable }}-src.tar.gz.asc">.asc</a>
    </div>
  </div>
  <div>
    <div>
      <span>Beta</span>
      <a href="https://static.rust-lang.org/dist/rustc-beta-src.tar.gz">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rustc-beta-src.gz.asc">.asc</a>
    </div>
  </div>
  <div>
    <div>
      <span>Nightly</span>
      <a href="https://static.rust-lang.org/dist/rustc-nightly-src.tar.gz">.tar.gz</a>
      <a href="https://static.rust-lang.org/dist/rustc-nightly-src.tar.gz.asc">.asc</a>
    </div>
  </div>
</div>

[installation page]: install.html
[`rustup`]: https://github.com/rust-lang-nursery/rustup.rs
[other-rustup]: https://github.com/rust-lang-nursery/rustup.rs#other-installation-methods
[`rustup-init.exe`]: https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe
[`rustup-init.sh`]: https://static.rust-lang.org/rustup/rustup-init.sh
[Homebrew]: http://brew.sh/
[Chocolatey]: http://chocolatey.org/
[drei Stufen]: https://forge.rust-lang.org/platform-support.html
[Rust signing key]: https://static.rust-lang.org/rust-key.gpg.ascii
[GPG]: https://gnupg.org/
[available on keybase.io]: https://keybase.io/rust
[the archives]: https://static.rust-lang.org/dist/index.html
