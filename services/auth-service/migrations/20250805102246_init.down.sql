-- Enhanced down migration script

-- Drop cleanup functions
DROP FUNCTION IF EXISTS cleanup_expired_tokens;
DROP FUNCTION IF EXISTS cleanup_old_audit_logs;
DROP FUNCTION IF EXISTS cleanup_old_login_attempts;
DROP FUNCTION IF EXISTS update_updated_at_column;

-- Drop triggers
DROP TRIGGER IF EXISTS update_users_updated_at ON users;
DROP TRIGGER IF EXISTS update_security_questions_updated_at ON security_questions;
DROP TRIGGER IF EXISTS update_user_security_questions_updated_at ON user_security_questions;

-- Drop GIN indexes
DROP INDEX IF EXISTS idx_audit_logs_metadata_gin;
DROP INDEX IF EXISTS idx_audit_logs_old_values_gin;
DROP INDEX IF EXISTS idx_audit_logs_new_values_gin;
DROP INDEX IF EXISTS idx_users_email_trgm;

-- Drop composite indexes
DROP INDEX IF EXISTS idx_users_role_verified_active;
DROP INDEX IF EXISTS idx_refresh_tokens_user_expires_revoked;
DROP INDEX IF EXISTS idx_user_sessions_user_active_expires;

-- Drop btree indexes
DROP INDEX IF EXISTS idx_users_created_at_btree;
DROP INDEX IF EXISTS idx_refresh_tokens_created_at_btree;
DROP INDEX IF EXISTS idx_user_sessions_created_at_btree;

-- Drop optimized indexes
DROP INDEX IF EXISTS idx_users_email_lower;
DROP INDEX IF EXISTS idx_users_role_active;
DROP INDEX IF EXISTS idx_users_is_verified_active;
DROP INDEX IF EXISTS idx_users_locked_until;
DROP INDEX IF EXISTS idx_users_last_login;

DROP INDEX IF EXISTS idx_refresh_tokens_device_user;
DROP INDEX IF EXISTS idx_login_attempts_identifier_time;
DROP INDEX IF EXISTS idx_login_attempts_ip_time;
DROP INDEX IF EXISTS idx_login_attempts_success_time;
DROP INDEX IF EXISTS idx_login_attempts_country_city;

DROP INDEX IF EXISTS idx_user_sessions_activity;
DROP INDEX IF EXISTS idx_user_sessions_device_user;
DROP INDEX IF EXISTS idx_blacklisted_tokens_user_type;
DROP INDEX IF EXISTS idx_user_permissions_expires_at;
DROP INDEX IF EXISTS idx_audit_logs_action_time;
DROP INDEX IF EXISTS idx_audit_logs_resource;
DROP INDEX IF EXISTS idx_audit_logs_ip_time;

-- Drop standard indexes
DROP INDEX IF EXISTS idx_audit_logs_created_at;
DROP INDEX IF EXISTS idx_audit_logs_action;
DROP INDEX IF EXISTS idx_audit_logs_user_id;

DROP INDEX IF EXISTS idx_blacklisted_tokens_expires_at;
DROP INDEX IF EXISTS idx_blacklisted_tokens_token_hash;

DROP INDEX IF EXISTS idx_user_sessions_expires_at;
DROP INDEX IF EXISTS idx_user_sessions_session_token;
DROP INDEX IF EXISTS idx_user_sessions_user_id;
DROP INDEX IF EXISTS idx_user_sessions_token;

DROP INDEX IF EXISTS idx_user_security_questions_user_id;
DROP INDEX IF EXISTS idx_user_security_questions_question_id;
DROP INDEX IF EXISTS idx_security_questions_created_by;
DROP INDEX IF EXISTS idx_security_questions_active;

DROP INDEX IF EXISTS idx_login_attempts_created_at;
DROP INDEX IF EXISTS idx_login_attempts_ip_address;
DROP INDEX IF EXISTS idx_login_attempts_email;

DROP INDEX IF EXISTS idx_password_reset_tokens_expires_at;
DROP INDEX IF EXISTS idx_password_reset_tokens_token_hash;
DROP INDEX IF EXISTS idx_password_reset_tokens_user_id;

DROP INDEX IF EXISTS idx_refresh_tokens_expires_at;
DROP INDEX IF EXISTS idx_refresh_tokens_token_hash;
DROP INDEX IF EXISTS idx_refresh_tokens_user_id;

DROP INDEX IF EXISTS idx_users_is_active;
DROP INDEX IF EXISTS idx_users_role;
DROP INDEX IF EXISTS idx_users_phone_number;
DROP INDEX IF EXISTS idx_users_email;
DROP INDEX IF EXISTS idx_user_permissions_user_id;
DROP INDEX IF EXISTS idx_user_permissions_permission;

-- Drop partitioned tables first (child tables)
DROP TABLE IF EXISTS login_attempts_2025_01;
DROP TABLE IF EXISTS login_attempts_2025_02;
DROP TABLE IF EXISTS login_attempts_2025_03;
DROP TABLE IF EXISTS login_attempts_2025_04;
DROP TABLE IF EXISTS login_attempts_2025_05;
DROP TABLE IF EXISTS login_attempts_2025_06;
DROP TABLE IF EXISTS login_attempts_2025_07;
DROP TABLE IF EXISTS login_attempts_2025_08;
DROP TABLE IF EXISTS login_attempts_2025_09;
DROP TABLE IF EXISTS login_attempts_2025_10;
DROP TABLE IF EXISTS login_attempts_2025_11;
DROP TABLE IF EXISTS login_attempts_2025_12;

DROP TABLE IF EXISTS audit_logs_2025_01;
DROP TABLE IF EXISTS audit_logs_2025_02;
DROP TABLE IF EXISTS audit_logs_2025_03;
DROP TABLE IF EXISTS audit_logs_2025_04;
DROP TABLE IF EXISTS audit_logs_2025_05;
DROP TABLE IF EXISTS audit_logs_2025_06;
DROP TABLE IF EXISTS audit_logs_2025_07;
DROP TABLE IF EXISTS audit_logs_2025_08;
DROP TABLE IF EXISTS audit_logs_2025_09;
DROP TABLE IF EXISTS audit_logs_2025_10;
DROP TABLE IF EXISTS audit_logs_2025_11;
DROP TABLE IF EXISTS audit_logs_2025_12;

-- Drop tables in reverse dependency order
DROP TABLE IF EXISTS audit_logs;
DROP TABLE IF EXISTS user_permissions;
DROP TABLE IF EXISTS blacklisted_tokens;
DROP TABLE IF EXISTS user_sessions;
DROP TABLE IF EXISTS user_security_questions;
DROP TABLE IF EXISTS security_questions;
DROP TABLE IF EXISTS login_attempts;
DROP TABLE IF EXISTS password_reset_tokens;
DROP TABLE IF EXISTS refresh_tokens;
DROP TABLE IF EXISTS users;

-- Drop extensions (only if not used by other tables)
DROP EXTENSION IF EXISTS pg_trgm;
DROP EXTENSION IF EXISTS btree_gin;