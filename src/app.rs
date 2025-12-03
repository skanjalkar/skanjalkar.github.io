use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::Layout;
use crate::pages::{BlogPage, BlogPostPage, HomePage, NotFoundPage, ProjectsPage};
use crate::state::AppStateProvider;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Shreyas Kanjalkar | Software Engineer"/>
        <Meta name="description" content="MS CS student at Georgia Tech, Software Engineer passionate about distributed systems and cloud computing."/>

        <AppStateProvider>
            <Router>
                <Layout>
                    <Routes>
                        <Route path="/" view=HomePage/>
                        <Route path="/projects" view=ProjectsPage/>
                        <Route path="/blog" view=BlogPage/>
                        <Route path="/blog/:slug" view=BlogPostPage/>
                        <Route path="/*any" view=NotFoundPage/>
                    </Routes>
                </Layout>
            </Router>
        </AppStateProvider>
    }
}
