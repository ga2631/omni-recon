# Skill: Database Migration Manager

**Description:** Standard protocol for managing PostgreSQL database schema changes using `sqlx-cli`.
**Trigger:** Invoke this skill whenever the user asks to modify the database schema, add tables, change columns, or setup the Medallion Silver/Gold layers.

## Execution Steps:

1. **Tool Verification:** Ensure `sqlx-cli` is available. If not, inform the user or use the Terminal to install it (`cargo install sqlx-cli --no-default-features --features native-tls,postgres`).
2. **Create Migration Files:** DO NOT execute raw SQL commands directly to modify the schema. Use the Terminal to run:
   `sqlx migrate add -r <migration_name>`
   _(Replace `<migration_name>` with a descriptive name, e.g., `create_unified_orders_table`)._
3. **Write SQL:**
   - Locate the newly generated `<timestamp>_<migration_name>.up.sql` and write the PostgreSQL-compliant DDL script to apply the changes.
   - Locate the corresponding `.down.sql` file and write the exact reverse DDL script to drop or revert the changes.
4. **Apply Migration:** Execute `sqlx migrate run` via Terminal to apply the changes to the local development database.
5. **Verify:** Check if the database schema is updated correctly. If there are errors, automatically trigger the **Smart Error Diagnostics** skill.
