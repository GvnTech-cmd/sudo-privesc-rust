<div align="center">

<br/>

# 🛡️ sudo-privesc-rust

### *Local privilege escalation awareness for Linux — `sudo`, `find`, and misconfiguration hunting*

[![Rust](https://img.shields.io/badge/engine-Rust-DEA584?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![CLI](https://img.shields.io/badge/CLI-clap-9cf?logo=gnu-bash&logoColor=white)](https://github.com/clap-rs/clap)

**⚡ Fast · 🧭 Structured · 📋 Audit-ready · 🔒 Authorized use only**

<br/>

[Overview](#overview) · [Features](#features) · [Install](#install) · [Usage](#usage) · [Sample report](#sample-report-json) · [Docs](#documentation) · [Ethics](#ethics--disclaimer)

<br/>

</div>

---

## 🎯 Overview

**sudo-privesc-rust** is a focused security utility for Linux. It inspects `sudo -l` for high-risk patterns like `NOPASSWD: /usr/bin/find`, enabling local privilege escalation via **GTFOBins** methods.

---

## ✨ Features

<table>
<tr>
<td width="50%" valign="top">

### ⚡ High-Performance Rust
Lightweight, single-binary scanner built with Rust for repeatable audits.

### 🧪 Automated Testing
Includes integration tests in the `tests/` directory to ensure logic accuracy. Run via `cargo test`.

</td>
<td width="50%" valign="top">

### 📄 Structured JSON Reports
Optional `--json` emits `audit_results.json` for professional audit artifacts.

### 🧭 Professional CLI
Rigid `--check` and `--exploit` modes powered by `clap` for a shipping-grade experience.

</td>
</tr>
</table>

---

## 📦 Install
```bash
git clone <repository-url>
cd sudo-privesc-rust
cargo build --release