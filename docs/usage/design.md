---
layout: base.njk
title: Looksyk - Design
---

<div class="theme theme-overview">
<iframe src="{{config.pathPrefix}}usage/theme_overview" frameborder="0" onload='javascript:(function(o){o.style.height=o.contentWindow.document.body.scrollHeight+"px";}(this));' style="height:200px;width:100%;border:none;overflow:hidden;"></iframe>
<div class="iframe-subtitle">Customize Looksyk's design to your liking</div>
</div>


The design is based on the [Material Design](https://material.io/design) guidelines.

There are three ways to customize the design:

* Change the design or single color variables in the `settings` tab in the sidebar
* Changing predefined color variables in the `config/config.json` file (see description below)
* Adding custom CSS in the `config/user-theme.css` file.

The following color values are supported:

* Predefined browser-colors (e.g. `black`, `white`)
* RGB `rgb(R, G, B)` (e.g. `rgb(255, 0, 0)`)
* RGBA `rgba(R, G, B, A)` (e.g. `rgba(255, 0, 0, 0.5)`)
* Hex `#RRGGBB` (e.g. `#FF0000`)
* HSL `hsl(H, S%, L%)` (e.g. `hsl(0, 100%, 50%)`)
* HSLA `hsla(H, S%, L%, A)` (e.g. `hsla(0, 100%, 50%, 0.5)`)

The design consists of the following elements:

* `primaryColor` is the color of the primary elements (e.g. links, buttons)
* `backgroundColor` is the color of the background
* `foregroundColor` is the color of the text
* `primaryShading` is the color of the shading, used for the sidebar, the hover effect and the active element
* `appearance` is the appearance of the application, either `dark` or `light`. This is used to set the default color
  scheme of the application. The default value is `dark`.

Inspire yourself with the following examples (all avaliable in the settings tab):

<div class="theme">
<img src="{{config.pathPrefix}}usage/themes/theme0.png">
<div class="image-subtitle">Theme: Glacier (default)</div>
</div>

<div class="theme">
<img src="{{config.pathPrefix}}usage/themes/theme1.png">
<div class="image-subtitle">Theme: Glacier (default)</div>
</div>

<div class="theme">
<img src="{{config.pathPrefix}}usage/themes/theme2.png">
<div class="image-subtitle">Theme: Glacier (default)</div>
</div>

<div class="theme">
<img src="{{config.pathPrefix}}usage/themes/theme3.png">
<div class="image-subtitle">Theme: Glacier (default)</div>
</div>

<div class="theme">
<img src="{{config.pathPrefix}}usage/themes/theme4.png">
<div class="image-subtitle">Theme: Glacier (default)</div>
</div>

<div class="theme">
<img src="{{config.pathPrefix}}usage/themes/theme5.png">
<div class="image-subtitle">Theme: Glacier (default)</div>
</div>

<div class="theme">
<img src="{{config.pathPrefix}}usage/themes/theme6.png">
<div class="image-subtitle">Theme: Glacier (default)</div>
</div>

<div class="theme">
<img src="{{config.pathPrefix}}usage/themes/theme7.png">
<div class="image-subtitle">Theme: Glacier (default)</div>
</div>

