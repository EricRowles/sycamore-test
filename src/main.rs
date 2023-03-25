#![allow(non_snake_case)]

use sycamore::prelude::*;


fn MainComponent<G: Html>(cx: Scope) -> View<G> {
    let state = create_signal(cx, 0);
    view! { cx,
        div {
            h1 { "Hello, world!" }
            button(on:click=|_| {
                state.set(*state.get() + 1);
            }) {
                "Click me!"
            }
            p {
                "Current count: " (state.get())
            }
        }
    }
}

fn main() {
    sycamore::render(|cx| 
        MainComponent(cx)
    );
}
