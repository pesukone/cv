use crate::components::Card;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::{view, IntoView};
use std::time::Duration;

fn display_class(class: &str) -> &str {
  match class {
    "hidden" => "h-fit slide",
    "sliding" => "h-fit slide slide-target",
    "visible" => "h-fit slide-target",
    _ => "slide",
  }
}

#[island]
fn SkillCard(idx: usize, skill: String) -> impl IntoView {
  let animating = expect_context::<ReadSignal<Option<usize>>>();
  let set_animating = expect_context::<WriteSignal<Option<usize>>>();

  view! {
    <div
      class=move || {
        display_class(
          match animating() {
            None => "hidden",
            Some(animating) => if animating >= idx { "sliding" } else { "hidden" }
          },
        )
      }
      on:transitionend=move |_| {
        log!("AKJSNDKJASNDKJSAD");
        set_animating(Some(idx + 1));
      }
    >
      {skill}
    </div>
  }
}

#[island]
pub fn Skills() -> impl IntoView {
  let (animating, set_animating) = signal(None::<usize>);
  provide_context(animating);
  provide_context(set_animating);

  let skills = vec![
    "JavaScript",
    "TypeScript",
    "HTML",
    "CSS",
    "React",
    "Redux",
    "REST",
    "Agile",
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
    "Tailwind",
    "Git",
  ];

  Effect::new(move |_| {
    set_timeout(
      move || {
        set_animating(Some(0));
      },
      Duration::from_secs(1),
    );
  });

  let mut skill_cards = skills.into_iter().enumerate().map(|(idx, skill)| {
    view! { <SkillCard idx=idx skill=skill.to_string() /> }
  });

  const ROW_LENGTH: usize = 8;

  let mut skills_and_lines = Vec::new();

  loop {
    let skills = skill_cards.by_ref().take(ROW_LENGTH).collect_view();
    let len = skills.len();

    skills_and_lines.push(view! {
      {skills}
      <hr class="col-span-8 h-2 bg-black border-none" />
    });

    if len < ROW_LENGTH {
      break;
    }
  }

  view! {
    <h2>Skills</h2>
    <div class="grid grid-cols-8 gap-1 grid-rows-[24px_8px_24px_8px_24px]">
      {skills_and_lines.collect_view()}
    </div>
  }
}
