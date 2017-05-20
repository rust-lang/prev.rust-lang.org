---
layout: zh-CN/default
title: Rust 团队 &middot; Rust 程序设计语言
extra_css:
  - team.css

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
    responsibility: "编译器内部、优化"
  Tooling and infrastructure:
    name: 工具和基础组件团队
    responsibility: "提高工具支持（例如 Cargo、rustup）、CI 基础控件，等等。"
  Community team:
    name: 社区团队
    responsibility: "协调活动、推广、商业用户、教材及曝光率"
  Documentation team:
    name: 文档团队
    responsibility: "确保 Rust 拥有极好的文档"
  Moderation team:
    name: 审核团队
    responsibility: "帮助维护<a href='https://www.rust-lang.org/conduct.html'>行为守则</a>"
  Rust team alumni:
    name: Rust 团队旧友
    responsibility: "享受悠哉的退休生活"
---

# Rust 团队

Rust 的项目是由若干个团队分别[管辖](https://github.com/rust-lang/rfcs/blob/master/text/1068-rust-governance.md)，
每个团队只关注特定的内容，以下是团队人员名单（按姓氏字母排序）。

要联系某个团队，请发布您的问题或评论至[内部论坛](https://internals.rust-lang.org/)，并用相应团队的名称标注您的帖子。请注意，安全披露应该遵循 
[Rust 安全披露流程](security.html)。

{% for team in site.data.team.teams %}
<section id="{{ team.name | replace:' ','-' }}">
<h2> {{ page.localized-teams[team.name].name | default: team.name }} </h2>

<strong>职能</strong>：<em>{{ page.localized-teams[team.name].responsibility | default: team.responsibility }}</em>

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
      <div>团队：{% for ex-team in person.ex-teams %}{% if forloop.first == false %}, {% endif %}{{ page.localized-ex-teams[ex-team] | default: ex-team }}{% endfor %}</div>
      {% endif %}
    </div>
    <img class="headshot" src="{{ website.avatar | replace:'%nick',nick }}" alt="{{ person.name }}">
  </a>
</li>
{% endfor %}
</ul>
</section>
{% endfor %}
