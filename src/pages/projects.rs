use leptos::*;
use crate::components::{ProjectCard, Loading};
use crate::api::fetch_github_repos;

#[component]
pub fn ProjectsPage() -> impl IntoView {
    let projects = create_resource(
        || (),
        |_| async move { fetch_github_repos("skanjalkar").await }
    );
    
    view! {
        <div>
            <h1 class="section-title">"Projects"</h1>
            
            <Suspense fallback=move || view! { <Loading /> }>
                {move || projects.get().map(|result| {
                    match result {
                        Ok(repos) => view! {
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                                {repos.into_iter().map(|project| {
                                    view! { <ProjectCard project=project /> }
                                }).collect_view()}
                            </div>
                        }.into_view(),
                        Err(e) => view! {
                            <p class="text-red-400">"Failed to load projects: " {e.to_string()}</p>
                        }.into_view(),
                    }
                })}
            </Suspense>
        </div>
    }
}