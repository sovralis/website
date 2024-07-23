#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[layout(Nav)]
    #[route("/")]
    Home {},
    #[route("/litepaper")]
    Litepaper {},
    #[route("/dashboard")]
    Dashboard {},
    #[route("/nexus")]
    Nexus {},
    #[route("/channels")]
    Channels {},
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
            Link { to: Route::Litepaper {}, "LITEPAPER" }
            Link { to: Route::Dashboard {}, "DASHBOARD" }
       }

        Outlet::<Route> {}
    }
}

#[component]
fn Channels() -> Element {
    rsx! {
        div {
            h1 { "Channels: The Scalability Solution of Sovralis" }
            p { "Channels in Sovralis are an advanced implementation of the Lightning Network concept, optimized for multi-party interactions and complex state management." }
            
            h2 { "Understanding Lightning Channels" }
            p { "Lightning channels, originally designed for Bitcoin, allow two parties to conduct multiple transactions off-chain, only settling the final balance on the blockchain. This significantly reduces transaction fees and increases throughput." }
            
            h2 { "Sovralis Channel Optimizations" }
            ul {
                li { "Multi-Party Support: Unlike traditional two-party channels, Sovralis channels can involve multiple participants." }
                li { "Complex State Management: Beyond simple value transfers, our channels can manage complex application states." }
                li { "Cross-Chain Compatibility: Channels can operate across different blockchain networks." }
            }
            
            h2 { "The Killer Feature: Minimal Blockchain Interaction" }
            p { "The most significant advantage of Sovralis channels is that blockchain interaction is only required when disputes need to be resolved. This design choice leads to:" }
            ul {
                li { "Dramatically Increased Scalability: Millions of transactions can occur off-chain." }
                li { "Reduced Costs: Fewer on-chain transactions mean lower fees." }
                li { "Near-Instant Finality: Off-chain transactions are almost instantaneous." }
                li { "Web2-Level Performance: The system can handle the same transaction volume as centralized Web2 systems." }
            }
            
            h2 { "How It Works" }
            ol {
                li { "Participants open a channel by locking funds in a smart contract." }
                li { "Transactions and state changes occur off-chain, signed by all relevant parties." }
                li { "The latest agreed-upon state is always ready to be submitted to the blockchain." }
                li { "If a dispute arises, the most recent valid state is submitted to the blockchain for resolution." }
                li { "When all parties agree, the channel can be closed, settling the final state on-chain." }
            }
            
            p { "By leveraging these advanced channel mechanisms, Sovralis achieves the scalability of Web2 systems while maintaining the security and decentralization benefits of blockchain technology." }
        }
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

            h2 { "Welcome to Sovralis" }
            p { "Sovralis is a realistic, down-to-earth approach to building the foundation layer of web3, solving scalability issues present in other designs." }
            h2 { "Key Features:" }
            ul {
                li { "Ethical Development: Built by anonymous builders in the true spirit of web3." }
                li { "Fair Launch: Tokens are mined by running foundational network layers, similar to Bitcoin's philosophy." }
                li { "Scalable Coordination Layer: Revolutionizing decentralized coordination." }
                li { "Grassroots Approach: Built from the ground up, ensuring true decentralization." }
            }
            p { "Join us in creating a scalable, ethical foundation for the future of decentralized networks!" }
        }
    }
}


