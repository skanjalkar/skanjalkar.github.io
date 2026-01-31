use crate::api::fetch_github_repos;
use crate::components::{Loading, ProjectCard};
use leptos::*;

#[component]
pub fn ProjectsPage() -> impl IntoView {
    let projects = create_local_resource(
        || (),
        |_| async move { fetch_github_repos("skanjalkar").await },
    );

    view! {
        <div>
            <h1 class="section-title section-title-animated animate-fade-in-down">"Projects"</h1>

            <Suspense fallback=move || view! { <Loading /> }>
                {move || projects.get().map(|repos| {
                    if repos.is_empty() {
                        view! {
                            <p class="text-gray-400">"Loading projects..."</p>
                        }.into_view()
                    } else {
                        view! {
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                                {repos.into_iter().enumerate().map(|(index, project)| {
                                    view! { <ProjectCard project=project index=index /> }
                                }).collect_view()}
                            </div>
                        }.into_view()
                    }
                })}
            </Suspense>
        </div>
    }
}
