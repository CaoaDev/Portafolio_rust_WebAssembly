use leptos::*;

use crate::islands::*;

// use crate::islands::contact::contact::Contact;
// use crate::islands::experience::experience::Experience;
// use crate::islands::footer::footer::Footer;
// use crate::islands::header::header::Header;
// use crate::islands::intro::intro::Intro;
// use crate::islands::portafolio::portafolio::Portfolio;
// use crate::islands::topbar::topbar::Topbar;

/// Default Home Page
#[island]
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
