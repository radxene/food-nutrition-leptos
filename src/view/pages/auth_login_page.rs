use leptos::*;

#[component]
pub fn AuthLoginPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="min-h-screen bg-gray-100 dark:bg-dark-bg py-6 flex flex-col justify-center sm:py-12">
            <div class="absolute font-mono top-5 left-5 block rounded-lg bg-white border border-gray-400 p-6 dark:bg-neutral-700 shadow-lg">
                <p class="mb-2 text-brand-500"><span class="text-black mr-2 select-none">"Email:"</span>"barrow@occurs.rs"</p>
                <p class="text-brand-500"><span class="text-black mr-2 select-none">"Passw:"</span>"resolved"</p>
            </div>
            <div class="relative py-3 sm:max-w-xl sm:mx-auto">
                <div
                    class="absolute inset-0 bg-gradient-to-r from-[#d0465a] to-brand-500 shadow-lg transform -skew-y-6 sm:skew-y-0 sm:-rotate-6 sm:rounded-3xl">
                </div>
                <div class="relative px-4 py-10 bg-white shadow-lg sm:rounded-3xl sm:p-20">
                    <div class="max-w-md mx-auto">
                        <div>
                            <h1 class="text-2xl font-semibold">"Login in to your account"</h1>
                        </div>
                        <div class="divide-y divide-gray-200">
                            <div class="py-8 text-base leading-6 space-y-8 text-gray-700 sm:text-lg sm:leading-7">
                                <div class="relative">
                                    <input autocomplete="off" id="email" name="email" type="text" class="peer placeholder-transparent h-10 w-full border-b-2 border-gray-300 text-gray-900 focus:outline-none focus:borer-rose-600" placeholder="Email address" />
                                    <label for="email" class="absolute left-0 -top-3.5 text-gray-600 text-sm peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-440 peer-placeholder-shown:top-2 transition-all peer-focus:-top-3.5 peer-focus:text-gray-600 peer-focus:text-sm">"Email Address"</label>
                                </div>
                                <div class="relative">
                                    <input autocomplete="off" id="password" name="password" type="password" class="peer placeholder-transparent h-10 w-full border-b-2 border-gray-300 text-gray-900 focus:outline-none focus:borer-rose-600" placeholder="Password" />
                                    <label for="password" class="absolute left-0 -top-3.5 text-gray-600 text-sm peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-440 peer-placeholder-shown:top-2 transition-all peer-focus:-top-3.5 peer-focus:text-gray-600 peer-focus:text-sm">"Password"</label>
                                </div>
                                <div class="flex justify-end">
                                    <button class="bg-brand-500 text-white rounded-md px-4 py-1.5 text-md">"Submit"</button>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
