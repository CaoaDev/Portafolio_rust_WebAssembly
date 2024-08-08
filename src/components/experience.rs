use icondata::{
    AiHtml5Outlined, BiAws, BiNodejs, BiReact, BiTailwindCss, BsFiletypeCss, BsFiletypeScss,
    BsFiletypeSql, FaRustBrands, SiPostgresql, SiRedux, SiSvelte, SiSwagger, TbBrandDocker,
    TbBrandJavascript, VsGithub,
}; // Verifica el import correcto
use leptos::*;
use leptos_icons::Icon; // Verifica el import correcto

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
            <article class="experience__details">
                <Icon icon=AiHtml5Outlined class="experience__details-icon" />
                <h4>"HTML +3"</h4>
            </article>
            <article class="experience__details">
                <Icon icon=BsFiletypeCss class="experience__details-icon" />
                <h4>"CSS +2"</h4>
            </article>
            <article class="experience__details">
                <Icon icon=TbBrandJavascript class="experience__details-icon" />
                <h4>"JavaScript +2"</h4>
            </article>
            <article class="experience__details">
                <Icon icon=BiReact class="experience__details-icon" />
                <h4>"React/Next +1"</h4>
            </article>
            <article class="experience__details">
                <Icon icon=SiRedux class="experience__details-icon" />
                <h4>"Redux +1"</h4>
            </article>
            <article class="experience__details">
                <Icon icon=BsFiletypeScss class="experience__details-icon" />
                <h4>"SCSS +1"</h4>
            </article>
            <article class="experience__details">
                <Icon icon=BiTailwindCss class="experience__details-icon" />
                <h4>"Tailwind 1"</h4>
            </article>
            <article class="experience__details">
                <Icon icon=SiSvelte class="experience__details-icon" />
                <h4>"Svelte +1"</h4>
            </article>
        </>
    }
}

fn backend_skills() -> impl IntoView {
    view! {
        <>
            <article class="experience__details">
                <Icon icon=FaRustBrands class="experience__details-icon" />
                <h4>"Rust/WebAssembly +1"</h4>
            </article>
            <article class="experience__details">
                <Icon icon=BiNodejs class="experience__details-icon" />
                <h4>"Node/NestJs +3"</h4>
            </article>
            <article class="experience__details">
                <Icon icon=SiPostgresql class="experience__details-icon" />
                <h4>"PostgreSQL +1"</h4>
            </article>
            <article class="experience__details">
                <Icon icon=SiSwagger class="experience__details-icon" />
                <h4>"Swagger/Jira +2"</h4>
            </article>
            <article class="experience__details">
                <Icon icon=BsFiletypeSql class="experience__details-icon" />
                <h4>"SQL/NoSql +3"</h4>
            </article>
            <article class="experience__details">
                <Icon icon=VsGithub class="experience__details-icon" />
                <h4>"Git/GitHub +3"</h4>
            </article>
            <article class="experience__details">
                <Icon icon=TbBrandDocker class="experience__details-icon" />
                <h4>"Docker +1"</h4>
            </article>
            <article class="experience__details">
                <Icon icon=BiAws class="experience__details-icon" />
                <h4>"AWS +1"</h4>
            </article>
        </>
    }
}

// fn skill_item(skill: &'static str) -> impl IntoView {
//     view! {
//         <article class="experience__details">
//             <BsFillPatchCheckFill class="experience__details-icon"/>
//             <h4>{skill}</h4>
//         </article>
//     }
// }

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
