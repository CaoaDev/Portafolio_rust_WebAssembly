use leptos::*;

use crate::components::*;

// use crate::components::contact::contact::Contact;
// use crate::components::experience::experience::Experience;
// use crate::components::footer::footer::Footer;
// use crate::components::header::header::Header;
// use crate::components::intro::intro::Intro;
// use crate::components::portafolio::portafolio::Portfolio;
// use crate::components::topbar::topbar::Topbar;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}
                </ul>
            }
        }>

            <div class="container">
                <Header />
                <Topbar />
                <Intro />
                <Experience />
                <Portfolio />
                // <Testimonials />
                <Contact />
                <Footer />
            </div>
        </ErrorBoundary>
    }
}
