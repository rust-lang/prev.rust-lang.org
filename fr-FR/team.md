---
layout: fr-FR/default
title: L'équipe Rust &middot; Rust, le langage de programmation

localized-teams:
  Core team:
    name: Équipe principale
    responsibility: "direction générale du projet, gestion des différentes équipes, sujets transverses"
  Language design team:
    name: Équipe de conception du langage
    responsibility: "concevoir les nouvelles fonctionnalités du langage"
  Library team:
    name: Équipe pour la bibliothèque
    responsibility: "la bibliothèque standard de Rust, les *crates* rust-lang et les conventions"
  Compiler team:
    name: Équipe du compilateur
    responsibility: "le fonctionnement interne du compilateur et les optimisations"
  Tooling and infrastructure:
    name: Outils et infrastructure
    responsibility: "prise en charge des outils (ex. Cargo, rustup), infrastructure d'intégration continue, etc."
  Community team:
    name: Équipe pour la communauté
    responsibility: "coordinations d'événements, rayonnement du projet, utilisateurs commerciaux, outils pédagogiques et publicité du projet"
  Documentation team:
    name: Équipe de documentation
    responsibility: "s'assurer que Rust dispose d'une documentation fantastique"
  Moderation team:
    name: Équipe de modération
    responsibility: "s'assurer de la bonne tenue du <a href='https://www.rust-lang.org/conduct.html'>code de conduite</a>"
  Rust team alumni:
    name: Équipe des anciens
    responsibility: "profiter d'une retraite paisible"
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

# L'équipe Rust

Le projet Rust est [gouverné](https://github.com/rust-lang/rfcs/blob/master/text/1068-rust-governance.md) par différentes équipes dont chacune est consacrée à un périmètre spécifique. La liste de ces équipes est détaillée ci-après.

Afin de contacter une équipe, vous pouvez poser votre question ou commenter sur [les forums internes](https://internals.rust-lang.org/) et étiqueter votre billet avec la catégorie correspondant à l'équipe. Note : les divulgations relatives aux problèmes de sécurité doivent respecter [le processus de divulgation du projet Rust quant aux problèmes de sécurité](security.html).

{% for team in site.data.team.teams %}
<section id="{{ team.name | replace:' ','-' }}">
<h2> {{ page.localized-teams[team.name].name | default: team.name }} </h2>

<strong>Responsabilité</strong> : <em>{{ page.localized-teams[team.name].responsibility | default: team.responsibility }}</em>

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
      <div>irc : {% if person.irc %}{{ person.irc }}{% else %}{{ nick }}{% endif %}</div>
      {% if person.ex-teams %}
      <div>équipes : {% for ex-team in person.ex-teams %}{% if forloop.first == false %}, {% endif %}{{ page.localized-ex-teams[ex-team] | default: ex-team }}{% endfor %}</div>
      {% endif %}
    </div>
    <img class="headshot" src="{{ website.avatar | replace:'%nick',nick }}" alt="{{ person.name }}">
  </a>
</li>
{% endfor %}
</ul>
</section>
{% endfor %}
