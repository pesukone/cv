use crate::components::Card;
use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn FormalQualificationsView() -> impl IntoView {
  view! {
    <div class="h-screen" id="qualifications">
      <Certificates />
      <Education />
    </div>
  }
}

#[component]
fn Certificates() -> impl IntoView {
  view! {
    <h2>sertifikaatit</h2>
    <Card>
      <h3 class="font-bold">Google cloud developer vai mikä olikaan</h3>
      <h3 class="font-medium">2024</h3>
      <div class="self-end w-20 h-20">
        <Icon icon=icondata::IoRibbonOutline width="100%" height="100%" />
      </div>
    </Card>
  }
}

#[component]
fn Education() -> impl IntoView {
  view! {
    <h2>koulutus</h2>
    <ol>
      <li>LuK, tietojenkäsittelytiede, Helsingin yliopisto 2019</li>
      <li>Etelä-Tapiolan lukio 2015</li>
    </ol>
  }
}
