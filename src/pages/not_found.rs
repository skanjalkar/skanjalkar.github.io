use leptos::*;
use leptos_router::*;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <section class="page-heading">
            <p class="eyebrow">"404 / A WRONG TURN"</p>
            <h1>"Nothing on "<em>"this shelf."</em></h1>
            <p class="body-copy">"That page isn’t in the workshop. Let’s find you a way back."</p>
            <A href="/" class="button primary">"Back home →"</A>
        </section>
    }
}
