---
layout: zh-CN/default
title: Rust 团队 &middot; Rust 程序设计语言

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

# Information about each team. Omit `lead` for teams without leaders.
teams:
  - name: 核心团队
    responsibility: "overall direction of the project, subteam leadership, cross-cutting concerns"
    members: [brson, alexcrichton, wycats, steveklabnik, nikomatsakis, aturon, pcwalton, erickt]
  - name: 语言设计团队
    responsibility: "为语言设计新特性"
    members: [eddyb, nrc, pnkfelix, nikomatsakis, aturon]
    lead: nikomatsakis
  - name: 库团队
    responsibility: "Rust 标准库、Rust 语言的包及公约"
    members: [brson, alexcrichton, sfackler, BurntSushi, Kimundi, aturon]
    lead: aturon
  - name: 编译器开发团队
    responsibility: "compiler internals, optimizations"
    members: [arielb1, eddyb, nrc, pnkfelix, bkoropoff, nikomatsakis, aatch, dotdash, michaelwoerister, jseyfried]
    lead: nikomatsakis
  - name: 工具和基础组件团队
    responsibility: "提高工具支持 (譬如 Cargo、rustup)、CI 基础控件，等。"
    members: [brson, nrc, alexcrichton, vadimcn, wycats, michaelwoerister]
    lead: alexcrichton
  - name: 社区团队
    responsibility: "coordinating events, outreach, commercial users, teaching materials, and exposure"
    lead: erickt
    members: [brson, skade, manishearth, johannhof, steveklabnik, carols10cents, badboy, booyaa, bstrie, erickt, jonathandturner, edunham]
    email: community-team@rust-lang.org
  - name: 文档团队
    responsibility: "确保 Rust 拥有极好的文档"
    members: [steveklabnik, GuillaumeGomez, jonathandturner, peschkaj]
  - name: Moderation team
    responsibility: "helping uphold the <a href='https://www.rust-lang.org/conduct.html'>code of conduct</a>"
    members: [mbrubeck, BurntSushi, manishearth, pnkfelix, niconii]
    email: rust-mods@rust-lang.org
  - name: Rust 团队旧友
    responsibility: "享受悠哉的退休生活"
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

# Rust 团队

Rust 的项目是由若干个小组分别
[管辖](https://github.com/rust-lang/rfcs/blob/master/text/1068-rust-governance.md)，
每个小组只关注特定的内容，以下是小组人名单（姓氏字母排序）。

To contact a team, post your question or comment to [the Internals
forum](https://internals.rust-lang.org/) and tag your post with the category
corresponding to the team name. Note that security disclosures should follow
the [Rust security disclosure process](security.html). 

{% for team in page.teams %}
<section id="{{ team.name | replace:' ','-' }}">
<h2> {{ team.name }} </h2>

<strong>职能</strong>: <em>{{ team.responsibility }}</em>

<br />

{% if team.email %}
  <strong>联系人</strong>:
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
  <li class="person {% if team.lead and team.lead == nick %}负责人{% endif %}">
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
