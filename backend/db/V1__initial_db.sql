CREATE TABLE public.user_account
(
    id       uuid,
    username character varying(20) NOT NULL,
    password character(96)         NOT NULL,
    email    character varying(80) NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT user_account_unique_username UNIQUE (username),
    CONSTRAINT user_account_unique_email UNIQUE (email)
);

CREATE TABLE public.user_activation_token
(
    user_id uuid NOT NULL,
    token   character(172),
    PRIMARY KEY (user_id),
    FOREIGN KEY (user_id) REFERENCES user_account (id) ON DELETE CASCADE
)