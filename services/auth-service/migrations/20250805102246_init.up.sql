-- Add up a migration script here
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    phone_number VARCHAR(20) UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(50) NOT NULL DEFAULT 'guest',
    is_verified BOOLEAN DEFAULT FALSE NOT NULL,
    is_active BOOLEAN DEFAULT TRUE NOT NULL,
    failed_login_attempts INTEGER DEFAULT 0 NOT NULL,
    locked_until TIMESTAMP WITH TIME ZONE,
    last_login_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (NOW() AT TIME ZONE 'UTC') NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT (NOW() AT TIME ZONE 'UTC') NOT NULL
);

-- Add role constraint for performance
ALTER TABLE users ADD CONSTRAINT users_role_check 
CHECK (role IN ('super_admin', 'admin', 'property_manager', 'tenant', 'landlord', 'maintenance', 'guest'));

-- Refresh tokens with partitioning preparation
CREATE TABLE refresh_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) NOT NULL,
    device_id VARCHAR(255),
    device_name VARCHAR(255),
    ip_address INET,
    user_agent TEXT,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    is_revoked BOOLEAN DEFAULT FALSE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (NOW() AT TIME ZONE 'UTC') NOT NULL,
    revoked_at TIMESTAMP WITH TIME ZONE
);

-- Password reset tokens with TTL-like behavior
CREATE TABLE password_reset_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    is_used BOOLEAN DEFAULT FALSE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (NOW() AT TIME ZONE 'UTC') NOT NULL,
    used_at TIMESTAMP WITH TIME ZONE
);

-- Login attempts with partitioning by month
CREATE TABLE login_attempts (
    id UUID DEFAULT gen_random_uuid(),
    identifier VARCHAR(255) NOT NULL,
    ip_address INET NOT NULL,
    user_agent TEXT,
    is_successful BOOLEAN NOT NULL,
    failure_reason VARCHAR(255),
    country VARCHAR(100),
    city VARCHAR(100),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (NOW() AT TIME ZONE 'UTC') NOT NULL,
    PRIMARY KEY (id, created_at)
) PARTITION BY RANGE (created_at);


-- Create monthly partitions for login_attempts (1 year of partitions)
CREATE TABLE login_attempts_2025_01 PARTITION OF login_attempts
FOR VALUES FROM ('2025-01-01') TO ('2025-02-01');

CREATE TABLE login_attempts_2025_02 PARTITION OF login_attempts
FOR VALUES FROM ('2025-02-01') TO ('2025-03-01');

CREATE TABLE login_attempts_2025_03 PARTITION OF login_attempts
FOR VALUES FROM ('2025-03-01') TO ('2025-04-01');

CREATE TABLE login_attempts_2025_04 PARTITION OF login_attempts
FOR VALUES FROM ('2025-04-01') TO ('2025-05-01');

CREATE TABLE login_attempts_2025_05 PARTITION OF login_attempts
FOR VALUES FROM ('2025-05-01') TO ('2025-06-01');

CREATE TABLE login_attempts_2025_06 PARTITION OF login_attempts
FOR VALUES FROM ('2025-06-01') TO ('2025-07-01');

CREATE TABLE login_attempts_2025_07 PARTITION OF login_attempts
FOR VALUES FROM ('2025-07-01') TO ('2025-08-01');

CREATE TABLE login_attempts_2025_08 PARTITION OF login_attempts
FOR VALUES FROM ('2025-08-01') TO ('2025-09-01');

CREATE TABLE login_attempts_2025_09 PARTITION OF login_attempts
FOR VALUES FROM ('2025-09-01') TO ('2025-10-01');

CREATE TABLE login_attempts_2025_10 PARTITION OF login_attempts
FOR VALUES FROM ('2025-10-01') TO ('2025-11-01');

CREATE TABLE login_attempts_2025_11 PARTITION OF login_attempts
FOR VALUES FROM ('2025-11-01') TO ('2025-12-01');

CREATE TABLE login_attempts_2025_12 PARTITION OF login_attempts
FOR VALUES FROM ('2025-12-01') TO ('2026-01-01');

-- Security questions
CREATE TABLE security_questions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    created_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    question VARCHAR(500) NOT NULL,
    is_active BOOLEAN DEFAULT TRUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (NOW() AT TIME ZONE 'UTC') NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT (NOW() AT TIME ZONE 'UTC') NOT NULL
);

