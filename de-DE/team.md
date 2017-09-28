---
layout: de-DE/default
title: Das Rust-Team &middot; Die Programmiersprache Rust
extra_css:
  - team.css
---

# Das Rust-Team

Das Rust-Projekt wird von mehreren Teams [verwaltet](https://github.com/rust-lang/rfcs/blob/master/text/1068-rust-governance.md), welche sich auf verschiedene Themengebiete spezialisiert haben.
Unten werden sie in alphabetischer Reihenfolge aufgelistet.

Um ein Team zu kontaktieren, poste deine Frage oder deinen Kommentar im [Internals
Forum](https://internals.rust-lang.org/) und tagge deinen Post mit der zum Teamnamen korrespondierenden Kategorie.
Sicherheitsmeldungen sollten dem [Veröffentlichungsprozess für Sicherheitslücken](security.html) folgen.

{% for team in site.data.team.teams %}
<section id="{{ team.name | replace:' ','-' }}">
<h2> {{ page.localized-teams[team.name].name | default: team.name }} </h2>

<strong>Responsibility</strong>: {{ page.localized-teams[team.name].responsibility | default: team.responsibility }}

<br />
<br />

{% if team.email %}
  <strong>Contact</strong>:
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
