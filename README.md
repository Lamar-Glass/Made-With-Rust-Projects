<div align="center">

# 🦀 Made With Rust

**A living portfolio — systems, tools, and AI projects built while learning Rust.**

![Rust](https://img.shields.io/badge/Rust-stable%20%2F%20nightly-DEA584?style=flat-square&logo=rust&logoColor=black)
[![Roadmap](https://img.shields.io/badge/Roadmap-Rust_×_AI-8A2BE2?style=flat-square)](ROADMAP.md)
[![License: GPL v3](https://img.shields.io/badge/License-GPL_v3-blue?style=flat-square)](LICENSE)

*Performance, safety, and zero fear of the borrow checker.*

</div>

---

## 📖 About

This repo is my project-based journey through Rust — starting from tiny CLI tools and climbing to the place where Rust shines today: **high-performance AI systems, inference engines, and tools too fast for Python.**

The plan lives in [ROADMAP.md](ROADMAP.md). This README is the scoreboard.

## 🗺️ The Path (summary)

| Phase | Theme | Rough timeline | Flagship outcome |
|:-----:|-------|:---------------:|-------------------|
| **1** | Foundations — *fight the borrow checker and win* | Weeks 1–4 | Small CLIs without clone()ing everything |
| **2** | CLI mastery — *tools you use daily* | Weeks 5–10 | Published, tested CLI tools |
| **3** | Async & web — *services, not scripts* | Weeks 11–16 | Production-ready Axum APIs |
| **4** | Rust × AI — *where the two repos meet* | Months 5–7 | Inference in Rust; Python accelerated by Rust |
| **5** | Mastery | Year 2+ | Engines and OSS contributions |

👉 **Full schedule, acceptance criteria, and stretch goals: [ROADMAP.md](ROADMAP.md)**

## ✅ Project Catalog

Each project lives in its own numbered folder. Status legend: ⬜ planned · 🚧 building · ✅ shipped

| # | Project | Phase | Core concepts | Status |
|:-:|---------|:-----:|---------------|:------:|
| 01 | [Temperature Converter](01-temp-converter/) | 1 | types, `match`, error handling | ⬜ |
| 02 | [Minigrep Clone](02-minigrep/) | 1 | ownership, borrowing, lifetimes | ⬜ |
| 03 | [Todo CLI](03-todo-cli/) | 1 | `serde` + JSON, modules | ⬜ |
| 04 | [File Organizer](04-file-organizer/) | 1 | `std::fs`, `Path`, `walkdir` | ⬜ |
| 05 | [Markdown → HTML](05-md2html/) | 1 | iterators, `Result` chains | ⬜ |

*Later projects get added here as I reach each phase — the full list lives in the [ROADMAP](ROADMAP.md).*

## 📁 How Every Project Is Structured

```
NN-project-name/
├── README.md         # started BEFORE coding, finished after shipping
├── Cargo.toml        # dependencies & metadata
└── src/main.rs       # (src/lib.rs + src/bin/ when it grows)
```

New project? `cargo new NN-name`, then `cp docs/project-template.md NN-name/README.md`.

## ▶️ Running a Project

```bash
cd NN-project-name
cargo run              # build + run
cargo test             # run the tests
cargo build --release  # the fast version
```

## 🧠 My Learning Rules

1. **One project at a time.** Finish, ship, then start the next.
2. **Read the compiler errors.** Rust's errors are the tutorial — fight the urge to cargo-check-and-pray.
3. **No `.clone()` without a reason.** Every clone is a question: could a reference do?
4. **Ship publicly.** Commit, README, share.
5. **Stuck > 45 min?** Write the borrow problem down in words — it usually dissolves.

## 🤝 Companion Repo

The Python side of this journey: [Made-With-Python-Projects](https://github.com/Lamar-Glass/Made-With-Python-Projects) — the two roads meet in Rust Roadmap Phase 4 (*Rust × AI*).

## 📄 License

Everything here is released under the [GPL-3.0 License](LICENSE) — learn from it, fork it, build on it.
