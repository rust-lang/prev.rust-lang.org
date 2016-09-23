---
layout: default
title: L'équipe Rust &middot; Rust, le langage de programmation

# Correspondance entre les noms d'utilisateur GitHub, les noms
# et les pseudonymes IRC (les pseudonymes IRC peuvent être omis
# lorsqu'ils correspondent aux noms d'utilisateurs GitHub).
people:
  aatch:
    name: James Miller
  arielb1:
    name: Ariel Ben-Yehuda
  alexcrichton:
    name: Alex Crichton
    irc: acrichto
  aturon:
    name: Aaron Turon
  bkoropoff:
    name: Brian Koropoff
  brson:
    name: Brian Anderson
  bstrie:
    name: Ben Striegel
  BurntSushi:
    name: Andrew Gallant
    irc: burntsushi
  carols10cents:
    name: Carol Nichols
  dotdash:
    name: Björn Steinbrink
    irc: doener
  eddyb:
    name: Eduard Burtescu
  edunham:
    name: Emily Dunham
  erickt:
    name: Erick Tryzelaar
  Gankro:
    name: Alexis Beingessner
    ex-teams: ["libs"]
  huonw:
    name: Huon Wilson
    irc: huon
    ex-teams: ["core", "lang", "libs"]
  GuillaumeGomez:
    name: Guillaume Gomez
    irc: imperio
  jonathandturner:
    name: Jonathan Turner
    irc: jntrnr
  jseyfried:
    name: Jeffrey Seyfried
    irc: jseyfried
  Kimundi:
    name: Marvin Löbel
    irc: kimundi
  manishearth:
    name: Manish Goregaokar
    irc: Manishearth
  mbrubeck:
    name: Matt Brubeck
  michaelwoerister:
    name: Michael Woerister
    irc: mw
  niconii:
    name: Nicolette Verlinden
    irc: niconii
  nikomatsakis:
    name: Niko Matsakis
    irc: nmatsakis
  nrc:
    name: Nick Cameron
  pcwalton:
    name: Patrick Walton
  peschkaj:
    name: Jeremiah Peschka
    irc: peschkaj
  pnkfelix:
    name: Felix Klock
  sfackler:
    name: Steven Fackler
  skade:
    name: Florian Gilcher
  steveklabnik:
    name: Steve Klabnik
  vadimcn:
    name: Vadim Chugunov
  wycats:
    name: Yehuda Katz
  jonathandturner:
    name: Jonathan Turner
  badboy:
    name: Jan-Erik Rediger
  johannhof:
    name: Johann Hofmann
  booyaa:
    name: Mark Sta Ana

# Informations à propos de chaque équipe. On omettra `lead` pour les
# équipes sans responsable.
teams:
  - name: Équipe principale
    responsibility: "direction générale du projet, gestion des différentes équipes, sujets transverses"
    members: [brson, alexcrichton, wycats, steveklabnik, nikomatsakis, aturon, pcwalton, erickt]
  - name: Équipe de conception du langage
    responsibility: "concevoir les nouvelles fonctionnalités du langage"
    members: [eddyb, nrc, pnkfelix, nikomatsakis, aturon]
    lead: nikomatsakis
  - name: Équipe pour la bibliothèque
    responsibility: "la bibliothèque standard de Rust, les *crates* rust-lang et les conventions"
    members: [brson, alexcrichton, sfackler, BurntSushi, Kimundi, aturon]
    lead: aturon
  - name: Équipe du compilateur
    responsibility: "le fonctionnement interne du compilateur et les optimisations"
    members: [arielb1, eddyb, nrc, pnkfelix, bkoropoff, nikomatsakis, aatch, dotdash, michaelwoerister, jseyfried]
    lead: nikomatsakis
  - name: Outils et infrastructure
    responsibility: "prise en charge des outils (ex. Cargo, rustup), infrastructure d'intégration continue, etc."
    members: [brson, nrc, alexcrichton, vadimcn, wycats, michaelwoerister]
    lead: alexcrichton
  - name: Équipe pour la communauté
    responsibility: "coordinations d'événements, rayonnement du projet, utilisateurs commerciaux, outils pédagogiques et publicité du projet"
    lead: erickt
    members: [brson, skade, manishearth, johannhof, steveklabnik, carols10cents, badboy, booyaa, bstrie, erickt, jonathandturner, edunham]
    email: community-team@rust-lang.org
  - name: Équipe de documentation
    responsibility: "s'assurer que Rust dispose d'une documentation fantastique"
    members: [steveklabnik, GuillaumeGomez, jonathandturner, peschkaj]
  - name: Équipe de modération
    responsibility: "s'assurer de la bonne tenue du <a href='https://www.rust-lang.org/conduct.html'>code de conduite</a>"
    members: [mbrubeck, BurntSushi, manishearth, pnkfelix, niconii]
    email: rust-mods@rust-lang.org
  - name: Équipe des anciens
    responsibility: "profiter d'une retraite paisible"
    members: [Gankro, huonw]

# Les sites sur lesquels obtenir les informations
sites:
  github:
    url: https://github.com/%nick
    avatar: https://avatars.githubusercontent.com/%nick
  twitter:
    url: https://twitter.com/%nick
    avatar: https://avatars.io/twitter/%nick?size=large
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

{% for team in page.teams %}
<section id="{{ team.name | replace:' ','-' }}">
<h2> {{ team.name }} </h2>

<strong>Responsabilité</strong> : <em>{{ team.responsibility }}</em>

<br />

{% if team.email %}
  <strong>Contact</strong>:
  <a href="mailto:{{ team.email | uri_escape }}">{{ team.email }}</a>
{% endif %}

<ul class="headshots">
{% for nick in team.members %}
  {% assign person = page.people[nick] %}
  {% if person.site %}
    {% assign sitename = person.site %}
  {% else %}
    {% assign sitename = "github" %}
  {% endif %}
  {% assign site = page.sites[sitename] %}
  <li class="person {% if team.lead and team.lead == nick %}lead{% endif %}">
  <a href="{{ site.url | replace:'%nick',nick }}">
    <div class="name">{{ person.name }}</div>
    <div class="details">
      <div>irc : {% if person.irc %}{{ person.irc }}{% else %}{{ nick }}{% endif %}</div>
      {% if person.ex-teams %}
      <div>équipes : {{ person.ex-teams | join: ", " }}</div>
      {% endif %}
    </div>
    <img class="headshot" src="{{ site.avatar | replace:'%nick',nick }}" alt="{{ person.name }}">
  </a>
</li>
{% endfor %}
</ul>
</section>
{% endfor %}
