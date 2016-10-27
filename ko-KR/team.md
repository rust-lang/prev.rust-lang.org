---
layout: ko-KR/default
title: Rust 팀 &middot; Rust 프로그래밍 언어

localized-teams:
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
    responsibility: "행사 조율, 외부 연락, 상용 사용자들, 교육 자료들, 프로젝트 노출"
  Documentation team:
    name: 문서 팀
    responsibility: "Rust가 멋진 문서를 가지도록 함"
  Moderation team:
    name: 중재 팀
    responsibility: "<a href='https://www.rust-lang.org/conduct.html'>행동 규약</a>이 잘 지켜지도록 도움"
  Style team:
    name: 스타일 팀
    responsibility: "(<a href='https://github.com/rust-lang/rfcs/blob/master/text/1607-style-rfcs.md'>RFC 1607</a>에 명시된 과정에 따라) 코드 스타일 가이드라인과 Rustfmt 설정의 결정을 맡는 임시 '공격대'"
  Rust team alumni:
    name: 옛 Rust 팀
    responsibility: "여유로운 은퇴 생활을 즐기는 중"

localized-ex-teams:
  core: 코어
  lang: 언어설계
  libs: 라이브러리
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
.lead .name::after { content: " (팀장)"; }
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

{% for team in site.data.team.teams %}
<section id="{{ team.name | replace:' ','-' }}">
<h2> {{ page.localized-teams[team.name].name | default: team.name }} </h2>

<strong>책임</strong>: <em>{{ page.localized-teams[team.name].responsibility | default: team.responsibility }}</em>

<br />

{% if team.email %}
  <strong>연락처</strong>:
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
      <div>팀: {% for ex-team in person.ex-teams %}{% if forloop.first == false %}, {% endif %}{{ page.localized-ex-teams[ex-team] | default: ex-team }}{% endfor %}</div>
      {% endif %}
    </div>
    <img class="headshot" src="{{ website.avatar | replace:'%nick',nick }}" alt="{{ person.name }}">
  </a>
</li>
{% endfor %}
</ul>
</section>
{% endfor %}