-- User security questions
CREATE TABLE user_security_questions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    question_id UUID NOT NULL REFERENCES security_questions(id) ON DELETE CASCADE,
    answer_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (NOW() AT TIME ZONE 'UTC') NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT (NOW() AT TIME ZONE 'UTC') NOT NULL,
    UNIQUE(user_id, question_id)
);

-- User sessions with automatic cleanup
CREATE TABLE user_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    session_token VARCHAR(255) NOT NULL UNIQUE,
    device_id VARCHAR(255),
    device_name VARCHAR(255),
    ip_address INET,
    user_agent TEXT,
    is_active BOOLEAN DEFAULT TRUE NOT NULL,
    last_activity_at TIMESTAMP WITH TIME ZONE DEFAULT (NOW() AT TIME ZONE 'UTC') NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (NOW() AT TIME ZONE 'UTC') NOT NULL
);

-- Blocklisted tokens with TTL
CREATE TABLE blacklisted_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    token_hash VARCHAR(255) NOT NULL UNIQUE,
    token_type VARCHAR(50) NOT NULL,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    reason VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (NOW() AT TIME ZONE 'UTC') NOT NULL
);

-- User permissions with caching optimization
CREATE TABLE user_permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    permission VARCHAR(100) NOT NULL,
    granted_by UUID REFERENCES users(id),
    granted_at TIMESTAMP WITH TIME ZONE DEFAULT (NOW() AT TIME ZONE 'UTC') NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE,
    is_active BOOLEAN DEFAULT TRUE NOT NULL,
    UNIQUE(user_id, permission)
);

-- Audit logs with partitioning by month
CREATE TABLE audit_logs (
    id UUID DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    action VARCHAR(100) NOT NULL,
    resource_type VARCHAR(50),
    resource_id UUID,
    old_values JSONB,
    new_values JSONB,
    ip_address INET,
    user_agent TEXT,
    metadata JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (NOW() AT TIME ZONE 'UTC') NOT NULL,
    PRIMARY KEY (id, created_at)
) PARTITION BY RANGE (created_at);

-- Create monthly partitions for audit_logs
CREATE TABLE audit_logs_2025_01 PARTITION OF audit_logs
FOR VALUES FROM ('2025-01-01') TO ('2025-02-01');

CREATE TABLE audit_logs_2025_02 PARTITION OF audit_logs
FOR VALUES FROM ('2025-02-01') TO ('2025-03-01');

CREATE TABLE audit_logs_2025_03 PARTITION OF audit_logs
FOR VALUES FROM ('2025-03-01') TO ('2025-04-01');

CREATE TABLE audit_logs_2025_04 PARTITION OF audit_logs
FOR VALUES FROM ('2025-04-01') TO ('2025-05-01');

CREATE TABLE audit_logs_2025_05 PARTITION OF audit_logs
FOR VALUES FROM ('2025-05-01') TO ('2025-06-01');

CREATE TABLE audit_logs_2025_06 PARTITION OF audit_logs
FOR VALUES FROM ('2025-06-01') TO ('2025-07-01');

CREATE TABLE audit_logs_2025_07 PARTITION OF audit_logs
FOR VALUES FROM ('2025-07-01') TO ('2025-08-01');

CREATE TABLE audit_logs_2025_08 PARTITION OF audit_logs
FOR VALUES FROM ('2025-08-01') TO ('2025-09-01');

CREATE TABLE audit_logs_2025_09 PARTITION OF audit_logs
FOR VALUES FROM ('2025-09-01') TO ('2025-10-01');

CREATE TABLE audit_logs_2025_10 PARTITION OF audit_logs
FOR VALUES FROM ('2025-10-01') TO ('2025-11-01');

CREATE TABLE audit_logs_2025_11 PARTITION OF audit_logs
FOR VALUES FROM ('2025-11-01') TO ('2025-12-01');

CREATE TABLE audit_logs_2025_12 PARTITION OF audit_logs
FOR VALUES FROM ('2025-12-01') TO ('2026-01-01');

-- **PERFORMANCE INDEXES** --

