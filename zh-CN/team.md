---
layout: zh-CN/default
title: Rust 团队 &middot; Rust 程序设计语言

localized-teams:
  Core team:
    name: 核心团队
    responsibility: "把握项目的总体方向、领导子团队、横向关注"
  Language design team:
    name: 语言设计团队
    responsibility: "为语言设计新特性"
  Library team:
    name: 库团队
    responsibility: "Rust 标准库、Rust 语言的包及公约"
  Compiler team:
    name: 编译器开发团队
    responsibility: "compiler internals, optimizations"
  Tooling and infrastructure:
    name: 工具和基础组件团队
    responsibility: "提高工具支持 (譬如 Cargo、rustup)、CI 基础控件，等。"
  Community team:
    name: 社区团队
    responsibility: "coordinating events, outreach, commercial users, teaching materials, and exposure"
  Documentation team:
    name: 文档团队
    responsibility: "确保 Rust 拥有极好的文档"
  Moderation team:
    name: 审核团队
    responsibility: "helping uphold the <a href='https://www.rust-lang.org/conduct.html'>code of conduct</a>"
  Rust team alumni:
    name: Rust 团队旧友
    responsibility: "享受悠哉的退休生活"
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
.lead .name::after { content: " (负责人)"; }
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

Rust 的项目是由若干个团队分别
[管辖](https://github.com/rust-lang/rfcs/blob/master/text/1068-rust-governance.md)，
的每个团队只关注特定的内容，以下是团队人名单（姓氏字母排序）。

To contact a team, post your question or comment to [the Internals
forum](https://internals.rust-lang.org/) and tag your post with the category
corresponding to the team name. Note that security disclosures should follow
the [Rust security disclosure process](security.html). 

{% for team in site.data.team.teams %}
<section id="{{ team.name | replace:' ','-' }}">
<h2> {{ page.localized-teams[team.name].name | default: team.name }} </h2>

<strong>职能</strong>: <em>{{ page.localized-teams[team.name].responsibility | default: team.responsibility }}</em>

<br />

{% if team.email %}
  <strong>联系人</strong>:
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
      <div>团队: {% for ex-team in person.ex-teams %}{% if forloop.first == false %}, {% endif %}{{ page.localized-ex-teams[ex-team] | default: ex-team }}{% endfor %}</div>
      {% endif %}
    </div>
    <img class="headshot" src="{{ website.avatar | replace:'%nick',nick }}" alt="{{ person.name }}">
  </a>
</li>
{% endfor %}
</ul>
</section>
{% endfor %}
