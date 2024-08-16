use icondata::{
    AiHtml5Outlined, BiNodejs, BiReact, BsFiletypeCss, BsFiletypeSql, FaAwsBrands, FaJavaBrands,
    FaRustBrands, SiPostgresql, SiRedux, SiSvelte, SiSwagger, TbBrandDocker, TbBrandJavascript,
    TbFileTypePhp, VsGithub,
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
                <a href="https://www.w3.org/" target="_blank" class="experience__details-link">
                <Icon icon=AiHtml5Outlined class="experience__details-icon" />
                    <h4>"HTML +3"</h4>
                </a>
            </article>

            <article class="experience__details">
                <a href="https://www.w3.org/Style/CSS/Overview.en.html" target="_blank" class="experience__details-link">
                <Icon icon=BsFiletypeCss class="experience__details-icon" />
                    <h4>"CSS/SCSS/SASS +2"</h4>
                </a>
            </article>

            <article class="experience__details">
                <a href="https://ecma-international.org/publications-and-standards/standards/ecma-262/" target="_blank" class="experience__details-link">
                <Icon icon=TbBrandJavascript class="experience__details-icon" />
                    <h4>"JavaScript +2"</h4>
                </a>
            </article>

            <article class="experience__details">
                <a href="https://reactjs.org/" target="_blank" class="experience__details-link">
                <Icon icon=BiReact class="experience__details-icon" />
            <h4>"React/Next +1"</h4>
                </a>
            </article>

            <article class="experience__details">
                <a href="https://redux.js.org/" target="_blank" class="experience__details-link">
                    <Icon icon=SiRedux class="experience__details-icon" />
                    <h4>"Redux +1"</h4>
                </a>
            </article>

            <article class="experience__details">
                <a href="https://www.java.com/" target="_blank" class="experience__details-link">
                    <Icon icon=FaJavaBrands class="experience__details-icon" />
                    <h4>"JAVA +1"</h4>
                </a>
            </article>

            <article class="experience__details">
                <a href="https://www.php.net/" target="_blank" class="experience__details-link">
                    <Icon icon=TbFileTypePhp class="experience__details-icon" />
                    <h4>"PHP +1"</h4>
                </a>
            </article>

            <article class="experience__details">
                <a href="https://svelte.dev/" target="_blank" class="experience__details-link">
                    <Icon icon=SiSvelte class="experience__details-icon" />
                    <h4>"Svelte +1"</h4>
                </a>
            </article>
        </>
    }
}

fn backend_skills() -> impl IntoView {
    view! {
        <>
            <article class="experience__details">
                <a href="https://www.rust-lang.org/" target="_blank" class="experience__details-link">
                    <Icon icon=FaRustBrands class="experience__details-icon" />
                    <h4>"Rust/WebAssembly +1"</h4>
                </a>
            </article>

            <article class="experience__details">
                <a href="https://nodejs.org/" target="_blank" class="experience__details-link">
                    <Icon icon=BiNodejs class="experience__details-icon" />
                    <h4>"Node/NestJs +3"</h4>
                </a>
            </article>

            <article class="experience__details">
                <a href="https://www.postgresql.org/" target="_blank" class="experience__details-link">
                    <Icon icon=SiPostgresql class="experience__details-icon" />
                    <h4>"PostgreSQL +1"</h4>
                </a>
            </article>

            <article class="experience__details">
                <a href="https://swagger.io/" target="_blank" class="experience__details-link">
                    <Icon icon=SiSwagger class="experience__details-icon" />
                    <h4>"Swagger/Jira +2"</h4>
                </a>
            </article>

            <article class="experience__details">
                <a href="https://www.sql.org/" target="_blank" class="experience__details-link">
                    <Icon icon=BsFiletypeSql class="experience__details-icon" />
                    <h4>"SQL/NoSql +3"</h4>
                </a>
            </article>

            <article class="experience__details">
                <a href="https://github.com/" target="_blank" class="experience__details-link">
                    <Icon icon=VsGithub class="experience__details-icon" />
                    <h4>"Git/GitHub +3"</h4>
                </a>
            </article>

            <article class="experience__details">
                <a href="https://www.docker.com/" target="_blank" class="experience__details-link">
                    <Icon icon=TbBrandDocker class="experience__details-icon" />
                    <h4>"Docker +1"</h4>
                </a>
            </article>

            <article class="experience__details">
                <a href="https://aws.amazon.com/" target="_blank" class="experience__details-link">
                    <Icon icon=FaAwsBrands class="experience__details-icon" />
                    <h4>"AWS +1"</h4>
                </a>
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
