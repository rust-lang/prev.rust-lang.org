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
  bstrie:
    name: Ben Striegel
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
    members: [eddyb, nrc, pnkfelix, bkoropoff, nikomatsakis, aatch, dotdash, michaelwoerister]
    lead: nikomatsakis
  - name: Tooling and infrastructure
    responsibility: "tool support (e.g. Cargo, multirust), CI infrastructure, etc."
    members: [brson, nrc, alexcrichton, vadimcn, wycats, michaelwoerister]
    lead: alexcrichton
  - name: Community
    responsibility: "coordinating events, outreach, commercial users, teaching materials, and exposure"
    lead: steveklabnik
    members: [brson, skade, manishearth, steveklabnik, bstrie, erickt]
  - name: Moderation
    responsibility: "helping uphold the <a href='http://www.rust-lang.org/conduct.html'>code of conduct</a>"
    members: [mbrubeck, BurntSushi, manishearth, pnkfelix, erickt, niconii]
    email: rust-mods [at] rust-lang [dot] org

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

The Rust project is
[governed](https://github.com/rust-lang/rfcs/blob/master/text/1068-rust-governance.md)
by a number of teams, each focused on a specific area of concern. Below are
the rosters, in alphabetical order.

To contact a team, post your question or comment to [the Internals
forum](https://internals.rust-lang.org/) and tag your post with the category
corresponding to the team name. Note that security disclosures should follow
the [Rust security disclosure process](https://www.rust-lang.org/security.html). 

{% for team in page.teams %}
<section id="{{ team.name | replace:' ','-' }}">
<h2> {{ team.name }} team</h2>

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
    <div class="irc">irc: {% if person.irc %}{{ person.irc }}{% else %}{{ nick }}{% endif %}</div>
    <img class="headshot" src="{{ site.avatar | replace:'%nick',nick }}">
  </a>
</li>
{% endfor %}
</ul>
</section>
{% endfor %}
