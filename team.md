---
layout: default
title: The Rust Team

# map from GH username to name & irc nick (irc nick can be omitted if it
# matches GH).
people:
  aatch:
    name: James Miller
  alexcrichton:
    name: Alex Crichton
    irc: acrichto
  aturon:
    name: Aaron Turon
  bkoropoff:
    name: Brian Koropoff
  brson:
    name: Brian Anderson
  BurntSushi:
    name: Andrew Gallant
    irc: burntsushi
  dotdash:
    name: Björn Steinbrink
    irc: doener
  eddyb:
    name: Eduard Burtescu
  erickt:
    name: Erick Tryzelaar
  Gankro:
    name: Alexis Beingessner
  huonw:
    name: Huon Wilson
    irc: huon
  "jakub-":
    name: Jakub Bukaj
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
  nikomatsakis:
    name: Niko Matsakis
    irc: nmatsakis
  nrc:
    name: Nick Cameron
  pcwalton:
    name: Patrick Walton
  pnkfelix:
    name: Felix Klock
  sfackler:
    name: Steven Fackler
  steveklabnik:
    name: Steve Klabnik
  vadimcn:
    name: Vadim Chugunov
  wycats:
    name: Yehuda Katz

# Information about each team. Omit `lead` for teams without leaders.
teams:
  - name: Core
    responsibility: "overall direction of the project, subteam leadership, cross-cutting concerns"
    members: [brson, alexcrichton, wycats, steveklabnik, nikomatsakis, aturon, pcwalton, huonw]
  - name: Language design
    responsibility: "designing new language features"
    members: [nrc, pnkfelix, nikomatsakis, aturon, huonw]
    lead: nikomatsakis
  - name: Library
    responsibility: "the Rust standard library, rust-lang crates, conventions"
    members: [brson, Gankro, alexcrichton, sfackler, BurntSushi, Kimundi, aturon, huonw]
    lead: aturon
  - name: Compiler
    responsibility: "compiler internals, optimizations"
    members: ["jakub-", eddyb, nrc, pnkfelix, bkoropoff, nikomatsakis, aatch, dotdash, michaelwoerister]
    lead: nikomatsakis
  - name: Tooling and infrastructure
    responsibility: "tool support (e.g. Cargo, multirust), CI infrastructure, etc."
    members: [brson, nrc, alexcrichton, vadimcn, wycats, michaelwoerister]
    lead: alexcrichton
  - name: Moderation
    responsibility: "helping uphold the [code of conduct](http://www.rust-lang.org/conduct.html)"
    members: [mbrubeck, BurntSushi, manishearth, pnkfelix, erickt]
    email: rust-mods@googlegroups.com
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
.irc {
  display: none;
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  background: rgba(0, 0, 0, 0.5);
  color: white;
  font-weight: normal;
}
.person:hover .irc {
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

The Rust project is [governed](https://github.com/rust-lang/rfcs/blob/master/text/1068-rust-governance.md) by a
number of teams, each focused on a specific area of concern. Below are the
rosters, in alphabetical order.

{% for team in page.teams %}
<section id="{{ team.name | replace:' ','-' }}">
## {{ team.name }} team

**Responsibility**: *{{ team.responsibility }}*

{% if team.email %}
  **Contact**:
  <a href="mailto:{{ team.email | uri_escape }}">{{ team.email }}</a>
{% endif %}

<ul class="headshots">
{% for github in team.members %}
  {% assign person = page.people[github] %}
  <li class="person {% if team.lead and team.lead == github %}lead{% endif %}">
  <a href="https://github.com/{{ github }}">
    <div class="name">{{ person.name }}</div>
    <div class="irc">irc: {% if person.irc %}{{ person.irc }}{% else %}{{ github }}{% endif %}</div>
    <img class="headshot" src="http://avatars.githubusercontent.com/{{ github }}">
  </a>
</li>
{% endfor %}
</ul>
</section>
{% endfor %}
