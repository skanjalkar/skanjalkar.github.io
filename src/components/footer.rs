use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="w-full py-8 text-center">
            <a 
                href="https://github.com/skanjalkar" 
                target="_blank" 
                class="text-white font-display font-bold no-underline hover:underline"
            >
                "Built with Rust & Leptos"
            </a>
        </footer>
    }
}