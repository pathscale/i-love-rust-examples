create schema if not exists tbl;

-- sequences
-- Sequence: seq_address_id
CREATE SEQUENCE tbl.seq_address_id
    NO MINVALUE
    NO MAXVALUE
    NO CYCLE
;


-- Sequence: seq_authorization_attempt_id
CREATE SEQUENCE tbl.seq_authorization_attempt_id
    NO MINVALUE
    NO MAXVALUE
    NO CYCLE
;

-- Sequence: seq_bad_request_id
CREATE SEQUENCE tbl.seq_bad_request_id
    NO MINVALUE
    NO MAXVALUE
    NO CYCLE
;

-- Sequence: seq_company_id
CREATE SEQUENCE tbl.seq_company_id
    NO MINVALUE
    NO MAXVALUE
    NO CYCLE
;

-- Sequence: seq_listing_id
CREATE SEQUENCE tbl.seq_listing_id
    NO MINVALUE
    NO MAXVALUE
    NO CYCLE
;

-- Sequence: seq_login_attempt_id
CREATE SEQUENCE tbl.seq_login_attempt_id
    NO MINVALUE
    NO MAXVALUE
    NO CYCLE
;

-- Sequence: seq_oauth_id
CREATE SEQUENCE tbl.seq_oauth_id
    NO MINVALUE
    NO MAXVALUE
    NO CYCLE
;

-- Sequence: seq_order_content_id
CREATE SEQUENCE tbl.seq_order_content_id
    NO MINVALUE
    NO MAXVALUE
    NO CYCLE
;

-- Sequence: seq_order_id
CREATE SEQUENCE tbl.seq_order_id
    NO MINVALUE
    NO MAXVALUE
    NO CYCLE
;

-- Sequence: seq_password_reset_attempt_id
CREATE SEQUENCE tbl.seq_password_reset_attempt_id
    NO MINVALUE
    NO MAXVALUE
    NO CYCLE
;

-- Sequence: seq_paypal_auth_id
CREATE SEQUENCE tbl.seq_paypal_auth_id
    NO MINVALUE
    NO MAXVALUE
    NO CYCLE
;

-- Sequence: seq_recovery_question_id
CREATE SEQUENCE tbl.seq_recovery_question_id
    NO MINVALUE
    NO MAXVALUE
    NO CYCLE
;

-- Sequence: seq_shipping_id
CREATE SEQUENCE tbl.seq_shipping_id
    NO MINVALUE
    NO MAXVALUE
    NO CYCLE
;

-- Sequence: seq_shipping_provider_id
CREATE SEQUENCE tbl.seq_shipping_provider_id
    NO MINVALUE
    NO MAXVALUE
    NO CYCLE
;

-- Sequence: seq_usr_id
CREATE SEQUENCE tbl.seq_usr_id
    NO MINVALUE
    NO MAXVALUE
    NO CYCLE
;

-- Sequence: seq_ver_id
CREATE SEQUENCE tbl.seq_ver_id
    NO MINVALUE
    NO MAXVALUE
    NO CYCLE
;

-- Sequence: seq_watchlist_id
CREATE SEQUENCE tbl.seq_watchlist_id
    NO MINVALUE
    NO MAXVALUE
    NO CYCLE
;

-- Sequence: seq_wishlist_id
CREATE SEQUENCE tbl.seq_wishlist_id
    NO MINVALUE
    NO MAXVALUE
    NO CYCLE
;


-- Table: authorization_attempt
CREATE TABLE tbl.authorization_attempt
(
    pkey_id     bigint  NOT NULL DEFAULT nextval('tbl.seq_authorization_attempt_id'),
    fkey_usr    bigint  NOT NULL,
    ip_address  inet    NOT NULL,
    is_token_ok boolean NOT NULL,
    moment      oid     NOT NULL DEFAULT EXTRACT(EPOCH FROM (NOW()))::bigint,
    CONSTRAINT "tbl.authorization_attempt_pk" PRIMARY KEY (pkey_id)
);

CREATE UNIQUE INDEX uidx_authorization_attempt_id on tbl.authorization_attempt (pkey_id ASC);

CREATE INDEX idx_authorization_attempt_id on tbl.authorization_attempt (fkey_usr ASC);

