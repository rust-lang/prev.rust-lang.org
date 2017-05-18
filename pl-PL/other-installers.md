---
layout: pl-PL/default
title: Inne metody instalacji &middot; Język programowania Rust
---

# Inne metody instalacji Rusta

- [Którego instalatora powinieneś użyć?](#which)
- [Inne sposoby instalacji `rustup`](#more-rustup)
- [Samodzielne instalatory](#standalone)
- [Kod źródłowy](#source)

## Którego instalatora powinieneś użyć?
<span id="which"></span>

Rust działa na wielu platformach oraz jest wiele sposobów instalacji Rusta. Jeżeli
chcesz zainstalować Rusta w najprostszy, polecany sposób to kieruj się
instrukcjami na głównej [stronie instalacji].

Tamta strona opisuje instalację za pomocą [`rustup`]: narzędzia, które zarządza
wieloma zestawami narzędzi Rusta w konsekwentny sposób na wszystkich platformach
wspieranych przez Rusta. Czemu ktoś może _nie_ chcieć zainstalować
używając tych instrukcji?

- Instalacja offline. `rustup` pobiera potrzebne zależności z internetu.
  Jeżeli potrzebujesz zainstalować Rust bez dostępu do internetu, `rustup`
  nie jest do tego odpowiednim narzędziem.
- Preferencja dla systemowego menedżera pakietów. Na Linuksie w szczególności,
  ale także na macOS z [Homebrew] oraz na Windowsie z [Chocolatey], deweloperzy
  czasami wolą zainstalować Rusta za pomocą systemowego menedżera pakietów.
- Preferencja przeciw `curl | sh`. Na Uniksowych systemach zazwyczaj
  instalujemy `rustup` za pomocą uruchomienia skryptu przez `curl`. Niektórzy
  mają wątpliwości dotyczące bezpieczeństwa takiego rozwiązania i wolą pobrać
  oraz uruchomić instalator sami.
- Sprawdzenie sygnatur. Mimo, że `rustup` pobiera za pomocą HTTPS, chwwilowo jedynym
  sposobem na weryfikację sygnatur instalatorów Rusta jest zrobienie tego ręcznie
  za pomocą samodzielnego instalatora.
- Instalacja za pomocą instalatora z GUI oraz integracja z „Dodaj/Usuń Programy”
  na Windowsie. `rustup` działa w konsoli i nie rejestruje swojej instalacji jak
  typowy program Windowsowy. Jeżeli wolisz typowy instalator z GUI na Windowsie,
  dostępne są instalatory `.msi`. W przyszłości `rustup` będzie miał instalator z GUI
  na Windowsie.

Platformy obsługiwane przez Rusta są podzielone na [trzy rodzaje], które
mają dużo wspólnego z dostępnymi metodami instalacji: w ogólności, projekt Rust
dostarcza wydań binarnych dla wszystkich platform rodzaju 1 oraz rodzaju 2 oraz
są one możliwe do zainstalowania przy pomocy `rustup`. Niektóre platformy
rodzaju 2 mają dostępną tylko standardową bibliotekę, bez kompilatora;
a więc są one jedynie celami kompilacji skrośnej; kod w Ruście może być uruchomiony na
tych platformach, ale nie sam kompilator. Takie cele mogą być zainstalowane
za pomocą komendy `rustup target add`.

## Inne sposoby instalacji `rustup`
<span id="rustup"></span>

Sposób instalacji `rustup` zależy od platformy:

* Na systemach Uniksowych, uruchom `curl https://sh.rustup.rs -sSf | sh` w
  swojej konsoli. Zostanie pobrany i uruchomiony [`rustup-init.sh`], który
  pobierze i uruchomi odpowiednią wersję `rustup-init` dla twojej platformy.
* Na Windowsie, pobierz i uruchom [`rustup-init.exe`].

`rustup-init` może być skonfigurowane interaktywnie. Dodatkowo, wszystkie opcje mogą być
kontrolowane przez argumenty linii poleceń, które mogą być przekazane
przez skrypt konsolowy. Dodaj `--help` do uruchomienia `rustup-init`, jak poniżej,
aby wyświetlić argumenty akceptowane przez `rustup-init`:

```
curl https://sh.rustup.rs -sSf | sh -s -- --help
```

Jeżeli wolisz nie używać tego skryptu, pobierz bezpośrednio `rustup-init` dla
twojej platformy:

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

## Samodzielne instalatory
<span id="standalone"></span>

Oficjalne samodzielne instalatory Rusta zawierają pojedyncze wydanie Rusta i nadają
się do instalacji offline. Występują w trzech formatach: archiwa tar
(rozszerzenie `.tar.gz`), działające na systemach Uniksowych, instalatory
Windowsowe (`.msi`), oraz instalatory Mac (`.pkg`). Te instalatory zawierają
`rustc`, `cargo`, `rustdoc`, standardową bibliotekę oraz standardową
dokumentację, ale w przeciwieństwie do `rustup`, nie dają dostępu do dodatkowych celów kompilacji skrośnej.

Najczęstsze powody, aby ich użyć to:

- Instalacja offline
- Preferowanie bardziej zintegrowanego z platformą, graficznego
  instalatora w Windows

Każda z tych binarek jest podpisana z użyciem [klucza GPG Rusta][rust-gpg-key]
([dostępnego na keybase.io][keybase]) przez infrastrukturę budującą Rusta za pomocą
[GPG]. Pliki `.asc` w tabelach poniżej to sygnatury.

Poprzednie wydania można znaleźć w [archiwum].

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

## Kod źródłowy
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

[stronie instalacji]: install.html
[`rustup`]: https://github.com/rust-lang-nursery/rustup.rs
[other-rustup]: https://github.com/rust-lang-nursery/rustup.rs#other-installation-methods
[`rustup-init.exe`]: https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe
[`rustup-init.sh`]: https://static.rust-lang.org/rustup/rustup-init.sh
[Homebrew]: http://brew.sh/
[Chocolatey]: http://chocolatey.org/
[trzy rodzaje]: https://forge.rust-lang.org/platform-support.html
[rust-gpg-key]: https://static.rust-lang.org/rust-key.gpg.ascii
[GPG]: https://gnupg.org/
[keybase]: https://keybase.io/rust
[archiwum]: https://static.rust-lang.org/dist/index.html