-- **ENABLE REQUIRED EXTENSIONS** --
CREATE EXTENSION IF NOT EXISTS pg_trgm; -- For fuzzy text search
CREATE EXTENSION IF NOT EXISTS btree_gin; -- For better GIN indexes

-- Users' table indexes (optimized)
CREATE UNIQUE INDEX idx_users_email_lower ON users(LOWER(email)) WHERE is_active = true;
CREATE INDEX idx_users_phone_number ON users(phone_number) WHERE phone_number IS NOT NULL AND is_active = true;
CREATE INDEX idx_users_role_active ON users(role, is_active);
CREATE INDEX idx_users_is_verified_active ON users(is_verified, is_active);
CREATE INDEX idx_users_locked_until ON users(locked_until) WHERE locked_until IS NOT NULL;
CREATE INDEX idx_users_last_login ON users(last_login_at) WHERE last_login_at IS NOT NULL;

-- Refresh tokens indexes (optimized for cleanup)
CREATE INDEX idx_refresh_tokens_user_id ON refresh_tokens(user_id) WHERE is_revoked = false;
CREATE UNIQUE INDEX idx_refresh_tokens_token_hash ON refresh_tokens(token_hash);
CREATE INDEX idx_refresh_tokens_expires_at ON refresh_tokens(expires_at) WHERE is_revoked = false;
CREATE INDEX idx_refresh_tokens_device_user ON refresh_tokens(user_id, device_id) WHERE is_revoked = false;

-- Password reset tokens indexes
CREATE INDEX idx_password_reset_tokens_user_id ON password_reset_tokens(user_id) WHERE is_used = false;
CREATE UNIQUE INDEX idx_password_reset_tokens_token_hash ON password_reset_tokens(token_hash) WHERE is_used = false;
CREATE INDEX idx_password_reset_tokens_expires_at ON password_reset_tokens(expires_at) WHERE is_used = false;

-- Login attempts indexes (for rate limiting and monitoring)
CREATE INDEX idx_login_attempts_identifier_time ON login_attempts(identifier, created_at);
CREATE INDEX idx_login_attempts_ip_time ON login_attempts(ip_address, created_at);
CREATE INDEX idx_login_attempts_success_time ON login_attempts(is_successful, created_at);
CREATE INDEX idx_login_attempts_country_city ON login_attempts(country, city) WHERE country IS NOT NULL;

-- Security questions indexes
CREATE INDEX idx_security_questions_created_by ON security_questions(created_by) WHERE is_active = true;
CREATE INDEX idx_security_questions_active ON security_questions(is_active);

-- User security questions indexes
CREATE INDEX idx_user_security_questions_question_id ON user_security_questions(question_id);

-- User sessions indexes (optimized for active sessions)
CREATE INDEX idx_user_sessions_user_id ON user_sessions(user_id) WHERE is_active = true;
CREATE UNIQUE INDEX idx_user_sessions_token ON user_sessions(session_token);
CREATE INDEX idx_user_sessions_expires_at ON user_sessions(expires_at) WHERE is_active = true;
CREATE INDEX idx_user_sessions_activity ON user_sessions(last_activity_at) WHERE is_active = true;
CREATE INDEX idx_user_sessions_device_user ON user_sessions(user_id, device_id) WHERE is_active = true;

-- Blacklisted tokens indexes (optimized for security validation)
CREATE UNIQUE INDEX idx_blacklisted_tokens_token_hash ON blacklisted_tokens(token_hash);
CREATE INDEX idx_blacklisted_tokens_expires_at ON blacklisted_tokens(expires_at);
CREATE INDEX idx_blacklisted_tokens_user_type ON blacklisted_tokens(user_id, token_type) WHERE user_id IS NOT NULL;

-- User permissions indexes
CREATE INDEX idx_user_permissions_user_id ON user_permissions(user_id) WHERE is_active = true;
CREATE INDEX idx_user_permissions_permission ON user_permissions(permission) WHERE is_active = true;
CREATE INDEX idx_user_permissions_expires_at ON user_permissions(expires_at) WHERE expires_at IS NOT NULL AND is_active = true;

