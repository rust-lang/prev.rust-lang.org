---
layout: pt-BR/default
title: O time Rust &middot; A linguagem de programação Rust

localized-teams:
  Core team:
    name: Time central
    responsibility: "direção geral do projeto, liderança de subtimes, outras preocupações"
  Language design team:
    name: Time de design da linguagem
    responsibility: "pensa em novas features para a linguagem"
  Library team:
    name: Time de bibliotecas
    responsibility: "a biblioteca padrão de Rust, crates da rust-lang, padrões"
  Compiler team:
    name: Time do compilador
    responsibility: "internos do compilador, optimizações"
  Tooling and infrastructure:
    name: Ferramentas e infraestrutura
    responsibility: "suporte de ferramentas (ex.: Cargo, rustup), Infraestrutura CI, etc."
  Community team:
    name: Time da comunidade
    responsibility: "coordenar eventos, propaganda, usuários comerciais, materiais de ensino e exposição"
  Documentation team:
    name: Time da documentação
    responsibility: "se certificam de que Rust possui uma incrível documentação"
  Moderation team:
    name: Time da moderação
    responsibility: "mantêm firme o <a href='https://www.rust-lang.org/conduct.html'>codigo de conduta</a>"
  Rust team alumni:
    name: Time Rust alumni
    responsibility: "curtir um calmo retiro"
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

# O Time Rust

O projeto Rust é [governado](https://github.com/rust-lang/rfcs/blob/master/text/1068-rust-governance.md)
por inúmeros times, cada um focado em uma área específica. Abaixo estão eles, em ordem alfabética.

Para contactar um time, poste sua pergunta ou comentário no [Fórum dos Internos](https://internals.rust-lang.org/) e
rotule a sua questão com a categoria que possui o nome do time. Note que revelações de falhas de segurança
devem seguir o [Processo de revelação de falhas de Segurança](security.html).

{% for team in site.data.team.teams %}
<section id="{{ team.name | replace:' ','-' }}">
<h2> {{ page.localized-teams[team.name].name | default: team.name }} </h2>

<strong>Responsabilidade</strong>: <em>{{ page.localized-teams[team.name].responsibility | default: team.responsibility }}</em>

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
      <div>irc: {% if person.irc %}{{ person.irc }}{% else %}{{ nick }}{% endif %}</div>
      {% if person.ex-teams %}
      <div>teams: {% for ex-team in person.ex-teams %}{% if forloop.first == false %}, {% endif %}{{ page.localized-ex-teams[ex-team] | default: ex-team }}{% endfor %}</div>
      {% endif %}
    </div>
    <img class="headshot" src="{{ website.avatar | replace:'%nick',nick }}" alt="{{ person.name }}">
  </a>
</li>
{% endfor %}
</ul>
</section>
{% endfor %}
