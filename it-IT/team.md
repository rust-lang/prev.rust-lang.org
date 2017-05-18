---
layout: it-IT/default
title: Il team di Rust &middot; Linguaggio di programmazione Rust

localized-teams:
  Core team:
    name: Team principale
    responsibility: "dirigono il progetto e i sottogruppi, lavorano su problematiche condivise"
  Language design team:
    name: Team di design del linguaggio
    responsibility: "progettano gli sviluppi del linguaggio"
  Library team:
    name: Team della libreria
    responsibility: "la libreria standard di rust, i *pacchetti* rust-lang e le convenzioni"
  Compiler team:
    name: Team del compilatore
    responsibility: "si occupano del compilatore e della sua ottimizzazione"
  Tooling and infrastructure:
    name: Utilità e infrastrutture
    responsibility: "lavorano agli strumenti (rustup, Cargo), alla piattaforma di controllo automatico, etc..."
  Community team:
    name: Team della comunità
    responsibility: "coordinano gli eventi, la riuscita, i rapporti con gli utenti commerciali e il materiale didattico"
  Documentation team:
    name: Team della documentazione
    responsibility: "assicurano che Rust abbia una documentazione fantastica"
  Moderation team:
    name: Team della moderazione
    responsibility: "assicurano il rispetto <a href='it-IT/conduct.html'>codice di comportamento</a>"
  Style team:
    name: Team dello stile
    responsibility: "gruppo di azione temporaneo che delibera sulla forma del linguaggio e sullo strumento rustfmt"
  Rust team alumni:
    name: Team degli anziani
    responsibility: "si godono un meritato riposo"
---

<style type="text/css">
.headshot {
  border: 1px solid #888;
  width: 140px;
}

.person {
  display: inline-block;
  position: relative;
  margin-bottom: 20px;
}
.lead { font-weight: bold; }
.lead .name::after { content: " (lead)"; }
.details {
  display: none;
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  background: rgba(0, 0, 0, 0.5);
  color: white;
  font-weight: normal;
}
.person:hover .details {
   display: block;
}

.headshots {
  text-align: center;
  margin: 0px auto;
  padding: 0;
  width: 700px;
  max-width: 100%;
  list-style: none;
}
</style>

# Il team di rust

Il progetto Rust è
[amministrato](https://github.com/rust-lang/rfcs/blob/master/text/1068-rust-governance.md)
da un numero di team, ciascuno focalizzato in una specifica area di competenza.
Sotto vi è una lista, in ordine alfabetico.

Per contattare un team, invia la tua domanda o commenta nel [Forum Sviluppo](https://internals.rust-lang.org/)
e tagga il tuo post con la categoria corrispondente al nome del team.
Nota che le vulnerabilità sono soggette al [processo di annuncio vulnerabilità](security.html). 

{% for team in site.data.team.teams %}
<section id="{{ team.name | replace:' ','-' }}">
<h2> {{ page.localized-teams[team.name].name | default: team.name }} </h2>

<strong>Responsabilità</strong>: <em>{{ page.localized-teams[team.name].responsibility | default: team.responsibility }}</em>

<br />

{% if team.email %}
  <strong>Contatto</strong>:
  <a href="mailto:{{ team.email | uri_escape }}">{{ team.email }}</a>
{% endif %}

<ul class="headshots">
{% for nick in team.members %}
  {% assign person = site.data.team.people[nick] %}
  {% if person.site %}
    {% assign sitename = person.site %}
  {% else %}
    {% assign sitename = "github" %}
  {% endif %}
  {% assign website = site.data.team.sites[sitename] %}
  <li class="person {% if team.lead and team.lead == nick %}lead{% endif %}">
  <a href="{{ website.url | replace:'%nick',nick }}">
    <div class="name">{{ person.name }}</div>
    <div class="details">
      <div>irc: {% if person.irc %}{{ person.irc }}{% else %}{{ nick }}{% endif %}</div>
      {% if person.ex-teams %}
      <div>teams: {% for ex-team in person.ex-teams %}{% if forloop.first == false %}, {% endif %}{{ page.localized-ex-teams[ex-team] | default: ex-team }}{% endfor %}</div>
      {% endif %}
    </div>
    <img class="headshot" src="{{ website.avatar | replace:'%nick',nick }}" alt="{{ person.name }}">
  </a>
</li>
{% endfor %}
</ul>
</section>
{% endfor %}
