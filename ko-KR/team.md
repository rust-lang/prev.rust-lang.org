---
layout: ko-KR/default
title: Rust 팀 &middot; Rust 프로그래밍 언어

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
  wycats:
    name: Yehuda Katz

# Information about each team. Omit `lead` for teams without leaders.
teams:
  - name: Core team
    responsibility: "overall direction of the project, subteam leadership, cross-cutting concerns"
    members: [brson, alexcrichton, wycats, steveklabnik, nikomatsakis, aturon, pcwalton, erickt]
  - name: Language design team
    responsibility: "designing new language features"
    members: [eddyb, nrc, pnkfelix, nikomatsakis, aturon]
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

localized:
  Core team:
    name: 코어 팀
    responsibility: "프로젝트의 전반적인 방향성, 서브팀 주도, 여러 팀을 아우르는 문제들"
  Language design team:
    name: 언어 설계 팀
    responsibility: "새 언어 기능의 개발"
  Library team:
    name: 라이브러리 팀
    responsibility: "Rust 표준 라이브러리, rust-lang 크레이트들, 코딩 규약들"
  Compiler team:
    name: 컴파일러 팀
    responsibility: "컴파일러 내부, 최적화"
  Tooling and infrastructure:
    name: 도구 및 인프라
    responsibility: "도구 지원 (Cargo, rustup 등), 지속적 통합(CI) 인프라 등"
  Community team:
    name: 커뮤니티 팀
    responsibility: "행사 조율, 소외층 지원, 상용 사용자들, 교육 자료들, 프로젝트 노출"
  Documentation team:
    name: 문서 팀
    responsibility: "Rust가 환상적인 문서를 가지도록 함"
  Moderation team:
    name: 중재 팀
    responsibility: "<a href='https://www.rust-lang.org/conduct.html'>행동 규약</a>이 잘 지켜지도록 도움"
  Style team:
    name: 스타일 팀
    responsibility: "(<a href='https://github.com/rust-lang/rfcs/blob/master/text/1607-style-rfcs.md'>RFC 1607</a>에 명시된 과정에 따라) 코드 스타일 가이드라인과 Rustfmt 설정의 결정을 맡는 임시 '공격대'"
  Rust team alumni:
    name: 옛 Rust 팀
    responsibility: "여유로운 은퇴 생활을 즐기는 중"

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

# Rust 팀

Rust 프로젝트는 여러 팀에 의해
[운영](https://github.com/rust-lang/rfcs/blob/master/text/1068-rust-governance.md)되며,
각 팀은 특정한 관심 영역에 주안점을 둡니다.
아래는 각 팀의 구성원으로 알파벳 순서를 따릅니다.

각 팀에게 연락하려면 [내부(internals) 포럼](https://internals.rust-lang.org/)에 질문이나 댓글을 남기고
글에 팀 이름에 해당하는 분류를 붙여 주세요.
보안 발표는 [Rust 보안 발표 절차](security.html)를 따라야 함을 유념해 주세요.

{% for team in page.teams %}
<section id="{{ team.name | replace:' ','-' }}">
<h2> {{ page.localized[team.name].name | default: team.name }} </h2>

<strong>책임</strong>: <em>{{ page.localized[team.name].responsibility | default: team.responsibility }}</em>

<br />

{% if team.email %}
  <strong>연락처</strong>:
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