-- Audit logs indexes (optimized for queries)
CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id) WHERE user_id IS NOT NULL;
CREATE INDEX idx_audit_logs_action_time ON audit_logs(action, created_at);
CREATE INDEX idx_audit_logs_resource ON audit_logs(resource_type, resource_id) WHERE resource_type IS NOT NULL;
CREATE INDEX idx_audit_logs_ip_time ON audit_logs(ip_address, created_at) WHERE ip_address IS NOT NULL;

-- **GIN INDEXES FOR JSONB COLUMNS** --
CREATE INDEX idx_audit_logs_metadata_gin ON audit_logs USING GIN(metadata) WHERE metadata IS NOT NULL;
CREATE INDEX idx_audit_logs_old_values_gin ON audit_logs USING GIN(old_values) WHERE old_values IS NOT NULL;
CREATE INDEX idx_audit_logs_new_values_gin ON audit_logs USING GIN(new_values) WHERE new_values IS NOT NULL;

-- **FUNCTION-BASED INDEX FOR CASE-INSENSITIVE EMAIL SEARCHES** --
CREATE INDEX idx_users_email_trgm ON users USING GIN(LOWER(email) gin_trgm_ops);

-- **BTREE INDEXES FOR RANGE QUERIES** --
CREATE INDEX idx_users_created_at_btree ON users(created_at);
CREATE INDEX idx_refresh_tokens_created_at_btree ON refresh_tokens(created_at);
CREATE INDEX idx_user_sessions_created_at_btree ON user_sessions(created_at);

-- **COMPOSITE INDEXES FOR COMMON QUERY PATTERNS** --
CREATE INDEX idx_users_role_verified_active ON users(role, is_verified, is_active);
CREATE INDEX idx_refresh_tokens_user_expires_revoked ON refresh_tokens(user_id, expires_at, is_revoked);
CREATE INDEX idx_user_sessions_user_active_expires ON user_sessions(user_id, is_active, expires_at);

-- **VACUUM AND ANALYZE SETTINGS** --
-- ALTER TABLE users SET (autovacuum_vacuum_scale_factor = 0.1);
-- ALTER TABLE login_attempts SET (autovacuum_vacuum_scale_factor = 0.05);
-- ALTER TABLE audit_logs SET (autovacuum_vacuum_scale_factor = 0.05);
-- ALTER TABLE refresh_tokens SET (autovacuum_vacuum_scale_factor = 0.1);
-- ALTER TABLE user_sessions SET (autovacuum_vacuum_scale_factor = 0.1);

-- **AUTOMATIC CLEANUP FUNCTIONS** --

-- Function to clean expired tokens
CREATE OR REPLACE FUNCTION cleanup_expired_tokens()
RETURNS void AS $$
BEGIN
    -- Clean expired password reset tokens
    DELETE FROM password_reset_tokens WHERE expires_at < NOW() - INTERVAL '1 day';
    
    -- Clean expired blocklisted tokens
    DELETE FROM blacklisted_tokens WHERE expires_at < NOW() - INTERVAL '1 day';
    
    -- Clean expired sessions
    DELETE FROM user_sessions WHERE expires_at < NOW();
    
    -- Clean revoked refresh tokens older than 30 days
    DELETE FROM refresh_tokens WHERE is_revoked = true AND created_at < NOW() - INTERVAL '30 days';
END;
$$ LANGUAGE plpgsql;

-- Function to clean old audit logs
CREATE OR REPLACE FUNCTION cleanup_old_audit_logs(retention_days INTEGER DEFAULT 90)
RETURNS void AS $$
BEGIN
    DELETE FROM audit_logs WHERE created_at < NOW() - (retention_days || ' days')::INTERVAL;
END;
$$ LANGUAGE plpgsql;

-- Function to clean old login attempts
CREATE OR REPLACE FUNCTION cleanup_old_login_attempts(retention_days INTEGER DEFAULT 30)
RETURNS void AS $$
BEGIN
    DELETE FROM login_attempts WHERE created_at < NOW() - (retention_days || ' days')::INTERVAL;
END;
$$ LANGUAGE plpgsql;

-- **TRIGGER FOR AUTOMATIC UPDATED_AT** --
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW() AT TIME ZONE 'UTC';
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Apply updated_at triggers
CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_security_questions_updated_at BEFORE UPDATE ON security_questions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_user_security_questions_updated_at BEFORE UPDATE ON user_security_questions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();