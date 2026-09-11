use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::Layout;
use crate::models::Profile;
use crate::pages::{
    AboutPage, BlogPage, BlogPostPage, HomePage, NotFoundPage, ProjectPage, ProjectsPage,
    TerminalPage,
};
use crate::state::AppStateProvider;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text=format!("{} | A personal workshop", Profile::default().username)/>
        <Meta name="description" content="Software engineer at AWS DSQL Storage. A personal workshop exploring distributed systems, robotics, and the things in between. Browse or explore in a terminal."/>
        <AppStateProvider>
            <Router>
                <Layout>
                    <Routes>
                        <Route path="/" view=HomePage/>
                        <Route path="/home" view=HomePage/>
                        <Route path="/about" view=AboutPage/>
                        <Route path="/terminal" view=TerminalPage/>
                        <Route path="/projects" view=ProjectsPage/>
                        <Route path="/projects/:slug" view=ProjectPage/>
                        <Route path="/blog" view=BlogPage/>
                        <Route path="/blog/:slug" view=BlogPostPage/>
                        <Route path="/*any" view=NotFoundPage/>
                    </Routes>
                </Layout>
            </Router>
        </AppStateProvider>
    }
}
