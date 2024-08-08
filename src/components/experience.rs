use leptos::*;

#[component]
pub fn Experience() -> impl IntoView {
    view! {
        <section id="experience">
            <h5>"The Skills I Have"</h5>
            <h2>"Skills"</h2>
            <div class="container experience__container">
                <div class="experience__frontend">
                    <h3>"Front-end Development"</h3>
                    <div class="experience__content">
                        {frontend_skills()}
                    </div>
                </div>
                <div class="experience__backend">
                    <h3>"Back-end Development"</h3>
                    <div class="experience__content">
                        {backend_skills()}
                    </div>
                </div>
            </div>
        </section>
    }
}

fn frontend_skills() -> impl IntoView {
    view! {
        <>
            {skill_item("HTML +3")}
            {skill_item("CSS +2")}
            {skill_item("JavaScript +2")}
            {skill_item("React/Next +1")}
            {skill_item("Redux +1")}
            {skill_item("SCSS +1")}
            {skill_item("Tailwind 1")}
            {skill_item("Svelte +1")}
        </>
    }
}

fn backend_skills() -> impl IntoView {
    view! {
        <>
            {skill_item("Rust/WebAssembly +1")}
            {skill_item("Node/NestJs +3")}
            {skill_item("PostgreSQL +1")}
            {skill_item("Swagger/Jira +2")}
            {skill_item("SQL/NoSql +3")}
            {skill_item("Git/GitHub +3")}
            {skill_item("Docker +1")}
            {skill_item("AWS +1")}
        </>
    }
}

fn skill_item(skill: &'static str) -> impl IntoView {
    view! {
        <article class="experience__details">
            <BsFillPatchCheckFill class="experience__details-icon"/>
            <h4>{skill}</h4>
        </article>
    }
}

// You'll need to implement this icon component or use an appropriate Leptos-compatible icon library
#[component]
fn BsFillPatchCheckFill(#[prop(into, optional)] class: Option<String>) -> impl IntoView {
    let class = class.unwrap_or_default();
    view! {
        <svg class={class} viewBox="0 0 16 16" fill="currentColor" height="1em" width="1em">
            <path d="M10.067.87a2.89 2.89 0 0 0-4.134 0l-.622.638-.89-.011a2.89 2.89 0 0 0-2.924 2.924l.01.89-.636.622a2.89 2.89 0 0 0 0 4.134l.637.622-.011.89a2.89 2.89 0 0 0 2.924 2.924l.89-.01.622.636a2.89 2.89 0 0 0 4.134 0l.622-.637.89.011a2.89 2.89 0 0 0 2.924-2.924l-.01-.89.636-.622a2.89 2.89 0 0 0 0-4.134l-.637-.622.011-.89a2.89 2.89 0 0 0-2.924-2.924l-.89.01-.622-.636zm.287 5.984-3 3a.5.5 0 0 1-.708 0l-1.5-1.5a.5.5 0 1 1 .708-.708L7 8.793l2.646-2.647a.5.5 0 0 1 .708.708z"/>
        </svg>
    }
}
