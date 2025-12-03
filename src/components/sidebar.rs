use leptos::*;
use leptos_router::*;
use crate::models::Profile;
use crate::components::{Icon, ThemeToggle};

#[component]
pub fn Sidebar() -> impl IntoView {
    let profile = Profile::default();
    
    view! {
        <aside class="sidebar" style="background: linear-gradient(90deg, rgba(10, 10, 10, 0.3), rgb(10, 10, 10, 1)), url('https://images.unsplash.com/photo-1553748024-d1b27fb3f960?w=1500&q=80') center;">
            <div class="flex flex-col gap-4">
                <div 
                    class="w-44 h-44 rounded-lg bg-cover bg-center bg-no-repeat"
                    style=format!("background-image: url('{}')", profile.avatar_url)
                />
                
                <div class="font-bold">
                    <span class="block text-2xl">{profile.name.clone()}</span>
                    <span class="flex items-center gap-2">
                        <Icon icon="github" />
                        <a href={profile.github_url.clone()} class="link" target="_blank">
                            "@"{profile.username.clone()}
                        </a>
                    </span>
                </div>
                
                <div class="text-2xl font-display">
                    {profile.bio.clone()}
                </div>
                
                <div class="flex flex-col gap-2 text-lg font-display">
                    <span class="flex items-center gap-2">
                        <Icon icon="mail" />
                        <a href=format!("mailto:{}", profile.email) class="link">
                            "E-mail"
                        </a>
                    </span>
                    
                    {profile.linkedin_url.clone().map(|url| view! {
                        <span class="flex items-center gap-2">
                            <Icon icon="linkedin" />
                            <a href={url} class="link" target="_blank">"LinkedIn"</a>
                        </span>
                    })}
                    
                    <span class="flex items-center gap-2">
                        <Icon icon="link" />
                        <a href="https://skanjalkar.github.io" class="link">"skanjalkar.github.io"</a>
                    </span>
                    
                    {profile.resume_url.clone().map(|url| view! {
                        <span class="flex items-center gap-2">
                            <Icon icon="file" />
                            <a href={url} class="link" target="_blank">"Résumé"</a>
                        </span>
                    })}
                    
                    <span class="flex items-center gap-2">
                        <Icon icon="map" />
                        <a 
                            href="https://www.google.com/maps/d/viewer?mid=11GruDkekjiIo1GEeitRkvQty-gw&hl=en_US&ll=33.77622619709335%2C-84.39699350000001&z=16" 
                            class="link" 
                            target="_blank"
                        >
                            {profile.location.clone()}
                        </a>
                    </span>
                </div>
                
                <nav class="mt-4 flex flex-col gap-2">
                    <A href="/" class="link hover:text-gray-300">"Home"</A>
                    <A href="/projects" class="link hover:text-gray-300">"Projects"</A>
                    <A href="/blog" class="link hover:text-gray-300">"Blog"</A>
                </nav>
                
                <ThemeToggle />
            </div>
        </aside>
    }
}