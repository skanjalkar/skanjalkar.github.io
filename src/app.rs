use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::Layout;
use crate::pages::{BlogPage, BlogPostPage, HomePage, NotFoundPage, ProjectsPage, TerminalPage};
use crate::state::AppStateProvider;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Shreyas Kanjalkar | Software Engineer"/>
        <Meta name="description" content="Software Engineer at AWS DSQL Storage passionate about distributed systems and cloud computing."/>

        <AppStateProvider>
            <Router>
                <Layout>
                    <Routes>
                        <Route path="/" view=HomePage/>
                        <Route path="/projects" view=ProjectsPage/>
                        <Route path="/blog" view=BlogPage/>
                        <Route path="/blog/:slug" view=BlogPostPage/>
                        <Route path="/terminal" view=TerminalPage/>
                        <Route path="/*any" view=NotFoundPage/>
                    </Routes>
                </Layout>
            </Router>
        </AppStateProvider>
    }
}
