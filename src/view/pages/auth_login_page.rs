use leptos::*;
use leptos_router::*;
use web_sys::{Event, MouseEvent};

use crate::{
    models::entities::{SotrageAuthUser, SotrageAuthUserData},
    routes::AppRoute,
    utils::validators::AuthValidatorUtil,
};

#[component]
pub fn AuthLoginPage(cx: Scope) -> impl IntoView {
    let (email_value, set_email_value) = create_signal(cx, "".to_owned());
    let (password_value, set_password_value) = create_signal(cx, "".to_owned());

    let navigate = use_navigate(cx);

    let change_email_handler = move |ev: Event| {
        set_email_value(event_target_value(&ev));
    };

    let change_password_handler = move |ev: Event| {
        set_password_value(event_target_value(&ev));
    };

    let click_button_handler = move |_: MouseEvent| {
        let email = email_value();
        let password = password_value();

        if AuthValidatorUtil::validate_storage_user_data(&email, &password) {
            SotrageAuthUser::login(email, password);
            navigate(AppRoute::Root.path(), NavigateOptions::default()).unwrap();
        }
    };

    view! { cx,
        <div class="min-h-screen bg-gray-100 dark:bg-dark-bg py-6 flex flex-col justify-center sm:py-12">
            <div class="absolute font-mono top-5 left-5 block p-6 rounded-lg bg-white border border-gray-400 dark:border-gray-700 dark:bg-dark-vague-2 shadow-lg">
                <p class="mb-2 text-brand-500 dark:text-gray-200">
                    <span class="text-black dark:text-gray-400 mr-2 select-none">"Email:"</span>
                    {SotrageAuthUserData::Email.to_string()}
                </p>
                <p class="text-brand-500 dark:text-gray-200">
                    <span class="text-black dark:text-gray-400 mr-2 select-none">"Passw:"</span>
                    {SotrageAuthUserData::Password.to_string()}
                </p>
            </div>
            <div class="relative py-3 sm:max-w-xl sm:mx-auto">
                <div class="absolute inset-0 bg-gradient-to-r from-[#d0465a] to-brand-500 shadow-lg transform -skew-y-6 sm:skew-y-0 sm:-rotate-6 sm:rounded-3xl mx-4 sm:mx-0" />
                <div class="relative px-4 py-10 bg-white dark:bg-dark-vague-2 shadow-lg sm:rounded-3xl sm:p-20 mx-4 sm:mx-0">
                    <div class="max-w-md mx-auto">
                        <div>
                            <h1 class="text-2xl dark:text-gray-200 font-semibold">"Login in to your account"</h1>
                        </div>
                        <div class="divide-y divide-gray-200">
                            <div class="py-8 text-base leading-6 space-y-8 text-gray-700 sm:text-lg sm:leading-7">
                                <div class="relative">
                                    <input on:input=change_email_handler autocomplete="off" id="email" name="email" type="text" class="peer placeholder-transparent h-10 w-full border-b-2 border-gray-300 text-gray-900 dark:text-gray-100 dark:bg-dark-vague-2 focus:outline-none focus:borer-rose-600" placeholder="Email address" />
                                    <label for="email" class="absolute left-0 -top-3.5 text-gray-600 dark:text-gray-400 text-sm peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-440 peer-placeholder-shown:top-2 transition-all peer-focus:-top-3.5 peer-focus:text-gray-600 dark:peer-focus:text-gray-400 peer-focus:text-sm">"Email Address"</label>
                                </div>
                                <div class="relative">
                                    <input on:input=change_password_handler autocomplete="off" id="password" name="password" type="password" class="peer placeholder-transparent h-10 w-full border-b-2 border-gray-300 text-gray-900 dark:text-gray-100 dark:bg-dark-vague-2 focus:outline-none focus:borer-rose-600" placeholder="Password" />
                                    <label for="password" class="absolute left-0 -top-3.5 text-gray-600 dark:text-gray-400 text-sm peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-440 peer-placeholder-shown:top-2 transition-all peer-focus:-top-3.5 peer-focus:text-gray-600 dark:peer-focus:text-gray-400 peer-focus:text-sm">"Password"</label>
                                </div>
                                <div class="flex justify-end">
                                    <button on:click=click_button_handler class="bg-brand-500 text-white dark:text-gray-200 rounded-md px-4 py-1.5 text-md">"Submit"</button>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
