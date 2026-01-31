use leptos::*;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center min-h-[50vh] text-center px-4">
            <pre class="text-green-400 text-xs sm:text-sm font-mono mb-6 leading-tight animate-fade-in-down animate-float">
{r#"
    .---.        .-----------
   /     \  __  /    ------
  / /     \(  )/    -----
 //////   ' \/ `   ---
//// / // :    : ---
// /   /  /`    '--
//          //..\\
       ====googl.teleport" DOES NOT EXIST
           ////
          ////
          ===="#}
            </pre>
            <h1 class="text-6xl font-bold mb-4 bg-gradient-to-r from-purple-400 to-pink-600 bg-clip-text text-transparent animate-scale-in animate-pulse-glow">"404"</h1>
            <p class="text-xl text-gray-400 mb-2 animate-fade-in-up stagger-2">"Oops! This page wandered into the void."</p>
            <p class="text-sm text-gray-500 mb-8 font-mono animate-fade-in-up stagger-3">"Error: ENOENT - no such file or directory"</p>
            <div class="flex gap-4 flex-wrap justify-center animate-fade-in-up stagger-4">
                <a href="/" class="btn-primary hover-lift">"Take Me Home"</a>
                <a href="/terminal" class="px-6 py-3 border border-gray-600 rounded-lg hover:border-green-400 hover:text-green-400 transition-all duration-200 hover-lift">"Try the Terminal"</a>
            </div>
            <p class="text-xs text-gray-600 mt-8 font-mono animate-fade-in stagger-5">"Hint: The page you're looking for might be in another castle"</p>
        </div>
    }
}
