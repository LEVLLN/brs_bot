--
-- PostgreSQL database dump
--

-- Dumped from database version 14.10
-- Dumped by pg_dump version 14.10

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: answerentitycontenttypesenum; Type: TYPE; Schema: public; Owner: bread_bot
--

CREATE TYPE public.answerentitycontenttypesenum AS ENUM (
    'TEXT',
    'VOICE',
    'PICTURE',
    'ANIMATION',
    'VIDEO',
    'VIDEO_NOTE',
    'STICKER'
    );


ALTER TYPE public.answerentitycontenttypesenum OWNER TO bread_bot;

--
-- Name: answerentitytypesenum; Type: TYPE; Schema: public; Owner: bread_bot
--

CREATE TYPE public.answerentitytypesenum AS ENUM (
    'TRIGGER',
    'SUBSTRING'
    );


ALTER TYPE public.answerentitytypesenum OWNER TO bread_bot;


SET default_tablespace = '';

SET default_table_access_method = heap;

-- Name: alembic_version; Type: TABLE; Schema: public; Owner: bread_bot
--

CREATE TABLE public.alembic_version
(
    version_num character varying(32) NOT NULL
);


ALTER TABLE public.alembic_version
    OWNER TO bread_bot;

--
-- Name: answer_entities; Type: TABLE; Schema: public; Owner: bread_bot
--

CREATE TABLE public.answer_entities
(
    is_active      boolean                             NOT NULL,
    id             integer                             NOT NULL,
    created_at     timestamp without time zone         NOT NULL,
    updated_at     timestamp without time zone         NOT NULL,
    key            character varying(255)              NOT NULL,
    value          text                                NOT NULL,
    reaction_type  public.answerentitytypesenum        NOT NULL,
    content_type   public.answerentitycontenttypesenum NOT NULL,
    description    text,
    pack_id        integer,
    file_unique_id character varying(255)
);


ALTER TABLE public.answer_entities
    OWNER TO bread_bot;

--
-- Name: answer_entities_id_seq; Type: SEQUENCE; Schema: public; Owner: bread_bot
--

CREATE SEQUENCE public.answer_entities_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.answer_entities_id_seq
    OWNER TO bread_bot;

--
-- Name: answer_entities_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: bread_bot
--

ALTER SEQUENCE public.answer_entities_id_seq OWNED BY public.answer_entities.id;


--
-- Name: answer_packs; Type: TABLE; Schema: public; Owner: bread_bot
--

CREATE TABLE public.answer_packs
(
    is_active     boolean                     NOT NULL,
    id            integer                     NOT NULL,
    created_at    timestamp without time zone NOT NULL,
    updated_at    timestamp without time zone NOT NULL,
    name          character varying(255)      NOT NULL,
    is_private    boolean                     NOT NULL,
    author        integer,
    answer_chance smallint                    NOT NULL
);


ALTER TABLE public.answer_packs
    OWNER TO bread_bot;

--
-- Name: answer_packs_id_seq; Type: SEQUENCE; Schema: public; Owner: bread_bot
--

CREATE SEQUENCE public.answer_packs_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.answer_packs_id_seq
    OWNER TO bread_bot;

--
-- Name: answer_packs_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: bread_bot
--

ALTER SEQUENCE public.answer_packs_id_seq OWNED BY public.answer_packs.id;


--
-- Name: answer_packs_to_chats; Type: TABLE; Schema: public; Owner: bread_bot
--

CREATE TABLE public.answer_packs_to_chats
(
    is_active  boolean                     NOT NULL,
    id         integer                     NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    pack_id    integer,
    chat_id    integer
);


ALTER TABLE public.answer_packs_to_chats
    OWNER TO bread_bot;

--
-- Name: answer_packs_to_chats_id_seq; Type: SEQUENCE; Schema: public; Owner: bread_bot
--

CREATE SEQUENCE public.answer_packs_to_chats_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.answer_packs_to_chats_id_seq
    OWNER TO bread_bot;

--
-- Name: answer_packs_to_chats_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: bread_bot
--

ALTER SEQUENCE public.answer_packs_to_chats_id_seq OWNED BY public.answer_packs_to_chats.id;


--
-- Name: chats; Type: TABLE; Schema: public; Owner: bread_bot
--

CREATE TABLE public.chats
(
    is_active           boolean                     NOT NULL,
    id                  integer                     NOT NULL,
    created_at          timestamp without time zone NOT NULL,
    updated_at          timestamp without time zone NOT NULL,
    chat_id             bigint                      NOT NULL,
    name                character varying(255),
    morph_answer_chance smallint                    NOT NULL,
    is_openai_enabled   boolean                     NOT NULL
);


