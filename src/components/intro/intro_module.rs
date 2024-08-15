use icondata::{RiMedal2BusinessLine, VsProject};
use leptos::*;
use leptos_icons::Icon;

#[component]
pub fn Intro() -> impl IntoView {
    view! {
        <section id="about">
            <h5>"Get to know"</h5>
            <h2>"About Me"</h2>
            <div class="container about__container">
                <div class="about__me">
                    <div class="about__me-image">
                        <img src="assets/yo.png" alt="me"/>
                    </div>
                </div>
                <div class="about__content">
                    <div class="about__cards">
                        <article class="about__card">
                            <a href="#experience">
                                <Icon icon=RiMedal2BusinessLine class="about__icon" />
                                <h5>"Experience"</h5>
                                <small>"1+ year"</small>
                            </a>
                        </article>
                        <article class="about__card">
                            <a href="#portfolio">
                                <Icon icon=VsProject class="about__icon" />
                                <h5>"Projects"</h5>
                                <small>"20+ Completed Projects"</small>
                            </a>
                        </article>
                    </div>
                    <p class="intro-text">
                        "I am a Computer Systems Engineer with a strong background in both backend and frontend development. I also have experience in preventive and corrective PC support and have conducted training sessions on computer science and systems. Holding a master’s degree in Telecommunications, I am noted for my problem-solving skills. My expertise spans operating systems such as Windows, GNU/Linux (with over 10 years of experience), and macOS. I possess strong communication skills and a self-taught approach to problem-solving."
                    </p>
                    <a href="#contact" class="btn btn-primary">"Let's Talk"</a>
                </div>
            </div>
        </section>
    }
}

#[component]
fn FaAward() -> impl IntoView {
    view! { <span>"Award Icon"</span> }
}

#[component]
fn VscFolderLibrary() -> impl IntoView {
    view! { <span>"Folder Library Icon"</span> }
}
