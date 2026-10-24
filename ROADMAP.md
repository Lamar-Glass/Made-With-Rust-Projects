# 🦀 The Rust Roadmap — from Borrow Checker to AI Engines

> **The plan:** ~20 projects, 5 phases. Rust as a *second* language: you already program, so every project targets the things Rust does differently — ownership, traits, `Result`, zero-cost abstraction — and ends in the high ground: **AI systems that are too fast for Python.**
>
> **The rule:** *One project at a time. Let the compiler teach you. Ship everything.*

Assumes **6–10 hours per week** alongside the Python roadmap. See [Adjusting the pace](#-adjusting-the-pace).

---

## 📊 Phase Overview

| Phase | Theme | Duration | Projects | You'll be able to… |
|:-----:|-------|:--------:|:--------:|--------------------|
| 1 | Foundations | Weeks 1–4 | 5 | Write small tools without fighting ownership |
| 2 | CLI mastery | Weeks 5–10 | 4 | Build & publish tested CLI tools |
| 3 | Async & web | Weeks 11–16 | 4 | Design and deploy real services |
| 4 | Rust × AI | Months 5–7 | 6 | Run, accelerate, and serve models in Rust |
| 5 | Mastery | Year 2+ | 5 | Build engines, contribute to OSS |

**Parallel reading:** [The Rust Book](https://doc.rust-lang.org/book/) — chapters map to phases below. Also great: [Rustlings](https://github.com/rust-lang/rustlings) exercises alongside Phase 1.

---

## Phase 1 — Foundations: *Make peace with the compiler* *(weeks 1–4)*

**Mindset:** tiny tools, 1–3 days each. The goal is **ownership intuition**, not cleverness. Read every compiler error twice — it's the best teacher in programming.

**Book alignment:** chapters 1–10 (ownership, structs, enums, `Result`, generics, traits).

| # | Project | You build | Core lesson | ⭐ Stretch |
|:-:|---------|-----------|-------------|-----------|
| 01 | **Temperature Converter** | `temp 32f 0c` conversions, CLI args | Types, `match`, `Result`, first error handling | Accept `98.6F → C` freeform input |
| 02 | **Minigrep Clone** | The book's grep, then *extended* — `-i`, `-n`, `-v` flags | Ownership, borrowing, lifetimes — the whole war in one project | Colored output with `owo-colors` |
| 03 | **Todo CLI** | JSON-file-backed todo with `add/list/done` | `serde` (derive), modules, `?` operator | Pretty table output |
| 04 | **File Organizer** | Sort a messy folder by extension/date | `std::fs`, `Path`, `walkdir`, `fs::rename` | Dry-run mode; undo file |
| 05 | **Markdown → HTML** | Parse basic markdown, emit HTML | Iterators, `Result` chains, string handling | Headings, lists, code blocks, links |

**✅ Phase checkpoint:** explain *why* `String` vs `&str` exists and when each is right — and write a function taking a reference without cloning.

---

## Phase 2 — CLI Mastery: *Tools you actually use* *(weeks 5–10)*

**Mindset:** 1–2 weeks per project, real quality bar: tests, docs (`--help` that reads well), releases. **CLI apps are Rust's home turf** — this phase ends with tools in your daily workflow.

**Book alignment:** chapters 11–13 (testing, closures, iterators) + [Rust CLI book](https://rust-cli.github.io/book/).

| # | Project | You build | Core lesson | ⭐ Stretch |
|:-:|---------|-----------|-------------|-----------|
| 06 | **Expense Tracker** | SQLite-backed tracker, monthly reports | `rusqlite`, `struct`s ↔ rows, `chrono` | CSV export; a bar chart in the terminal |
| 07 | **HTTP CLI Client** | A tiny `curl`-ish tool: method, headers, JSON body | `reqwest`, `serde_json`, `clap` derive | Timeouts, retries, pretty-print responses |
| 08 | **jq-lite** | Query + transform JSON from stdin | Recursive data types, pattern matching | Filters like `.users[].name` |
| 09 | **TUI Pomodoro** | Terminal pomodoro timer with stats | `ratatui`, event loops, state machines | Persistent stats; keybindings |

**✅ Phase checkpoint:** `cargo publish` a tool (or cut a GitHub release with a binary), and a friend runs it on their machine. **Ship one tool you personally keep using.**

---

## Phase 3 — Async & Web: *Services, not scripts* *(weeks 11–16)*

**Mindset:** async Rust is a different mental model — you're now building **the backend**, not calling it. 2 weeks per project.

**Book alignment:** [Tokio tutorial](https://tokio.rs/tokio/tutorial), [Zero To Production In Rust](https://www.zero2prod.com/) (free online) — the best resource for this phase.

| # | Project | You build | Core lesson | ⭐ Stretch |
|:-:|---------|-----------|-------------|-----------|
| 10 | **URL Shortener API** | Axum service: shorten, redirect, click stats | Routes, extractors, state sharing (`Arc`) | SQLx + Postgres; deploy on Fly.io |
| 11 | **JSON REST API** | CRUD resource API with validation + tests | SQLx (compile-checked queries!), middleware | Auth with JWT middleware |
| 12 | **Async Web Scraper** | Crawl many pages concurrently, respect `robots.txt`, rate-limit | `tokio`, `futures`, semaphores, streams | Crawl 10k pages; benchmark throughput |
| 13 | **gRPC Service** | A tonic service (e.g. the scraper, remote-controlled) | Protobuf, streaming RPC | Load-test it; report p99 latency |

**✅ Phase checkpoint:** explain what a `Future` is, why Rust async needs a runtime, and how `Arc<Mutex<T>>` shares state across tasks — over a service you deployed.

---

## Phase 4 — Rust × AI: *Where the repos meet* *(months 5–7)*

**Mindset:** this is why you learned Rust. Python owns training; **Rust owns serving and speed.** These projects make you dangerous: bilingual AI engineers who can optimize what others merely use.

| # | Project | You build | Core lesson | ⭐ Stretch |
|:-:|---------|-----------|-------------|-----------|
| 14 | **PyO3 Speed-Up** | Take a slow Python function (tokenizer, embedding loop, image filter) → rewrite in Rust, import it back | PyO3, releasing the GIL, benchmarking honestly (`criterion`) | 10×+ speed-up measured, not vibes |
| 15 | **Fast Log Processor** | Parse multi-GB log files with `nom` + `rayon`, answer queries in seconds | Parallelism, parsers, memory layout | Bench: Rust vs pandas on the same file |
| 16 | **LLM Inference with candle** | Run a small open model (e.g. a 1–3B LLM or Whisper) locally via Hugging Face `candle` | Loading weights, inference loops, f32 vs quantized | Token streaming; benchmark tokens/sec |
| 17 | **Embeddings Server** | Axum service that embeds text via `candle` or a local model, with a batch endpoint | Serving models behind HTTP; queues; concurrency | Plus cosine-similarity search endpoint |
| 18 | **Train a Tiny Model with burn** | Train a small NN (MNIST-classifier) using the `burn` framework in pure Rust | Autograd in Rust, dataset pipelines | Export weights; serve via project 17 |
| 19 | **WASM Inference in the Browser** | Compile a tiny classifier/inference to WebAssembly, run client-side | `wasm-bindgen`, target differences | Compare latency vs server round-trip |

**✅ Phase checkpoint:** run an open-weight model entirely in Rust, explain the trade-offs vs Python (vLLM/transformers), and show a benchmark where Rust won.

---

## Phase 5 — Mastery *(year 2+)*

**Mindset:** build engines, not apps. Contribute to the tools you use.

| # | Project | You build | Why it matters |
|:-:|---------|-----------|----------------|
| 20 | **Mini-GPT in Pure Rust** | Tensor ops with `ndarray` + a from-scratch transformer — the Rust twin of Python Phase 6's nanoGPT | You'll understand every layer, twice over |
| 21 | **Quantized Inference Engine** | Load a GGUF model; implement k-quant inference + KV cache | Deep systems + ML knowledge — rare and paid |
| 22 | **MCP Server in Rust** | Ship a Model Context Protocol server for an LLM tool | The AI-tooling ecosystem is being written now |
| 23 | **OSS Contributions** | PRs to `candle`, `burn`, `tokenizers`, or `rig` | Working in real production Rust codebases |
| 24 | **Your Own Crate** | Extract a useful piece of your projects into a published, documented crate | API design; the craft of making things reusable |

**✅ Mastery checkpoint:** read unfamiliar Rust codebase (e.g. candle internals) and follow it — then improve it.

---

## 🗓️ Weekly Rhythm (suggested, alongside Python)

| Day | Focus |
|-----|-------|
| Mon–Tue | Python roadmap |
| Wed–Thu | Rust roadmap |
| Fri | **Ship both**: commits, READMEs, retros |
| Sat | Study (book chapter matching next week's problem) |
| Sun | Rest |

*Prefer focusing? Alternate: 2 weeks Python, 2 weeks Rust. Momentum beats balance.*

---

## 🐢 Adjusting the Pace

- **Rust feeling slow?** It is — for everyone, for months. That's normal, not a sign to quit.
- **Overwhelmed by two languages?** Finish Python Phases 1–2 first, then start this roadmap from the beginning.
- **Stuck on lifetimes?** Write the relationship in a comment, then annotate. If stuck > 1 day: `.clone()` it, ship, and revisit the borrow later.

---

## 📚 Curated Free Resources

| When | Resource |
|------|----------|
| Always open | [The Rust Book](https://doc.rust-lang.org/book/) |
| Phase 1 | [Rustlings](https://github.com/rust-lang/rustlings), [Rust by Example](https://doc.rust-lang.org/rust-by-example/) |
| Phase 2 | [Rust CLI Book](https://rust-cli.github.io/book/), [clap docs](https://docs.rs/clap/) |
| Phase 3 | [Tokio Tutorial](https://tokio.rs/tokio/tutorial), [Zero2Prod](https://www.zero2prod.com/), [Axum examples](https://github.com/tokio-rs/axum/tree/main/examples) |
| Phase 4 | [candle](https://github.com/huggingface/candle), [burn](https://burn.dev/), [PyO3 guide](https://pyo3.rs/) |
| Phase 5 | [Awesome Rust](https://github.com/rust-unofficial/awesome-rust), crate source code itself |

---

## 🏁 Progress Tracker

- [ ] Phase 1 — 5/5 shipped · *checkpoint passed*
- [ ] Phase 2 — 4/4 shipped · *checkpoint passed* · *a tool I use daily*
- [ ] Phase 3 — 4/4 shipped · *checkpoint passed* · *something deployed*
- [ ] Phase 4 — 6/6 shipped · *checkpoint passed* · *benchmarked faster than Python*
- [ ] Phase 5 — mastery capstones (ongoing)

*Update this file as you go. Future-you is the audience.*
