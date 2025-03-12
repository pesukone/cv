use leptos::prelude::*;
use leptos::{view, IntoView};
use std::time::Duration;

fn display_class(class: &str) -> &str {
  match class {
    "hidden" => "hidden slide-2",
    "block" => "block slide-2",
    _ => "hidden slide-2",
  }
}

#[island]
pub fn Skills() -> impl IntoView {
  let (animating, set_animating) = signal(None);

  let skills = vec![
    "JavaScript",
    "TypeScript",
    "HTML",
    "CSS",
    "React",
    "Redux",
    "REST",
    "GraphQL",
    "Node.js",
    "SQL",
    "Docker",
    "Kubernetes",
    "Nix",
    "Ansible",
    "C++",
    "Go",
    "Rust",
  ];

  Effect::new(move |_| {
    set_timeout(
      move || {
        set_animating(Some(0));
      },
      Duration::from_secs(1),
    );
  });

  let skill_components = skills
    .iter()
    .enumerate()
    .map(|(idx, skill)| {
      view! {
        <div
          class=move || {
              display_class(
                  match animating() {
                      None => "hidden",
                      Some(i) => if i >= idx { "block" } else { "hidden" }
                  },
              )
          }
          on:animationend=move |_| set_animating(Some(idx + 1))
        >
          {skill.to_string()}
        </div>
      }
    })
    .collect_view();

  view! { <div class="flex flex-row gap-1">{skill_components}</div> }
}
