# Comprehensive Testing & Validation Rule

Whenever you (Antigravity Agent) complete writing or modifying code for a task, you MUST thoroughly test both the Backend and Frontend in BOTH Development (`dev`) and Production (`prod`) environments before documenting or concluding the task.

Strictly enforce these testing constraints:

1. **Development Environment (Dev):**
   - **Backend:** Execute dev-level checks and tests (e.g., `cargo check`, `cargo test`, or relevant test runners). Ensure all unit, integration, and ETL logic tests pass.
   - **Frontend:** Verify the local dev build (e.g., `npm run dev` / `yarn dev`). Ensure components render correctly without state management issues.
2. **Production Environment (Prod):**
   - **Backend:** Validate the release build (e.g., `cargo build --release`).
   - **Frontend:** Execute the production build process (e.g., `npm run build`).
   - **Infrastructure:** If applicable, run the containerized setup (e.g., `docker-compose`) to verify reverse proxy routing, environment variables, and database connections work flawlessly in a production-like state.
3. **Zero Tolerance for Errors & Warnings:**
   - Your testing MUST cover all possible edge cases, including missing data, malformed inputs, and unexpected user flows.
   - You MUST resolve ALL compiler warnings, linter warnings, console errors, and build failures. The final output must be 100% clean.
4. **Halt on Failure:** If any test fails or any warning appears in either environment, you MUST fix the code and re-test the entire flow. Do not proceed to generate the Technical Documentation until the validation is completely green.
