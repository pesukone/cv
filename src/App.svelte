<script>
  import { onMount } from "svelte"
  import Scroller from "@sveltejs/svelte-scroller"

  import Education from "./Education.svelte"
  import Projects from "./Projects.svelte"
  import Skills from "./Skills.svelte"
  import Rest from "./Rest.svelte"

  const headers = new Headers({
    "User-Agent": "pesukone-cv"
  })

  let dateOfLatestCommit

  onMount(async () => {
    const res = await fetch("https://api.github.com/repos/ko-osakunta/osakunta/commits", {
      headers: headers
    })

    const array = await res.json()
    
    dateOfLatestCommit = new Date(array[0].commit.author.date)
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
Sähköposti: <a href="mailto:j.v.aalto@gmail.com">j.v.aalto@gmail.com</a>
<br>
Puhelin: 0500925583

<section><Education /></section>

<section><Projects /></section>

<section><Skills /></section>

<section><Rest /></section>
