<div align="center">



<br/>



# 🛡️ sudo-privesc-rust



### *Local privilege escalation awareness for Linux — `sudo`, `find`, and misconfiguration hunting*



[![Rust](https://img.shields.io/badge/engine-Rust-DEA584?logo=rust&logoColor=white)](https://www.rust-lang.org/)

[![CLI](https://img.shields.io/badge/CLI-clap-9cf?logo=gnu-bash&logoColor=white)](https://github.com/clap-rs/clap)

[![License](https://img.shields.io/badge/license-TBD-lightgrey)](#license)



**⚡ Fast · 🧭 Structured · 📋 Audit-ready · 🔒 Authorized use only**



<br/>



[Overview](#overview) · [Features](#features) · [Install](#install) · [Usage](#usage) · [Sample report](#sample-report-json) · [Methodology](#security-assessment-methodology) · [Docs](#documentation) · [Ethics](#ethics--disclaimer)



<br/>



</div>



---



## 🎯 Overview



**sudo-privesc-rust** is a small, focused **open-source security utility** for Linux environments. It inspects the effective **`sudo`** policy for the current user (`sudo -l`) and flags a classic high-risk pattern: **`NOPASSWD`** access to **`/usr/bin/find`**, which can enable **local privilege escalation** via documented **`find -exec`** primitives (see [**GTFOBins**](https://gtfobins.github.io/)).



Use it for **defensive validation**, **labs**, **CTF write-ups**, and **portfolio-grade reporting** — not for unauthorized access.



---



## ✨ Features



<table>

<tr>

<td width="50%" valign="top">



### ⚡ High-Performance Rust Engine



Zero-cost abstractions where it matters: the scanner stays **lightweight**, **single-binary friendly**, and suitable for **repeatable audits** on real systems (always in authorized contexts).



### 🧭 Professional CLI (Clap)



**`clap`** with derive macros: rigid **`--check` / `--exploit`** modes, **`--json`** artifact output, **`--help`** / **`--version`**, and behavior that reads like a **shipping CLI**, not a throwaway script.



</td>

<td width="50%" valign="top">



### 📄 Structured JSON Audit Reports



Optional **`--json`** emits a **pretty-printed** **`audit_results.json`** — machine-readable, diff-friendly, and easy to attach to coursework or internal tickets.



### 🕐 Timestamped Logging



Every JSON report carries an **RFC 3339 UTC timestamp** (**chrono**), plus **`severity`**, **`target_os`**, and structured **`sudo_fetch`** / **`exploit`** blocks so reviewers see **what was measured and when**.



</td>

</tr>

</table>



<p align="center"><sub>🛡️ <b>Defensive default:</b> <code>--check</code> never executes the demonstration. <code>--exploit</code> requires explicit confirmation.</sub></p>



---



## 📦 Install



**Requirements:** Linux (or WSL / VM with realistic `sudo`). [Rust toolchain](https://www.rust-lang.org/tools/install) (stable).



```bash

git clone <repository-url>

cd sudo-privesc-rust

cargo build --release

```



Binary: **`target/release/sudo-privesc-rust`** (on Windows builds: `sudo-privesc-rust.exe` — Linux/WSL recommended for meaningful results).



---



## 🖥️ Usage



### Help



```text

Sudo NOPASSWD + `find` misconfiguration checker (research / authorized use only)



Usage: sudo-privesc-rust [OPTIONS] <--check|--exploit>



Options:

      --check    Only check `sudo -l` and report if the risky pattern is present (no exploit)

      --exploit  If vulnerable, prompt for confirmation, then attempt the demonstration escalation

      --json     Write results to `audit_results.json` instead of printing to the terminal

  -h, --help     Print help

  -V, --version  Print version

```



### Commands



| Goal | Command |

|------|---------|

| Scan only (human-readable output) | `cargo run --release -- --check` |

| Scan + write JSON audit file | `cargo run --release -- --check --json` |

| Interactive demo path (authorized labs only) | `cargo run --release -- --exploit` |

| Demo path + JSON artifact | `cargo run --release -- --exploit --json` |



After a successful **`--json`** run:



```text

[+] Security audit completed. Report saved to audit_results.json

```



---



## 📋 Sample report JSON



Illustrative output (fields match the tool’s schema; `sudo_l_stdout_preview` is truncated in real runs):



```json

{

  "tool": "sudo-privesc-rust",

  "timestamp": "2026-04-04T14:22:01+00:00",

  "severity": "High",

  "target_os": "Linux",

  "mode": "check",

  "sudo_fetch": {

    "ok": true,

    "sudo_l_stdout_preview": "User alice may run the following commands on lab-host:\n    (ALL) NOPASSWD: /usr/bin/find\n"

  },

  "vulnerability_detected": true,

  "exploit": null

}

```



When no pattern matches, expect **`"severity": "Informational"`** and **`"vulnerability_detected": false`**. If `sudo -l` cannot be retrieved, **`severity`** reflects an **`Error`**-class outcome and **`sudo_fetch.ok`** is **`false`**.



---



## 🔬 Security Assessment Methodology



### STRIDE



We map this scenario using **STRIDE** in [`THREAT_MODEL.md`](THREAT_MODEL.md). The dominant concern is **Elevation of Privilege**: a standard user leverages **over-broad `sudo`** to reach **root-equivalent execution**. Other STRIDE categories inform **root cause** (e.g. unsafe **`sudoers`** change management) and **detection** strategy.



### Least Privilege



**Least privilege** means granting the **minimum** commands and arguments required — never **`NOPASSWD`** on general utilities like **`find`** unless you have accepted the full risk. Operational hardening guidance (including **`visudo`**) lives in [`REMEDIATION.md`](REMEDIATION.md).



---



## 📎 Why `find`? (GTFOBins)



Under **`sudo`**, **`find`**’s **`-exec`** can launch arbitrary programs with elevated identity. That is why **`NOPASSWD: /usr/bin/find`** is a **configuration smell** worth catching early.



---



## 📚 Documentation



| Asset | Role |

|--------|------|

| [`THREAT_MODEL.md`](THREAT_MODEL.md) | STRIDE-oriented model & attack surface |

| [`REMEDIATION.md`](REMEDIATION.md) | SysAdmin remediation & `visudo` workflow |

| `audit_results.json` | Structured output from **`--json`** |



---



## ⚖️ Ethics & Disclaimer



- **Authorized systems only** — labs you control, coursework VMs, or engagements with **written permission**.

- Tool provided **“as is”** for **education and research**. **No warranty.** **Not legal advice.**



---



## 🧪 Limitations



Heuristic detection (substring match on `sudo -l` output) — **not** a full **`sudoers`** parser. Always corroborate in your environment.



---



## 🔮 Roadmap



- Broader **`sudoers` pattern** coverage with lower false-positive noise  

- Richer reporting (exit codes, SARIF/HTML)  

- Guided **remediation checklists** post-findings  



---



## 📜 License



Add a `LICENSE` file before wide redistribution (MIT, Apache-2.0, or your institution’s choice).



---



<div align="center">



<br/>



**Stack:** Rust · clap · serde · serde_json · chrono  



*Ship safe defaults. Teach real defenders.*



<br/>



</div>