ALTER TABLE public.chats
    OWNER TO bread_bot;

--
-- Name: chats_id_seq; Type: SEQUENCE; Schema: public; Owner: bread_bot
--

CREATE SEQUENCE public.chats_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.chats_id_seq
    OWNER TO bread_bot;

--
-- Name: chats_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: bread_bot
--

ALTER SEQUENCE public.chats_id_seq OWNED BY public.chats.id;


--
-- Name: chats_to_members; Type: TABLE; Schema: public; Owner: bread_bot
--

CREATE TABLE public.chats_to_members
(
    is_active  boolean                     NOT NULL,
    id         integer                     NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    member_id  integer,
    chat_id    integer
);


ALTER TABLE public.chats_to_members
    OWNER TO bread_bot;

--
-- Name: chats_to_members_id_seq; Type: SEQUENCE; Schema: public; Owner: bread_bot
--

CREATE SEQUENCE public.chats_to_members_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.chats_to_members_id_seq
    OWNER TO bread_bot;

--
-- Name: chats_to_members_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: bread_bot
--

ALTER SEQUENCE public.chats_to_members_id_seq OWNED BY public.chats_to_members.id;


--
-- Name: dictionary_entities; Type: TABLE; Schema: public; Owner: bread_bot
--

CREATE TABLE public.dictionary_entities
(
    is_active  boolean                     NOT NULL,
    id         integer                     NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    value      text                        NOT NULL,
    chat_id    integer
);


ALTER TABLE public.dictionary_entities
    OWNER TO bread_bot;

--
-- Name: dictionary_entities_id_seq; Type: SEQUENCE; Schema: public; Owner: bread_bot
--

CREATE SEQUENCE public.dictionary_entities_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.dictionary_entities_id_seq
    OWNER TO bread_bot;

--
-- Name: dictionary_entities_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: bread_bot
--

ALTER SEQUENCE public.dictionary_entities_id_seq OWNED BY public.dictionary_entities.id;


--
-- Name: members; Type: TABLE; Schema: public; Owner: bread_bot
--

CREATE TABLE public.members
(
    is_active  boolean                     NOT NULL,
    id         integer                     NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    username   character varying(255)      NOT NULL,
    first_name character varying(255),
    last_name  character varying(255),
    is_bot     boolean,
    member_id  bigint                      NOT NULL
);


ALTER TABLE public.members
    OWNER TO bread_bot;

--
-- Name: members_id_seq; Type: SEQUENCE; Schema: public; Owner: bread_bot
--

CREATE SEQUENCE public.members_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.members_id_seq
    OWNER TO bread_bot;

--
-- Name: members_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: bread_bot
--

ALTER SEQUENCE public.members_id_seq OWNED BY public.members.id;



--
-- Name: users; Type: TABLE; Schema: public; Owner: bread_bot
--

CREATE TABLE public.users
(
    is_active       boolean                     NOT NULL,
    id              integer                     NOT NULL,
    created_at      timestamp without time zone NOT NULL,
    updated_at      timestamp without time zone NOT NULL,
    username        character varying(255)      NOT NULL,
    first_name      character varying(255),
    surname         character varying(255),
    email           character varying(255)      NOT NULL,
    hashed_password character varying           NOT NULL,
    is_admin        boolean                     NOT NULL
);


ALTER TABLE public.users
    OWNER TO bread_bot;

--
-- Name: users_id_seq; Type: SEQUENCE; Schema: public; Owner: bread_bot
--

CREATE SEQUENCE public.users_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.users_id_seq
    OWNER TO bread_bot;

--
-- Name: users_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: bread_bot
--

ALTER SEQUENCE public.users_id_seq OWNED BY public.users.id;


--
-- Name: answer_entities id; Type: DEFAULT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.answer_entities
    ALTER COLUMN id SET DEFAULT nextval('public.answer_entities_id_seq'::regclass);


--
-- Name: answer_packs id; Type: DEFAULT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.answer_packs
    ALTER COLUMN id SET DEFAULT nextval('public.answer_packs_id_seq'::regclass);


--
-- Name: answer_packs_to_chats id; Type: DEFAULT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.answer_packs_to_chats
    ALTER COLUMN id SET DEFAULT nextval('public.answer_packs_to_chats_id_seq'::regclass);


--
-- Name: chats id; Type: DEFAULT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.chats
    ALTER COLUMN id SET DEFAULT nextval('public.chats_id_seq'::regclass);


