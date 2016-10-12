---
layout: default
title: The Rust Team &middot; The Rust Programming Language

# map from GH username to name & irc nick (irc nick can be omitted if it
# matches GH).
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
  badboy:
    name: Jan-Erik Rediger
  bkoropoff:
    name: Brian Koropoff
  booyaa:
    name: Mark Sta Ana
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
  japaric:
    name: Jorge Aparicio
  johannhof:
    name: Johann Hofmann
  jonathandturner:
    name: Jonathan Turner
    irc: jntrnr
  joshtriplett:
    name: Josh Triplett
    irc: JoshTriplett
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
  solson:
    name: Scott Olson
    irc: scott
  steveklabnik:
    name: Steve Klabnik
  vadimcn:
    name: Vadim Chugunov
  ubsan:
    name: Nicole Mazzuca
  withoutboats:
    name: Without Boats
  wycats:
    name: Yehuda Katz

# Information about each team. Omit `lead` for teams without leaders.
teams:
  - name: Core team
    responsibility: "overall direction of the project, subteam leadership, cross-cutting concerns"
    members: [brson, alexcrichton, wycats, steveklabnik, nikomatsakis, aturon, pcwalton, erickt]
  - name: Language design team
    responsibility: "designing new language features"
    members: [eddyb, nrc, pnkfelix, nikomatsakis, aturon, withoutboats]
    lead: nikomatsakis
  - name: Library team
    responsibility: "the Rust standard library, rust-lang crates, conventions"
    members: [brson, alexcrichton, sfackler, BurntSushi, Kimundi, aturon]
    lead: aturon
  - name: Compiler team
    responsibility: "compiler internals, optimizations"
    members: [arielb1, eddyb, nrc, pnkfelix, bkoropoff, nikomatsakis, aatch, dotdash, michaelwoerister, jseyfried]
    lead: nikomatsakis
  - name: Tooling and infrastructure
    responsibility: "tool support (e.g. Cargo, rustup), CI infrastructure, etc."
    members: [brson, nrc, alexcrichton, vadimcn, wycats, michaelwoerister]
    lead: alexcrichton
  - name: Community team
    responsibility: "coordinating events, outreach, commercial users, teaching materials, and exposure"
    lead: erickt
    members: [brson, skade, manishearth, johannhof, steveklabnik, carols10cents, badboy, booyaa, bstrie, erickt, jonathandturner, edunham]
    email: community-team@rust-lang.org
  - name: Documentation team
    responsibility: "ensuring Rust has fantastic documentation"
    members: [steveklabnik, GuillaumeGomez, jonathandturner, peschkaj]
  - name: Moderation team
    responsibility: "helping uphold the <a href='https://www.rust-lang.org/conduct.html'>code of conduct</a>"
    members: [mbrubeck, BurntSushi, manishearth, pnkfelix, niconii]
    email: rust-mods@rust-lang.org
  - name: Style team
    members: [brson, japaric, joshtriplett, nrc, solson, steveklabnik, ubsan]
    lead: nrc
    responsibility: "temporary 'strike team' charged with deciding on code style guidelines and configuring Rustfmt (process specified in <a href='https://github.com/rust-lang/rfcs/blob/master/text/1607-style-rfcs.md'>RFC 1607</a>)"
    email: style-team@rust-lang.org
  - name: Rust team alumni
    responsibility: "enjoying a leisurely retirement"
    members: [Gankro, huonw]

# Information on sites to get profile information from
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

# The Rust Team

The Rust project is
[governed](https://github.com/rust-lang/rfcs/blob/master/text/1068-rust-governance.md)
by a number of teams, each focused on a specific area of concern. Below are
the rosters, in alphabetical order.

To contact a team, post your question or comment to [the Internals
forum](https://internals.rust-lang.org/) and tag your post with the category
corresponding to the team name. Note that security disclosures should follow
the [Rust security disclosure process](security.html). 

{% for team in page.teams %}
<section id="{{ team.name | replace:' ','-' }}">
<h2> {{ team.name }} </h2>

<strong>Responsibility</strong>: <em>{{ team.responsibility }}</em>

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
      <div>irc: {% if person.irc %}{{ person.irc }}{% else %}{{ nick }}{% endif %}</div>
      {% if person.ex-teams %}
      <div>teams: {{ person.ex-teams | join: ", " }}</div>
      {% endif %}
    </div>
    <img class="headshot" src="{{ site.avatar | replace:'%nick',nick }}" alt="{{ person.name }}">
  </a>
</li>
{% endfor %}
</ul>
</section>
{% endfor %}
