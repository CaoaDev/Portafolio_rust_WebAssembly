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
            {skill_item("HTML +3", "https://www.w3.org/", AiHtml5Outlined)}
            {skill_item("CSS/SCSS/SASS +2", "https://www.w3.org/Style/CSS/Overview.en.html", BsFiletypeCss)}
            {skill_item("JavaScript +2", "https://ecma-international.org/publications-and-standards/standards/ecma-262/", TbBrandJavascript)}
            {skill_item("React/Next +1", "https://reactjs.org/", BiReact)}
            {skill_item("Redux +1", "https://redux.js.org/", SiRedux)}
            {skill_item("JAVA +1", "https://www.java.com/", FaJavaBrands)}
            {skill_item("PHP +1", "https://www.php.net/", TbFileTypePhp)}
            {skill_item("Svelte +1", "https://svelte.dev/", SiSvelte)}
        </>
    }
}

fn backend_skills() -> impl IntoView {
    view! {
        <>
            {skill_item("Rust/WebAssembly +1", "https://www.rust-lang.org/", FaRustBrands)}
            {skill_item("Node/NestJs +3", "https://nodejs.org/", BiNodejs)}
            {skill_item("PostgreSQL +1", "https://www.postgresql.org/", SiPostgresql)}
            {skill_item("Swagger/Jira +2", "https://swagger.io/", SiSwagger)}
            {skill_item("SQL/NoSql +3", "https://www.sql.org/", BsFiletypeSql)}
            {skill_item("Git/GitHub +3", "https://github.com/", VsGithub)}
            {skill_item("Docker +1", "https://www.docker.com/", TbBrandDocker)}
            {skill_item("AWS +1", "https://aws.amazon.com/", FaAwsBrands)}
        </>
    }
}

fn skill_item(skill: &'static str, url: &'static str, icon: icondata::Icon) -> impl IntoView {
    view! {
        <article class="experience__details">
            <a href={url} target="_blank" class="experience__details-link">
                <Icon icon={icon} class="experience__details-icon" />
                <h4>{skill}</h4>
            </a>
        </article>
    }
}
