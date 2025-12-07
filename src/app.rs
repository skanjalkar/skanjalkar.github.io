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
                <Routes>
                    <Route path="/" view=TerminalPage/>
                    <Route path="/home" view=|| view! { <Layout><HomePage/></Layout> }/>
                    <Route path="/projects" view=|| view! { <Layout><ProjectsPage/></Layout> }/>
                    <Route path="/blog" view=|| view! { <Layout><BlogPage/></Layout> }/>
                    <Route path="/blog/:slug" view=|| view! { <Layout><BlogPostPage/></Layout> }/>
                    <Route path="/*any" view=|| view! { <Layout><NotFoundPage/></Layout> }/>
                </Routes>
            </Router>
        </AppStateProvider>
    }
}
