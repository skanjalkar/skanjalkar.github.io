use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div>
            <section id="about" class="mb-12">
                <h1 class="section-title">"About me"</h1>
                <p class="text-lg font-display leading-relaxed mb-4">
                    "My name is Shreyas Kanjalkar and that is my dog in my pfp. His name is Spiky. Cute right? "
                    "I am a Masters Student studying Computer Science at Georgia Institute of Technology (Gatech) in Atlanta, Georgia. "
                    "I have done masters in Robotics at Worcester Polytechnic Institute, WPI."
                </p>
                <p class="text-lg font-display leading-relaxed mb-4">
                    "I did my undergrad in Mechanical Engineering at Manipal Institute of Technology. During my time at WPI, "
                    "I have grown interest and affection towards Software Engineering. Now I wish to be able to work in the industry. "
                    "I am still learning about all there is to offer about Computer Science. My main interest is in cloud computing "
                    "and distributed systems, hoping to work in the industry on those topics."
                </p>
                <p class="text-lg font-display leading-relaxed">
                    "When I am not working, I enjoy watching and playing chess. I religiously follow Formula 1 and no, "
                    "I am not a \"Big 3\" fan. I am a McLaren life long fan. I am currently taking a break from Dota2 "
                    "and sometimes I click circles on osu!"
                </p>
            </section>
            
            <section id="featured-projects" class="mb-12">
                <h1 class="section-title">"Featured Projects"</h1>
                <p class="text-gray-400 font-display">
                    "Check out the "
                    <a href="/projects" class="link">"Projects page"</a>
                    " for all my work."
                </p>
            </section>
            
            <section id="recent-posts">
                <h1 class="section-title">"Recent Posts"</h1>
                <p class="text-gray-400 font-display">
                    "Visit the "
                    <a href="/blog" class="link">"Blog"</a>
                    " for all posts."
                </p>
            </section>
        </div>
    }
}