-- Table: bad_request
CREATE TABLE tbl.bad_request
(
    pkey_id     bigint         NOT NULL DEFAULT nextval('tbl.seq_bad_request_id'),
    fkey_usr    bigint         NULL,
    ip_address  inet           NOT NULL,
    method_code integer        NULL,
    error_code  integer        NOT NULL,
    device_id   varchar(256)   NULL,
    device_os   varchar(64)    NULL,
    raw         varchar(16384) NULL,
    moment      oid            NOT NULL DEFAULT EXTRACT(EPOCH FROM (NOW()))::bigint,
    CONSTRAINT "tbl.bad_request_pk" PRIMARY KEY (pkey_id)
);

CREATE UNIQUE INDEX uidx_bad_request_id on tbl.bad_request (pkey_id ASC);

CREATE INDEX idx_bad_request_id on tbl.bad_request (fkey_usr ASC);


-- Table: login_attempt
CREATE TABLE tbl.login_attempt
(
    pkey_id        bigint       NOT NULL DEFAULT nextval('tbl.seq_login_attempt_id'),
    fkey_usr       bigint       NULL,
    username       varchar(20)  NOT NULL,
    password_hash  bytea        NOT NULL,
    ip_address     inet         NOT NULL,
    device_id      varchar(256) NULL,
    device_os      varchar(64)  NULL,
    is_password_ok boolean      NULL,
    moment         oid          NOT NULL DEFAULT EXTRACT(EPOCH FROM (NOW()))::bigint,
    CONSTRAINT "tbl.login_attempt_pk" PRIMARY KEY (pkey_id)
);

CREATE UNIQUE INDEX uidx_login_attempt_id on tbl.login_attempt (pkey_id ASC);

CREATE INDEX idx_login_attempt_id on tbl.login_attempt (fkey_usr ASC);

-- Table: password_reset_attempt
CREATE TABLE tbl.password_reset_attempt
(
    pkey_id      bigint       NOT NULL DEFAULT nextval('tbl.seq_password_reset_attempt_id'),
    fkey_usr     bigint       NOT NULL,
    initiated_at oid          NOT NULL DEFAULT EXTRACT(EPOCH FROM (NOW()))::bigint,
    valid_until  oid          NOT NULL DEFAULT EXTRACT(EPOCH FROM (NOW()))::bigint + 86400,
    code         varchar(256) NOT NULL,
    CONSTRAINT password_reset_attempt_pk PRIMARY KEY (pkey_id)
);

CREATE UNIQUE INDEX uidx_password_reset_attempt_id on tbl.password_reset_attempt (pkey_id ASC);

CREATE UNIQUE INDEX uidx_password_reset_attempt_user_id on tbl.password_reset_attempt (fkey_usr ASC);

-- Table: recovery_question
CREATE TABLE tbl.recovery_question
(
    pkey_id       bigint       NOT NULL DEFAULT nextval('tbl.seq_recovery_question_id'),
    fkey_usr      bigint       NOT NULL,
    fkey_question smallint     NOT NULL,
    answer        varchar(256) NOT NULL,
    CONSTRAINT recovery_question_pk PRIMARY KEY (pkey_id)
);

CREATE UNIQUE INDEX uidx_recovery_question_id on tbl.recovery_question (pkey_id ASC);

CREATE INDEX idx_recovery_question_usr on tbl.recovery_question (fkey_usr ASC);

CREATE UNIQUE INDEX uidx_recovery_question_usr_question_id on tbl.recovery_question (fkey_usr ASC, fkey_question ASC);

-- Table: recovery_question_data
CREATE TABLE tbl.recovery_question_data
(
    pkey_id  smallint                            NOT NULL,
    content  varchar(256)                        NOT NULL,
    category tbl.enum_recovery_question_category NOT NULL,
    CONSTRAINT recovery_question_data_pk PRIMARY KEY (pkey_id)
);

CREATE UNIQUE INDEX uidx_recovery_question_data on tbl.recovery_question_data (pkey_id ASC);

