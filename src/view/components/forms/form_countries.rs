use leptos::{ev::MouseEvent, *};

#[component]
pub fn FormCountries(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    let click_handler = {
        move |_: MouseEvent| {
            set_count.update(|value| *value += 1);
        }
    };

    view! { cx,
        <>
            <h2 class="text-4xl font-extrabold dark:text-white">"Total cliks: "{count}</h2>

            <br />

            <button on:click=click_handler class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">"Click me"</button>

            <br /><br />

            // https://github.com/tailwindlabs/tailwindcss-forms

            <form action="">
                <input type="email" class="form-input px-4 py-3 rounded-full" />

                <select class="form-select pl-4 pr-9 py-3 rounded-full">
                    <option selected>"Choose a country"</option>
                    <option value="US">"United States"</option>
                    <option value="CA">"Canada"</option>
                    <option value="FR">"France"</option>
                    <option value="DE">"Germany"</option>
                </select>

                <input type="checkbox" class="form-checkbox rounded text-pink-500" />
            </form>
        </>
    }
}
