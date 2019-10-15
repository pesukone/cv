<script>
  import { dictionary, _ } from "svelte-i18n"

  dictionary.update(dict => ({
    fi: {
      ...dict.fi,
      projects: "projektit",
      KOwebsite: "Karjalaisen Osakunnan nettisivut",
      xoliba: "Xoliba-lautapeli, selaintoteutus ja tekoälyvastustaja"
    },
    en: {
      ...dict.en,
      projects: "projects",
      KOwebsite: "Website for Karjalainen Osakunta",
      xoliba: "Browser implementation and AI for the board game Xoliba"
    }
  }))
</script>

<style>
</style>

<h2>{$_.title("projects")}</h2>

<ul>
  <li>{$_("KOwebsite")} (<a href="https://github.com/ko-osakunta/osakunta">https://github.com/ko-osakunta/osakunta</a>), React, Redux, Firebase, REST</li>
  <li>{$_("xoliba")} (<a href="https://github.com/xoliba">https://github.com/xoliba</a>), JavaScript, webpack, WebSocket, Java, Scrum</li>
</ul>