#[component]
fn Litepaper() -> Element {
    rsx! {
        div {
            Logo {}
            h2 { "Introduction" }
            p { "Sovralis is a revolutionary project that aims to redefine the landscape of Web3 by introducing a set of powerful primitives. Our vision is to create a more efficient, scalable, and interoperable decentralized ecosystem." }
            
            h2 { "Core Layers" }
            
            h3 { "1. Nexus Layer" }
            p { "The foundation of Sovralis is our innovative Nexus Layer. This layer facilitates seamless interaction and synchronization between different blockchain networks and off-chain systems. It acts as a decentralized orchestrator, enabling complex multi-chain operations and enhancing overall network efficiency. " }
            Link { to: Route::Nexus {}, "Learn more about the Nexus Layer" }
            
            h3 { "2. Channels" }
            p { "Building upon the concept of payment channels, Sovralis introduces advanced multi-party channels. These channels can be thought of as 'Lightning Network on steroids', allowing for:"}
            ul {
                li { "Instant, low-cost transactions between multiple parties" }
                li { "Complex state management beyond simple value transfers" }
                li { "Cross-chain interoperability" }
                li { "Scalable off-chain computation with on-chain security guarantees" }
            }
            Link { to: Route::Channels {}, "Learn more about Channels" }
            
            h3 { "3. Blockchain Arbitration" }
            p { "The third layer of Sovralis is not a specific blockchain, but rather a protocol that allows any existing blockchain to function as an arbiter. This approach ensures:"}
            ul {
                li { "Maximum flexibility and future-proofing" }
                li { "Ability to leverage the security and features of established blockchains" }
                li { "True interoperability across the entire blockchain ecosystem" }
            }
            
            h2 { "Key Features and Benefits" }
            ul {
                li { "Unprecedented Scalability: By moving most operations off-chain, Sovralis can handle millions of transactions per second." }
                li { "Interoperability: Seamless interaction between different blockchain networks and traditional systems." }
                li { "Cost-Efficiency: Dramatically reduced transaction costs compared to on-chain operations." }
                li { "Privacy: Enhanced transaction privacy through off-chain channels." }
                li { "Flexibility: Adaptable to various use cases, from simple payments to complex smart contract executions." }
            }
            
            h2 { "Conclusion" }
            p { "Sovralis represents a paradigm shift in the Web3 space. By introducing these powerful primitives, we're laying the groundwork for a more efficient, scalable, and interconnected decentralized future. Join us in revolutionizing the blockchain landscape and unlocking the true potential of Web3." }
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
fn Nexus() -> Element {
    rsx! {
        div {
            h1 { "Nexus Layer: The Backbone of Sovralis" }
            p { "The Nexus Layer is the essential connecting infrastructure of Sovralis, designed to ensure secure, private, and efficient communication across the entire network." }
            
            h2 { "Key Features" }
            ul {
                li { 
                    strong { "Onion-based Routing: " }
                    "Protects user location and identity, ensuring anonymity in all network interactions."
                }
                li { 
                    strong { "Incentivized Network: " }
                    "Node operators earn rewards for running and maintaining the network, ensuring its robustness and decentralization."
                }
                li { 
                    strong { "Public Key Addressing: " }
                    "Every participant is reachable via their unique public key address, simplifying secure communication."
                }
                li { 
                    strong { "End-to-End Encryption: " }
                    "All communication on the Nexus Layer is end-to-end encrypted by default, guaranteeing privacy and security."
                }
                li { 
                    strong { "Technology Agnostic: " }
                    "The Nexus Layer is designed to support any technology or protocol, making it a versatile foundation for diverse applications."
                }
            }
            
            h2 { "How It Works" }
            p { "The Nexus Layer acts as a decentralized coordination mechanism, allowing parties to find each other and establish secure connections. Here's a brief overview of its operation:" }
            ol {
                li { "When a user joins the network, they generate a unique public-private key pair." }
                li { "The public key serves as the user's address on the network." }
                li { "To communicate, users leverage the onion-routing protocol, which encrypts messages in layers and routes them through multiple nodes." }
                li { "Each node in the route only knows the previous and next hop, preserving anonymity." }
                li { "The Nexus Layer facilitates the discovery of routes and establishment of connections between parties." }
                li { "Once a connection is established, end-to-end encryption ensures that only the intended recipients can decrypt the messages." }
            }
            
            h2 { "Benefits" }
            ul {
                li { "Enhanced Privacy: Protects user identity and location." }
                li { "Increased Security: End-to-end encryption by default." }
                li { "Scalability: Designed to support millions of users and devices." }
                li { "Interoperability: Enables communication between different blockchain networks and off-chain systems." }
                li { "Flexibility: Supports a wide range of applications and use cases." }
            }
            
            p { "The Nexus Layer is the cornerstone of Sovralis, providing the secure and private infrastructure necessary for the next generation of decentralized applications and services." }
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
