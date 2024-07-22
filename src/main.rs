#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[layout(Nav)]
    #[route("/")]
    Home {},
    #[route("/about")]
    About {},
    #[route("/dashboard")]
    Dashboard {},
    #[end_layout]

    // Finally, we need to handle the 404 page
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}


#[component]
fn Nav() -> Element {
    rsx! {
       nav {
            class: "centered-nav",
            Link { to: Route::Home {}, "HOME" }
            Link { to: Route::About {}, "ABOUT" }
            Link { to: Route::Dashboard {}, "DASHBOARD" }
       }

        Outlet::<Route> {}
    }
}

#[component]
fn Logo() -> Element {
    rsx! {
        svg { 
            id: "logo",
            "viewBox": "0 0 800 160",
            "preserveAspectRatio": "xMidYMid meet",
            text { 
                x: "10",
                y: "20",
                fill: "#00ff00",
                "font-family": "monospace",
                "font-size": "20",
                tspan { x: "10", dy: "00", "░██████╗░█████╗░██╗░░░██╗██████╗░░█████╗░██╗░░░░░██╗░██████╗" }
                tspan { x: "10", dy: "20", "██╔════╝██╔══██╗██║░░░██║██╔══██╗██╔══██╗██║░░░░░██║██╔════╝" }
                tspan { x: "10", dy: "20", "╚█████╗░██║░░██║╚██╗░██╔╝██████╔╝███████║██║░░░░░██║╚█████╗░" }
                tspan { x: "10", dy: "20", "░╚═══██╗██║░░██║░╚████╔╝░██╔══██╗██╔══██║██║░░░░░██║░╚═══██╗" }
                tspan { x: "10", dy: "20", "██████╔╝╚█████╔╝░░╚██╔╝░░██║░░██║██║░░██║███████╗██║██████╔╝" }
                tspan { x: "10", dy: "20", "╚═════╝░░╚════╝░░░░╚═╝░░░╚═╝░░╚═╝╚═╝░░╚═╝╚══════╝╚═╝╚═════╝░" }
            }
            text {
                x: "400",
                y: "150",
                fill: "white",
                "font-family": "monospace",
                "font-size": "16",
                "text-anchor": "middle",
                "The Vanguard of Web3"
            }
        }
    }
}

#[component]
fn App() -> Element {
    rsx! {
        div {
            Router::<Route> {}
        }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div {
            Logo {}
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
fn About() -> Element {
    rsx! {
        div {
            h1 { "Sovralis: The Future of Web3" }
            p { "Welcome to Sovralis, the revolutionary coordination layer for the decentralized web." }
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

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}
