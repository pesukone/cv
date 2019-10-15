<script>
  import { dictionary, _ } from "svelte-i18n"

  dictionary.update(dict => ({
    fi: {
      ...dict.fi,
      skills: "tekniset taidot",
      frontend: "selainpuolen web-ohjelmointi",
      backend: "palvelinpuolen web-ohjelmointi",
      functional: "funktionaalinen ohjelmointi",
      oop: "olio-ohjelmointi",
      vc: "versionhallinta",
      linux: "Linux päivittäisessä käytössä vuodesta 2012"
    },
    en: {
      ...dict.en,
      skills: "skills",
      frontend: "frontend",
      backend: "backend",
      functional: "functional programming",
      oop: "object-oriented programming",
      vc: "version control",
      linux: "used Linux daily since 2012"
    }
  }))
</script>

<style>
</style>

<h2>{$_.title("skills")}</h2>

<ul>
  <li>{$_.capital("frontend")}: HTML, CSS, React, Redux, REST, ES6</li>
  <li>{$_.capital("backend")}: MVC, SQL, NoSQL, Rails, Node.js</li>
  <li>{$_.capital("functional")}: Haskell</li>
  <li>{$_.capital("oop")}: Java</li>
  <li>{$_.capital("vc")}: Git</li>
  <li>{$_.capital("linux")}</li>
</ul>
