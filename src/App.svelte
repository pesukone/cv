<script>
  import { onMount } from "svelte"
  import { locale, dictionary, getClientLocale, _ } from "svelte-i18n"

  import Education from "./Education.svelte"
  import Projects from "./Projects.svelte"
  import Skills from "./Skills.svelte"
  import Rest from "./Rest.svelte"

  const headers = new Headers({
    "User-Agent": "pesukone-cv"
  })

  let dateOfLatestCommit

  onMount(async () => {
    const res = await fetch("https://api.github.com/repos/pesukone/cv/commits", {
      headers: headers
    })

    const array = await res.json()
    
    dateOfLatestCommit = new Date(array[0].commit.author.date)
  })

  locale.set(
    getClientLocale({
      fallback: "fi",
      navigator: true,
      search: 'lang',
      hash: 'locale'
    })
  )

  dictionary.set({
    fi: {
      email: "sähköposti",
      phone: "puhelin",
      education: "koulutus",
      highschool: "Ylioppilas, Etelä-Tapiolan luokio",
    },
    en: {
      email: "email",
      phone: "phone",
      education: "education",
      highschool: "Upper secondary education, Etelä-Tapiolan lukio"
    }
  })
</script>

<style>
</style>
<h1>
  Curriculum Vitae - Jussi Aalto
  {dateOfLatestCommit ? dateOfLatestCommit.toLocaleDateString("fi-FI") : ""}
</h1>

Github: <a href="https://www.github.com/pesukone">https://www.github.com/pesukone</a>
<br>
{$_.capital("email")}: <a href="mailto:j.v.aalto@gmail.com">j.v.aalto@gmail.com</a>
<br>
{$_.capital("phone")}: +358500925583

<section><Education /></section>

<section><Projects /></section>

<section><Skills /></section>

<section><Rest /></section>
