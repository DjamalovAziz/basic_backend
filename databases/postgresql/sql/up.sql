-- Admin
CREATE TABLE admins (
    id VARCHAR PRIMARY KEY,
    
    password VARCHAR NOT NULL,
    role VARCHAR NOT NULL,
    phone_number VARCHAR NOT NULL,

    created_at VARCHAR NOT NULL,
    updated_at VARCHAR
);


-- Management
CREATE TABLE users (
    id VARCHAR PRIMARY KEY,

    password VARCHAR NOT NULL,
    image_path VARCHAR NOT NULL,
    phone_number VARCHAR NOT NULL,

    email VARCHAR,

    created_at VARCHAR NOT NULL,
    updated_at VARCHAR
);

CREATE TABLE relations (
    id VARCHAR PRIMARY KEY,
    
    organization_id VARCHAR NOT NULL,
    branch_id VARCHAR NOT NULL,
    user_id VARCHAR NOT NULL,
    role VARCHAR NOT NULL,
    relation_type VARCHAR NOT NULL,

    created_at VARCHAR NOT NULL,
    updated_at VARCHAR
);


-- Message
CREATE TABLE telegram_groups (
    id VARCHAR PRIMARY KEY,

    group_id VARCHAR NOT NULL,

    name VARCHAR,

    organization_id VARCHAR NOT NULL,
    branch_id VARCHAR NOT NULL,

    created_at VARCHAR NOT NULL,
    updated_at VARCHAR
);

CREATE TABLE fcm_subscriptions (
    id VARCHAR PRIMARY KEY,
    
    fcm_token VARCHAR NOT NULL,

    organization_id VARCHAR NOT NULL,
    branch_id VARCHAR NOT NULL,
    user_id VARCHAR NOT NULL,

    created_at VARCHAR NOT NULL,
);

CREATE TABLE subscriptions (
    id VARCHAR PRIMARY KEY,
    
    subscription VARCHAR NOT NULL, -- SubscriptionField

    organization_id VARCHAR NOT NULL,
    branch_id VARCHAR NOT NULL,
    user_id VARCHAR NOT NULL,

    created_at VARCHAR NOT NULL,
);


-- Organization
CREATE TABLE organizations (
    id VARCHAR PRIMARY KEY,

    name VARCHAR NOT NULL,

    created_at VARCHAR NOT NULL,
    updated_at VARCHAR
);

CREATE TABLE branchs (
    id VARCHAR PRIMARY KEY,

    name VARCHAR NOT NULL,

    branch_location VARCHAR,
    for_call VARCHAR, -- Vec<ForCall>
    
    organization_id VARCHAR NOT NULL,

    created_at VARCHAR NOT NULL,
    updated_at VARCHAR
);

-- Admins
ALTER TABLE admins
ADD CONSTRAINT unique_phone_number UNIQUE (phone_number);

-- Users
ALTER TABLE users
ADD CONSTRAINT unique_phone_number UNIQUE (phone_number),
CONSTRAINT unique_email UNIQUE (email);

-- Telegram Groups
ALTER TABLE telegram_groups
ADD CONSTRAINT unique_group_id UNIQUE (group_id);

-- Branches
ALTER TABLE branchs
ADD CONSTRAINT unique_name UNIQUE (name);

-- FCM Subscriptions
ALTER TABLE fcm_subscriptions
ADD CONSTRAINT unique_fcm_token UNIQUE (fcm_token);

-- Subscriptions
ALTER TABLE subscriptions
ADD CONSTRAINT unique_subscription UNIQUE (subscription);

-- Organizations
ALTER TABLE organizations
ADD CONSTRAINT unique_name UNIQUE (name);