<script>
  import { onMount } from "svelte"
  import { dictionary, _ } from "svelte-i18n"

  onMount(() => {
    dictionary.update(dict => ({
      fi: {
        ...dict.fi,
        education: "koulutus",
        bachelorsDegree: "Luonnontieteiden kandidaatti, tietojenkäsittelytiede, Helsingin yliopisto",
        highschool: "Ylioppilas, Etelä-Tapiolan lukio"
      },
      en: {
        ...dict.en,
        education: "education",
        bachelorsDegree: "Bachelor of Science in Computer Science, University of Helsinki",
        highschool: "Upper secondary education, Etelä-Tapiolan lukio"
      }
    }))
  })
</script>

<style>
  ol li {
    list-style-type: disc;
  }
</style>

<h2>{$_.capital("education")}</h2>

<ol>
  <li>{$_("bachelorsDegree")}, 10.6.2019</li>
  <li>{$_("highschool")}, 2015</li>
</ol>