--
-- Name: chats_to_members id; Type: DEFAULT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.chats_to_members
    ALTER COLUMN id SET DEFAULT nextval('public.chats_to_members_id_seq'::regclass);


--
-- Name: dictionary_entities id; Type: DEFAULT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.dictionary_entities
    ALTER COLUMN id SET DEFAULT nextval('public.dictionary_entities_id_seq'::regclass);


--
-- Name: members id; Type: DEFAULT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.members
    ALTER COLUMN id SET DEFAULT nextval('public.members_id_seq'::regclass);


--
-- Name: users id; Type: DEFAULT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.users
    ALTER COLUMN id SET DEFAULT nextval('public.users_id_seq'::regclass);



--
-- Name: alembic_version alembic_version_pkc; Type: CONSTRAINT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.alembic_version
    ADD CONSTRAINT alembic_version_pkc PRIMARY KEY (version_num);


--
-- Name: answer_entities answer_entities_pkey; Type: CONSTRAINT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.answer_entities
    ADD CONSTRAINT answer_entities_pkey PRIMARY KEY (id);


--
-- Name: answer_packs answer_packs_pkey; Type: CONSTRAINT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.answer_packs
    ADD CONSTRAINT answer_packs_pkey PRIMARY KEY (id);


--
-- Name: answer_packs_to_chats answer_packs_to_chats_pkey; Type: CONSTRAINT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.answer_packs_to_chats
    ADD CONSTRAINT answer_packs_to_chats_pkey PRIMARY KEY (id);


--
-- Name: chats chats_chat_id_key; Type: CONSTRAINT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.chats
    ADD CONSTRAINT chats_chat_id_key UNIQUE (chat_id);


--
-- Name: chats chats_pkey; Type: CONSTRAINT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.chats
    ADD CONSTRAINT chats_pkey PRIMARY KEY (id);


--
-- Name: chats_to_members chats_to_members_pkey; Type: CONSTRAINT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.chats_to_members
    ADD CONSTRAINT chats_to_members_pkey PRIMARY KEY (id);


--
-- Name: dictionary_entities dictionary_entities_pkey; Type: CONSTRAINT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.dictionary_entities
    ADD CONSTRAINT dictionary_entities_pkey PRIMARY KEY (id);


--
-- Name: members members_pkey; Type: CONSTRAINT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.members
    ADD CONSTRAINT members_pkey PRIMARY KEY (id);

--
-- Name: users users_email_key; Type: CONSTRAINT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_email_key UNIQUE (email);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: users users_username_key; Type: CONSTRAINT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_username_key UNIQUE (username);


--
-- Name: answer_entities answer_entities_pack_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.answer_entities
    ADD CONSTRAINT answer_entities_pack_id_fkey FOREIGN KEY (pack_id) REFERENCES public.answer_packs (id) ON DELETE CASCADE;


--
-- Name: answer_packs answer_packs_author_fkey; Type: FK CONSTRAINT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.answer_packs
    ADD CONSTRAINT answer_packs_author_fkey FOREIGN KEY (author) REFERENCES public.members (id) ON DELETE CASCADE;


--
-- Name: answer_packs_to_chats answer_packs_to_chats_chat_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.answer_packs_to_chats
    ADD CONSTRAINT answer_packs_to_chats_chat_id_fkey FOREIGN KEY (chat_id) REFERENCES public.chats (id) ON DELETE CASCADE;


--
-- Name: answer_packs_to_chats answer_packs_to_chats_pack_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.answer_packs_to_chats
    ADD CONSTRAINT answer_packs_to_chats_pack_id_fkey FOREIGN KEY (pack_id) REFERENCES public.answer_packs (id) ON DELETE CASCADE;


--
-- Name: chats_to_members chats_to_members_chat_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.chats_to_members
    ADD CONSTRAINT chats_to_members_chat_id_fkey FOREIGN KEY (chat_id) REFERENCES public.chats (id) ON DELETE CASCADE;


--
-- Name: chats_to_members chats_to_members_member_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.chats_to_members
    ADD CONSTRAINT chats_to_members_member_id_fkey FOREIGN KEY (member_id) REFERENCES public.members (id) ON DELETE CASCADE;


--
-- Name: dictionary_entities dictionary_entities_chat_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.dictionary_entities
    ADD CONSTRAINT dictionary_entities_chat_id_fkey FOREIGN KEY (chat_id) REFERENCES public.chats (id) ON DELETE CASCADE;

--
-- PostgreSQL database dump complete
--

