-- Created by Vertabelo (http://vertabelo.com)
-- Last modification date: 2022-09-12 17:12:02.177

-- tables
-- Table: asset
CREATE TABLE tbl.asset (
    pkey_id bigint  NOT NULL,
    short_name varchar  NOT NULL,
    long_name varchar  NOT NULL,
    description varchar  NOT NULL,
    network_type varchar  NOT NULL,
    contract_address varchar  NOT NULL,
    precision int  NOT NULL,
    fkey_prototype bigint  NULL,
    CONSTRAINT asset_pk PRIMARY KEY (pkey_id)
);

-- Table: asset_prototype
CREATE TABLE tbl.asset_prototype (
    pkey_id bigint  NOT NULL,
    name varchar  NOT NULL,
    description varchar  NOT NULL,
    tokenomics_url varchar  NULL,
    network_type varchar  NOT NULL,
    fkey_organization bigint  NULL,
    precision int  NOT NULL,
    CONSTRAINT asset_prototype_pk PRIMARY KEY (pkey_id)
);

-- Table: asset_tokenomics
CREATE TABLE tbl.asset_tokenomics (
    pkey_id bigint  NOT NULL,
    fkey_asset_prototype bigint  NOT NULL,
    name varchar  NOT NULL,
    note varchar  NOT NULL,
    wallet_address varchar  NOT NULL,
    value real  NOT NULL,
    CONSTRAINT asset_tokenomics_pk PRIMARY KEY (pkey_id)
);

-- Table: asset_transfer_plan
CREATE TABLE tbl.asset_transfer_plan (
    pkey_id bigint  NOT NULL,
    fkey_asset bigint  NOT NULL,
    fkey_asset_tokenomics bigint  NULL,
    wallet_from varchar  NOT NULL,
    wallet_to varchar  NOT NULL,
    transfer_value real  NOT NULL,
    transaction_hash varchar  NOT NULL,
    updated_at oid  NOT NULL,
    created_at oid  NOT NULL,
    CONSTRAINT asset_transfer_plan_pk PRIMARY KEY (pkey_id)
);

-- Table: authorization_attempt
CREATE TABLE tbl.authorization_attempt (
    pkey_id bigint  NOT NULL DEFAULT nextval( 'tbl.seq_authorization_attempt_id' ),
    fkey_user bigint  NOT NULL,
    ip_address inet  NOT NULL,
    is_token_ok boolean  NOT NULL,
    moment oid  NOT NULL DEFAULT EXTRACT(EPOCH FROM (NOW()))::bigint,
    CONSTRAINT "tbl.authorization_attempt_pk" PRIMARY KEY (pkey_id)
);

-- Table: bad_request
CREATE TABLE tbl.bad_request (
    pkey_id bigint  NOT NULL DEFAULT nextval( 'tbl.seq_bad_request_id' ),
    fkey_user bigint  NULL,
    ip_address inet  NOT NULL,
    method_code integer  NULL,
    error_code integer  NOT NULL,
    device_id varchar(256)  NULL,
    device_os varchar(64)  NULL,
    raw varchar(16384)  NULL,
    moment oid  NOT NULL DEFAULT EXTRACT(EPOCH FROM (NOW()))::bigint,
    CONSTRAINT "tbl.bad_request_pk" PRIMARY KEY (pkey_id)
);

-- Table: bucket
CREATE TABLE tbl.bucket (
    pkey_id bigint  NOT NULL,
    name varchar  NOT NULL,
    fkey_organization bigint  NOT NULL,
    always_online boolean  NOT NULL,
    network_type bigint  NOT NULL,
    CONSTRAINT bucket_pk PRIMARY KEY (pkey_id)
);

-- Table: favourite_wallet
CREATE TABLE tbl.favourite_wallet (
    pkey_id bigint  NOT NULL,
    fkey_user bigint  NOT NULL,
    fkey_wallet bigint  NOT NULL,
    user_pkey_id bigint  NOT NULL,
    CONSTRAINT favourite_wallet_pk PRIMARY KEY (pkey_id)
);

-- Table: login_attempt
CREATE TABLE tbl.login_attempt (
    pkey_id bigint  NOT NULL DEFAULT nextval( 'tbl.seq_login_attempt_id' ),
    fkey_user bigint  NULL,
    username varchar(20)  NOT NULL,
    password_hash bytea  NOT NULL,
    ip_address inet  NOT NULL,
    device_id varchar(256)  NULL,
    device_os varchar(64)  NULL,
    is_password_ok boolean  NULL,
    moment oid  NOT NULL DEFAULT EXTRACT(EPOCH FROM (NOW()))::bigint,
    CONSTRAINT "tbl.login_attempt_pk" PRIMARY KEY (pkey_id)
);

-- Table: organization
CREATE TABLE tbl.organization (
    pkey_id bigint  NOT NULL,
    name varchar  NOT NULL,
    country varchar  NOT NULL,
    tax_id varchar  NOT NULL,
    address varchar  NOT NULL,
    note varchar  NOT NULL,
    CONSTRAINT organization_pk PRIMARY KEY (pkey_id)
);

-- Table: organization_membership
CREATE TABLE tbl.organization_membership (
    pkey_id bigint  NOT NULL,
    fkey_user bigint  NOT NULL,
    fkey_organization bigint  NOT NULL,
    is_admin boolean  NOT NULL,
    is_owner boolean  NOT NULL,
    created_at bigint  NOT NULL,
    accepted boolean  NOT NULL,
    responsed boolean  NOT NULL,
    CONSTRAINT organization_membership_pk PRIMARY KEY (pkey_id)
);

-- Table: password_reset_attempt
CREATE TABLE tbl.password_reset_attempt (
    pkey_id bigint  NOT NULL DEFAULT nextval( 'tbl.seq_password_reset_attempt_id' ),
    fkey_user bigint  NOT NULL,
    initiated_at oid  NOT NULL DEFAULT EXTRACT(EPOCH FROM (NOW()))::bigint,
    valid_until oid  NOT NULL DEFAULT EXTRACT(EPOCH FROM (NOW()))::bigint + 86400,
    code varchar(256)  NOT NULL,
    CONSTRAINT password_reset_attempt_pk PRIMARY KEY (pkey_id)
);

-- Table: recovery_question
CREATE TABLE tbl.recovery_question (
    pkey_id bigint  NOT NULL DEFAULT nextval( 'tbl.seq_recovery_question_id' ),
    fkey_user bigint  NOT NULL,
    fkey_question smallint  NOT NULL,
    answer varchar(256)  NOT NULL,
    CONSTRAINT recovery_question_pk PRIMARY KEY (pkey_id)
);

-- Table: recovery_question_data
CREATE TABLE tbl.recovery_question_data (
    pkey_id smallint  NOT NULL,
    content varchar(256)  NOT NULL,
    category enum_recovery_question_category  NOT NULL,
    CONSTRAINT recovery_question_data_pk PRIMARY KEY (pkey_id)
);

-- Table: support_ticket
CREATE TABLE tbl.support_ticket (
    pkey_id bigint  NOT NULL,
    fkey_user bigint  NOT NULL,
    fkey_handler_user bigint  NULL,
    content varchar  NOT NULL,
    response varchar  NOT NULL,
    created_at oid  NOT NULL,
    updated_at oid  NOT NULL,
    CONSTRAINT support_ticket_pk PRIMARY KEY (pkey_id)
);

-- Table: transfer
CREATE TABLE tbl.transfer (
    pkey_id bigint  NOT NULL DEFAULT NEXTVAL( 'tbl.seq_transfer_id' ),
    fkey_wallet bigint  NULL,
    source_address varchar  NOT NULL,
    destination_address varchar  NOT NULL,
    quantity real  NOT NULL,
    network varchar  NOT NULL,
    contract_address varchar  NOT NULL DEFAULT 'pending',
    fkey_asset bigint  NULL,
    created_at  oid  NOT NULL DEFAULT EXTRACT(EPOCH FROM (NOW()))::bigint,
    updated_at oid  NOT NULL,
    CONSTRAINT transfer_pk PRIMARY KEY (pkey_id)
);

CREATE INDEX udix_transaction_id on tbl.transfer (pkey_id ASC);

-- Table: user
CREATE TABLE tbl."user" (
    pkey_id bigint  NOT NULL DEFAULT nextval( 'tbl.seq_user_id' ),
    role enum_role  NOT NULL DEFAULT 'user',
    public_id bigint  NOT NULL,
    username varchar(20)  NOT NULL,
    password_hash bytea  NOT NULL,
    password_salt bytea  NOT NULL,
    age smallint  NOT NULL,
    preferred_language varchar(5)  NOT NULL,
    family_name varchar(128)  NULL,
    given_name varchar(128)  NULL,
    agreed_tos boolean  NOT NULL,
    created_at oid  NOT NULL DEFAULT EXTRACT(EPOCH FROM (NOW()))::bigint,
    updated_at oid  NOT NULL DEFAULT EXTRACT(EPOCH FROM (NOW()))::bigint,
    email varchar(320)  NULL,
    phone_number varchar(15)  NULL,
    last_ip inet  NOT NULL,
    last_login oid  NULL,
    last_password_reset oid  NULL,
    logins_count integer  NOT NULL DEFAULT 0,
    user_device_id uuid  NULL,
    admin_device_id uuid  NULL,
    password_reset_token uuid  NULL,
    is_blocked boolean  NOT NULL DEFAULT false,
    CONSTRAINT uidx_user_username UNIQUE (username) NOT DEFERRABLE  INITIALLY IMMEDIATE,
    CONSTRAINT uidx_user_public_id UNIQUE (public_id) NOT DEFERRABLE  INITIALLY IMMEDIATE,
    CONSTRAINT user_pk PRIMARY KEY (pkey_id)
);

-- Table: vault_metadata
CREATE TABLE tbl.vault_metadata (
    pkey_id bigint  NOT NULL,
    public_key varchar  NOT NULL,
    fkey_asset bigint  NOT NULL,
    fkey_bucket bigint  NOT NULL,
    bucket_pkey_id bigint  NOT NULL,
    is_backed_up boolean  NOT NULL,
    CONSTRAINT vault_metadata_pk PRIMARY KEY (pkey_id)
);

-- Table: vault_wallet
CREATE TABLE tbl.vault_wallet (
    pkey_id bigint  NOT NULL,
    address varchar  NOT NULL,
    balance real  NOT NULL,
    fkey_vault_metadata bigint  NOT NULL,
    is_gas_vault boolean  NOT NULL,
    CONSTRAINT vault_wallet_pk PRIMARY KEY (pkey_id)
);

-- Table: wallet_blacklist
CREATE TABLE tbl.wallet_blacklist (
    pkey_id bigint  NOT NULL,
    network_type varchar  NOT NULL,
    address varchar  NOT NULL,
    CONSTRAINT wallet_blacklist_pk PRIMARY KEY (pkey_id)
);

-- foreign keys
-- Reference: asset_asset_prototype (table: asset)
ALTER TABLE tbl.asset ADD CONSTRAINT asset_asset_prototype
    FOREIGN KEY (fkey_prototype)
    REFERENCES tbl.asset_prototype (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: asset_prototype_asset_tokenomics (table: asset_tokenomics)
ALTER TABLE tbl.asset_tokenomics ADD CONSTRAINT asset_prototype_asset_tokenomics
    FOREIGN KEY (fkey_asset_prototype)
    REFERENCES tbl.asset_prototype (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: asset_transfer_plan_asset (table: asset_transfer_plan)
ALTER TABLE tbl.asset_transfer_plan ADD CONSTRAINT asset_transfer_plan_asset
    FOREIGN KEY (fkey_asset)
    REFERENCES tbl.asset (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: asset_transfer_plan_asset_tokenomics (table: asset_transfer_plan)
ALTER TABLE tbl.asset_transfer_plan ADD CONSTRAINT asset_transfer_plan_asset_tokenomics
    FOREIGN KEY (fkey_asset_tokenomics)
    REFERENCES tbl.asset_tokenomics (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: asset_vault_metadata (table: vault_metadata)
ALTER TABLE tbl.vault_metadata ADD CONSTRAINT asset_vault_metadata
    FOREIGN KEY (fkey_asset)
    REFERENCES tbl.asset (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: authorization_attempt_user (table: authorization_attempt)
ALTER TABLE tbl.authorization_attempt ADD CONSTRAINT authorization_attempt_user
    FOREIGN KEY (fkey_user)
    REFERENCES tbl."user" (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: bad_request_user (table: bad_request)
ALTER TABLE tbl.bad_request ADD CONSTRAINT bad_request_user
    FOREIGN KEY (fkey_user)
    REFERENCES tbl."user" (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: favourite_wallet_user (table: favourite_wallet)
ALTER TABLE tbl.favourite_wallet ADD CONSTRAINT favourite_wallet_user
    FOREIGN KEY (fkey_user)
    REFERENCES tbl."user" (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: login_attempt_user (table: login_attempt)
ALTER TABLE tbl.login_attempt ADD CONSTRAINT login_attempt_user
    FOREIGN KEY (fkey_user)
    REFERENCES tbl."user" (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: organization_asset_prototype (table: asset_prototype)
ALTER TABLE tbl.asset_prototype ADD CONSTRAINT organization_asset_prototype
    FOREIGN KEY (fkey_organization)
    REFERENCES tbl.organization (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: organization_bucket_hot (table: bucket)
ALTER TABLE tbl.bucket ADD CONSTRAINT organization_bucket_hot
    FOREIGN KEY (fkey_organization)
    REFERENCES tbl.organization (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: organization_organization_membership (table: organization_membership)
ALTER TABLE tbl.organization_membership ADD CONSTRAINT organization_organization_membership
    FOREIGN KEY (fkey_organization)
    REFERENCES tbl.organization (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: password_reset_attempt_user (table: password_reset_attempt)
ALTER TABLE tbl.password_reset_attempt ADD CONSTRAINT password_reset_attempt_user
    FOREIGN KEY (fkey_user)
    REFERENCES tbl."user" (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: recovery_question_recovery_question_data (table: recovery_question)
ALTER TABLE tbl.recovery_question ADD CONSTRAINT recovery_question_recovery_question_data
    FOREIGN KEY (fkey_question)
    REFERENCES tbl.recovery_question_data (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: recovery_question_user (table: recovery_question)
ALTER TABLE tbl.recovery_question ADD CONSTRAINT recovery_question_user
    FOREIGN KEY (fkey_user)
    REFERENCES tbl."user" (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: support_ticket_user (table: support_ticket)
ALTER TABLE tbl.support_ticket ADD CONSTRAINT support_ticket_user
    FOREIGN KEY (fkey_handler_user)
    REFERENCES tbl."user" (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: transfer_asset (table: transfer)
ALTER TABLE tbl.transfer ADD CONSTRAINT transfer_asset
    FOREIGN KEY (fkey_asset)
    REFERENCES tbl.asset (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: transfer_vault_wallet (table: transfer)
ALTER TABLE tbl.transfer ADD CONSTRAINT transfer_vault_wallet
    FOREIGN KEY (fkey_wallet)
    REFERENCES tbl.vault_wallet (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: user_organization_membership (table: organization_membership)
ALTER TABLE tbl.organization_membership ADD CONSTRAINT user_organization_membership
    FOREIGN KEY (fkey_user)
    REFERENCES tbl."user" (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: user_support_ticket (table: support_ticket)
ALTER TABLE tbl.support_ticket ADD CONSTRAINT user_support_ticket
    FOREIGN KEY (fkey_user)
    REFERENCES tbl."user" (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: vault_metadata_bucket (table: vault_metadata)
ALTER TABLE tbl.vault_metadata ADD CONSTRAINT vault_metadata_bucket
    FOREIGN KEY (fkey_bucket)
    REFERENCES tbl.bucket (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: vault_wallet_favourite_wallet (table: favourite_wallet)
ALTER TABLE tbl.favourite_wallet ADD CONSTRAINT vault_wallet_favourite_wallet
    FOREIGN KEY (fkey_wallet)
    REFERENCES tbl.vault_wallet (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: vault_wallet_vault_metadata (table: vault_wallet)
ALTER TABLE tbl.vault_wallet ADD CONSTRAINT vault_wallet_vault_metadata
    FOREIGN KEY (fkey_vault_metadata)
    REFERENCES tbl.vault_metadata (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- sequences
-- Sequence: seq_address_id
CREATE SEQUENCE tbl.seq_address_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
      AS bigint
;

-- Sequence: seq_authorization_attempt_id
CREATE SEQUENCE tbl.seq_authorization_attempt_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
      AS bigint
;

-- Sequence: seq_bad_request_id
CREATE SEQUENCE tbl.seq_bad_request_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
      AS bigint
;

-- Sequence: seq_bucket_id
CREATE SEQUENCE tbl.seq_bucket_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
      AS bigint
;

-- Sequence: seq_company_id
CREATE SEQUENCE tbl.seq_company_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
      AS bigint
;

-- Sequence: seq_exchange_account_id
CREATE SEQUENCE tbl.seq_exchange_account_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
      AS bigint
;

-- Sequence: seq_external_wallet_id
CREATE SEQUENCE tbl.seq_external_wallet_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
      AS bigint
;

-- Sequence: seq_fiat_account_id
CREATE SEQUENCE tbl.seq_fiat_account_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
      AS bigint
;

-- Sequence: seq_internal_wallet_id
CREATE SEQUENCE tbl.seq_internal_wallet_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
      AS bigint
;

-- Sequence: seq_listing_id
CREATE SEQUENCE tbl.seq_listing_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
      AS bigint
;

-- Sequence: seq_login_attempt_id
CREATE SEQUENCE tbl.seq_login_attempt_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
      AS bigint
;

-- Sequence: seq_managed_wallet_id
CREATE SEQUENCE tbl.seq_managed_wallet_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
      AS bigint
;

-- Sequence: seq_oauth_id
CREATE SEQUENCE tbl.seq_oauth_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
      AS bigint
;

-- Sequence: seq_order_content_id
CREATE SEQUENCE tbl.seq_order_content_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
      AS bigint
;

-- Sequence: seq_order_id
CREATE SEQUENCE tbl.seq_order_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
      AS bigint
;

-- Sequence: seq_password_reset_attempt_id
CREATE SEQUENCE tbl.seq_password_reset_attempt_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
      AS bigint
;

-- Sequence: seq_recovery_question_id
CREATE SEQUENCE tbl.seq_recovery_question_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
      AS bigint
;

-- Sequence: seq_shipping_provider_id
CREATE SEQUENCE tbl.seq_shipping_provider_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
      AS bigint
;

-- Sequence: seq_transfer_id
CREATE SEQUENCE tbl.seq_transfer_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
      AS bigint
;

-- Sequence: seq_user_id
CREATE SEQUENCE tbl.seq_user_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
      AS bigint
;

-- Sequence: seq_vault_id
CREATE SEQUENCE tbl.seq_vault_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
      AS bigint
;

-- Sequence: seq_ver_id
CREATE SEQUENCE tbl.seq_ver_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
      AS bigint
;

-- End of file.

