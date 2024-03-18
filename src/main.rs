use leptos::{component, prelude::*, view, IntoView};

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! {
        <p>"from rust"</p>
        <CounterButton/>
    });

}

#[component]
fn CounterButton() -> impl IntoView {
    let count = create_rw_signal(0usize);

    let increment_count = 
    move |_| {
        count.update(|count| *count += 1)
    };

    let is_count_even = 
    move || {
        count.get() % 2 == 0
    };

    view! {
        <button
            on:click=increment_count
            class:red=is_count_even
        >
            {count}
        </button>
    }
}
