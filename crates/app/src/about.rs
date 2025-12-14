//! About module
//!
//! Contains the About page with technical details about the website's architecture.

use leptos::prelude::*;

/// External link component that opens in a new tab
#[component]
fn ExtLink(href: &'static str, children: Children) -> impl IntoView {
    view! {
      <a href=href target="_blank" rel="noopener noreferrer" class="text-blue-600 dark:text-blue-400 hover:underline">
        {children()}
      </a>
    }
}

/// Section heading component
#[component]
fn SectionHeading(children: Children) -> impl IntoView {
    view! { <h3 class="mt-8 mb-4 text-xl font-semibold text-slate-900 dark:text-slate-100">{children()}</h3> }
}

/// Prose paragraph component
#[component]
fn Prose(children: Children) -> impl IntoView {
    view! { <p class="mb-4 text-slate-700 dark:text-slate-300 leading-relaxed">{children()}</p> }
}

#[component]
pub fn About() -> impl IntoView {
    view! {
      <div class="mx-auto max-w-3xl py-6">
        <h2 class="mb-6 text-3xl font-bold text-slate-900 dark:text-slate-100">"About"</h2>

        <div class="rounded-lg bg-white dark:bg-slate-800 p-6 shadow-md">
          <Prose>
            "This website is a personal food tracking application. The source code is available on "
            <ExtLink href="https://github.com/lemmih-com/food">"GitHub"</ExtLink> "."
          </Prose>

          <SectionHeading>"Architecture"</SectionHeading>

          <Prose>
            "Both the server and the frontend are written entirely in Rust. The server compiles to "
            "WebAssembly and runs as a Cloudflare Worker, providing low-latency responses from edge "
            "locations worldwide. The frontend also compiles to WebAssembly, enabling the same Rust "
            "codebase to run seamlessly in the browser."
          </Prose>

          <Prose>
            "The application uses " <ExtLink href="https://leptos.dev/">"Leptos"</ExtLink>
            ", a full-stack Rust framework that supports both server-side rendering (SSR) and "
            "client-side rendering (CSR). When you request a page, the server renders the initial "
            "HTML and sends it to your browser. The browser then loads a WebAssembly bundle that "
            "hydrates the static HTML, attaching event handlers and making the page interactive. "
            "This approach provides fast initial page loads while maintaining a rich, responsive " "user experience."
          </Prose>

          <SectionHeading>"Infrastructure"</SectionHeading>

          <Prose>
            "The server runs on " <ExtLink href="https://workers.cloudflare.com/">"Cloudflare Workers"</ExtLink>
            ", a serverless platform that executes code at the edge. This eliminates cold starts "
            "typical of traditional serverless functions and ensures consistent performance "
            "regardless of user location. Data is stored in "
            <ExtLink href="https://developers.cloudflare.com/d1/">"Cloudflare D1"</ExtLink>
            ", a serverless SQLite database that runs alongside the Workers."
          </Prose>

          <SectionHeading>"Deployment"</SectionHeading>

          <Prose>
            "The website is deployed automatically via GitHub Actions. The "
            <code class="bg-slate-100 dark:bg-slate-700 px-1 rounded text-sm">"main"</code>
            " branch serves as the source of truth: every push triggers a CI pipeline that builds "
            "the application, runs tests, and deploys to production. Pull requests get their own "
            "preview environments, allowing changes to be tested in isolation before merging."
          </Prose>

          <SectionHeading>"Testing"</SectionHeading>

          <Prose>
            "End-to-end testing is automated using "
            <ExtLink href="https://github.com/stevepryde/thirtyfour">"thirtyfour"</ExtLink>
            ", a Rust WebDriver client. The test suite spins up a local instance of the "
            "application, drives a real browser through user flows, and verifies that the "
            "application behaves correctly. This ensures that changes don't break existing "
            "functionality and catches regressions before they reach production."
          </Prose>

          <SectionHeading>"Build System"</SectionHeading>

          <Prose>
            "The project uses " <ExtLink href="https://nixos.org/manual/nix/stable/">"Nix"</ExtLink>
            " for reproducible builds and development environments. Nix pins all dependencies to "
            "exact versions, ensuring that builds are deterministic and that the development "
            "environment matches production. The Rust toolchain version, WebAssembly optimization "
            "tools, and all other dependencies are specified in the Nix configuration, eliminating "
            "\"works on my machine\" issues."
          </Prose>
        </div>
      </div>
    }
}
