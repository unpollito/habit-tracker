CREATE TABLE public.user_account
(
    id         uuid,
    username   character varying(20) NOT NULL,
    password   character(96)         NOT NULL,
    email      character varying(80) NOT NULL,
    is_enabled boolean               NOT NULL DEFAULT FALSE,
    created_at timestamp             NOT NULL DEFAULT NOW(),
    updated_at timestamp             NOT NULL DEFAULT NOW(),
    deleted_at timestamp             NULL,
    PRIMARY KEY (id),
    CONSTRAINT user_account_unique_username UNIQUE (username),
    CONSTRAINT user_account_unique_email UNIQUE (email)
);

CREATE TABLE public.user_activation_token
(
    user_id    uuid      NOT NULL,
    token      character(172),
    created_at timestamp NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id),
    FOREIGN KEY (user_id) REFERENCES user_account (id) ON DELETE CASCADE
);
