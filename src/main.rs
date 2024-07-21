#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/about")]
    About {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        div {
            Router::<Route> {}
        }
    }
}

#[component]
fn About() -> Element {
    rsx! {
        div {
            h1 { "Welcome to Sovralis" }
            p { "Sovralis is the vanguard of web3, a groundbreaking project launched from the grassroots." }
            h2 { "Key Features:" }
            ul {
                li { "Coordination Layer: Our first release focuses on revolutionizing decentralized coordination." }
                li { "Anonymous and Grassroots: Built from the ground up, as it should be." }
                li { "Fair Token Distribution: No token sales - all tokens are mined through network participation." }
            }
            p { "Join us in shaping the future of decentralized networks!" }
        }
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div {
            h1 { "Sovralis: The Future of Web3" }
            p { "Welcome to Sovralis, the revolutionary coordination layer for the decentralized web." }
            Link { to: Route::About {}, "Learn More" }
        }
    }
}

#[component]
fn Dashboard() -> Element {
    rsx! {
        div {
            h1 { "Dashboard" }
        }
    }

}
