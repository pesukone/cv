<script>
  import { dictionary, _ } from "svelte-i18n"

  dictionary.update(dict => ({
    fi: {
      ...dict.fi,
      other: "muuta",
      language: {
        skills: "kielitaito",
        finnish: "suomi",
        english: "englanti",
        swedish: "ruotsi",
        french: "ranska"
      },
      levels: {
        native: "äidinkieli",
        fluent: "erinomainen",
        basic: "tyydyttävä"
      }
    },
    en: {
      ...dict.en,
      other: "other things",
      language: {
        skills: "languages",
        finnish: "Finnish",
        english: "English",
        swedish: "Swedish",
        french: "French"
      },
      levels: {
        native: "native",
        fluent: "fluent",
        basic: "basic"
      }
    }
  }))
</script>

<style>
</style>

<h2>{$_.title("other")}</h2>

<ul>
  <li>{$_.title("language.skills")}
    <ul>
      <li>{$_("language.finnish")}: {$_("levels.native")}</li>
      <li>{$_("language.english")}: {$_("levels.fluent")}</li>
      <li>{$_("language.swedish")}: {$_("levels.basic")}</li>
      <li>{$_("language.french")}: {$_("levels.basic")}</li>
    </ul>
  </li>
  <li>Varusmiespalvelus: kersantti, PSVIESTIK, Panssariprikaati, 2016</li>
  <li>Tykkään sulkapallosta</li>
</ul>
