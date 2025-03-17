use leptos::prelude::*;

#[component]
pub fn Card(children: Children) -> impl IntoView {
  view! {
    <div class="flex flex-col items-center p-6 bg-white rounded-md shadow-md box-border outline-black/5 out line">
      {children()}
    </div>
  }
}
