use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="fixed bottom-4 right-4 z-40">
            <a
                href="https://github.com/leptos-rs/leptos"
                target="_blank"
                class="text-gray-500 hover:text-white text-sm font-display transition-colors duration-200"
            >
                "Built with Rust & Leptos 🦀"
            </a>
        </footer>
    }
}