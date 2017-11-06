---
layout: sv-SE/default
title: Rustteamet &middot; Programmeringsspråket Rust
extra_css:
  - team.css
---

# Rustteamet

Rustprojektet [styrs](https://github.com/rust-lang/rfcs/blob/master/text/1068-rust-governance.md)
by a number of teams som var och en fokuserade på ett specifikt problemområde.
Nedan finns laguppställningar i alfabetisk ordning.

För att kontakta ett team, skicka in din fråga eller kommentar på
[internforumet](https://internals.rust-lang.org/) och märk ditt inskick med
kategorin som motsvarar teamets namn. Notera att tillkännagivanden kring säkerhet
ska följa [rusts säkherhetstillkännagivandeprocess](security.html). 

{% for team in site.data.team.teams %}
<section id="{{ team.name | replace:' ','-' }}">
<h2> {{ page.localized-teams[team.name].name | default: team.name }} </h2>

<strong>Ansvar</strong>: <em>{{ page.localized-teams[team.name].responsibility | default: team.responsibility }}</em>

<br />

{% if team.email %}
  <strong>Kontakt</strong>:
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
