# Skill: Smart Error Diagnostics

**Description:** Advanced protocol for debugging, log extraction, and root-cause analysis when build failures, runtime panics, or container crashes occur.
**Trigger:** Invoke this skill automatically whenever a terminal command fails (exit code != 0), a compiler error/warning appears, or the user reports a bug in the application/Docker.

## Execution Steps:

1. **Context Extraction (Do not guess):**
   - **For Rust/Cargo errors:** Run `cargo check` or `cargo build` and extract the exact error trace.
   - **For Docker/Container crashes:** Run `docker-compose logs --tail=100 <failing_service>` to extract runtime logs.
   - **For Vue 3/Vite errors:** Read the terminal output of the frontend build process.
2. **Root Cause Analysis:** Analyze the extracted logs. Identify the exact file name, line number, or configuration mismatch (e.g., missing environment variables, database connection refused, ownership issues).
3. **Structured Reporting:** Before applying any fix, you MUST output a brief diagnostic report to the user in this format:
   - **🔴 Error Source:** (e.g., Backend / Database / Frontend)
   - **🔍 Root Cause:** (A clear, concise explanation of WHY it failed)
   - **🛠️ Proposed Fix:** (What files/code you plan to change to fix it)
4. **Execution & Verification:** Once the fix is applied, re-run the exact command that originally failed to ensure the issue is completely resolved (following the Strict Testing Rule).