-- Table: usr
CREATE TABLE tbl.usr
(
    pkey_id              bigint        NOT NULL DEFAULT nextval('tbl.seq_usr_id'),
    role                 tbl.enum_role NOT NULL DEFAULT 'user',
    public_id            bigint        NOT NULL,
    username             varchar(20)   NOT NULL,
    password_hash        bytea         NOT NULL,
    password_salt        bytea         NOT NULL,
    age                  smallint      NOT NULL,
    preferred_language   varchar(5)    NOT NULL,
    family_name          varchar(128)  NULL,
    given_name           varchar(128)  NULL,
    agreed_tos           boolean       NOT NULL,
    agreed_privacy       boolean       NOT NULL,
    created_at           oid           NOT NULL DEFAULT EXTRACT(EPOCH FROM (NOW()))::bigint,
    updated_at           oid           NOT NULL DEFAULT EXTRACT(EPOCH FROM (NOW()))::bigint,
    email                varchar(320)  NULL,
    phone_number         varchar(15)   NULL,
    last_ip              inet          NOT NULL,
    last_login           oid           NULL,
    last_password_reset  oid           NULL,
    logins_count         integer       NOT NULL DEFAULT 0,
    user_device_id       varchar(256)  NULL,
    admin_device_id      varchar(256)  NULL,
    user_token           uuid          NULL,
    admin_token          uuid          NULL,
    password_reset_token uuid          NULL,
    reset_token_valid    oid           NULL,
    is_blocked           boolean       NOT NULL DEFAULT false,
    CONSTRAINT ak_user_public_id UNIQUE (public_id) NOT DEFERRABLE INITIALLY IMMEDIATE,
    CONSTRAINT ak_username UNIQUE (username) NOT DEFERRABLE INITIALLY IMMEDIATE,
    CONSTRAINT "tbl.usr_pk" PRIMARY KEY (pkey_id)
);

CREATE UNIQUE INDEX uidx_user_id on tbl.usr (pkey_id ASC);

CREATE UNIQUE INDEX uidx_user_username on tbl.usr (username ASC);

CREATE UNIQUE INDEX uidx_user_public_id on tbl.usr (public_id ASC);

CREATE UNIQUE INDEX uidx_case_insens_username ON tbl.usr (lower(username) ASC);;


-- Reference: authorization_attempt_usr (table: authorization_attempt)
ALTER TABLE tbl.authorization_attempt
    ADD CONSTRAINT authorization_attempt_usr
        FOREIGN KEY (fkey_usr)
            REFERENCES tbl.usr (pkey_id)
            NOT DEFERRABLE
                INITIALLY IMMEDIATE
;

-- Reference: bad_request_usr (table: bad_request)
ALTER TABLE tbl.bad_request
    ADD CONSTRAINT bad_request_usr
        FOREIGN KEY (fkey_usr)
            REFERENCES tbl.usr (pkey_id)
            NOT DEFERRABLE
                INITIALLY IMMEDIATE
;

-- Reference: login_attempt_usr (table: login_attempt)
ALTER TABLE tbl.login_attempt
    ADD CONSTRAINT login_attempt_usr
        FOREIGN KEY (fkey_usr)
            REFERENCES tbl.usr (pkey_id)
            NOT DEFERRABLE
                INITIALLY IMMEDIATE
;


-- Reference: password_reset_attempt_usr (table: password_reset_attempt)
ALTER TABLE tbl.password_reset_attempt
    ADD CONSTRAINT password_reset_attempt_usr
        FOREIGN KEY (fkey_usr)
            REFERENCES tbl.usr (pkey_id)
            NOT DEFERRABLE
                INITIALLY IMMEDIATE
;

-- Reference: recovery_question_recovery_question_data (table: recovery_question)
ALTER TABLE tbl.recovery_question
    ADD CONSTRAINT recovery_question_recovery_question_data
        FOREIGN KEY (fkey_question)
            REFERENCES tbl.recovery_question_data (pkey_id)
            NOT DEFERRABLE
                INITIALLY IMMEDIATE
;

-- Reference: recovery_question_usr (table: recovery_question)
ALTER TABLE tbl.recovery_question
    ADD CONSTRAINT recovery_question_usr
        FOREIGN KEY (fkey_usr)
            REFERENCES tbl.usr (pkey_id)
            NOT DEFERRABLE
                INITIALLY IMMEDIATE
;


