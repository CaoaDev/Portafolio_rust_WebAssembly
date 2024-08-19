use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[island]
pub fn Contact() -> impl IntoView {
    // let (message, set_message) = create_signal(false);
    // let form_ref = create_node_ref::<html::Form>();
    //
    // let handle_submit = move |ev: ev::SubmitEvent| {
    //     ev.prevent_default();
    //     set_message.set(true);
    //     log("Form submitted");
    //     if let Some(form) = form_ref.get() {
    //         form.reset();
    //     }
    // };

    view! {
        <section id="contact">
            <h5>"Get In Touch"</h5>
            <h2>"Contact Me"</h2>
            <div class="container contact__container">
                <div class="contact__options">
                    <article class="contact__option">
                        <h4>"Email"</h4>
                        <h5>"greenkat087@gmail.com"</h5>
                        <a href="mailto:greenkat087@gmail.com">"Send a message"</a>
                    </article>
                </div>
                // <form
                //     on:submit=handle_submit
                //     node_ref=form_ref
                // >
                //     <input
                //         type="text"
                //         placeholder="Your Full Name"
                //         name="user_name"
                //         required
                //     />
                //     <input
                //         type="text"
                //         placeholder="Your Email"
                //         name="user_email"
                //         required
                //     />
                //     <textarea
                //         placeholder="Your message"
                //         rows="7"
                //         name="message"
                //         required
                //     ></textarea>
                //     <button type="submit" class="btn btn-primary">
                //         "Send Message"
                //     </button>
                //     {move || message.get().then(|| view! { <span>"Thank you, I will get in touch as soon as possible :), best regards from me!"</span> })}
                // </form>
            </div>
        </section>
    }
}
