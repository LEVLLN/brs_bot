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
SELECT pg_catalog.set_config('search_path', '', false);
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

--
-- Name: procrastinate_job_event_type; Type: TYPE; Schema: public; Owner: bread_bot
--

CREATE TYPE public.procrastinate_job_event_type AS ENUM (
    'deferred',
    'started',
    'deferred_for_retry',
    'failed',
    'succeeded',
    'cancelled',
    'scheduled'
);


ALTER TYPE public.procrastinate_job_event_type OWNER TO bread_bot;

--
-- Name: procrastinate_job_status; Type: TYPE; Schema: public; Owner: bread_bot
--

CREATE TYPE public.procrastinate_job_status AS ENUM (
    'todo',
    'doing',
    'succeeded',
    'failed'
);


ALTER TYPE public.procrastinate_job_status OWNER TO bread_bot;

--
-- Name: procrastinate_defer_job(character varying, character varying, text, text, jsonb, timestamp with time zone); Type: FUNCTION; Schema: public; Owner: bread_bot
--

CREATE FUNCTION public.procrastinate_defer_job(queue_name character varying, task_name character varying, lock text, queueing_lock text, args jsonb, scheduled_at timestamp with time zone) RETURNS bigint
    LANGUAGE plpgsql
    AS $$
DECLARE
job_id bigint;
BEGIN
INSERT INTO procrastinate_jobs (queue_name, task_name, lock, queueing_lock, args, scheduled_at)
VALUES (queue_name, task_name, lock, queueing_lock, args, scheduled_at)
    RETURNING id INTO job_id;

RETURN job_id;
END;
$$;


ALTER FUNCTION public.procrastinate_defer_job(queue_name character varying, task_name character varying, lock text, queueing_lock text, args jsonb, scheduled_at timestamp with time zone) OWNER TO bread_bot;

--
-- Name: procrastinate_defer_periodic_job(character varying, character varying, character varying, character varying, bigint); Type: FUNCTION; Schema: public; Owner: bread_bot
--

CREATE FUNCTION public.procrastinate_defer_periodic_job(_queue_name character varying, _lock character varying, _queueing_lock character varying, _task_name character varying, _defer_timestamp bigint) RETURNS bigint
    LANGUAGE plpgsql
    AS $$
DECLARE
_job_id bigint;
	_defer_id bigint;
BEGIN

INSERT
INTO procrastinate_periodic_defers (task_name, queue_name, defer_timestamp)
VALUES (_task_name, _queue_name, _defer_timestamp)
    ON CONFLICT DO NOTHING
        RETURNING id into _defer_id;

IF _defer_id IS NULL THEN
        RETURN NULL;
END IF;

UPDATE procrastinate_periodic_defers
SET job_id = procrastinate_defer_job(
        _queue_name,
        _task_name,
        _lock,
        _queueing_lock,
        ('{"timestamp": ' || _defer_timestamp || '}')::jsonb,
        NULL
             )
WHERE id = _defer_id
        RETURNING job_id INTO _job_id;

DELETE
FROM procrastinate_periodic_defers
    USING (
            SELECT id
            FROM procrastinate_periodic_defers
            WHERE procrastinate_periodic_defers.task_name = _task_name
            AND procrastinate_periodic_defers.queue_name = _queue_name
            AND procrastinate_periodic_defers.defer_timestamp < _defer_timestamp
            ORDER BY id
            FOR UPDATE
        ) to_delete
WHERE procrastinate_periodic_defers.id = to_delete.id;

RETURN _job_id;
END;
$$;


ALTER FUNCTION public.procrastinate_defer_periodic_job(_queue_name character varying, _lock character varying, _queueing_lock character varying, _task_name character varying, _defer_timestamp bigint) OWNER TO bread_bot;

--
-- Name: procrastinate_defer_periodic_job(character varying, character varying, character varying, character varying, character varying, bigint, jsonb); Type: FUNCTION; Schema: public; Owner: bread_bot
--

CREATE FUNCTION public.procrastinate_defer_periodic_job(_queue_name character varying, _lock character varying, _queueing_lock character varying, _task_name character varying, _periodic_id character varying, _defer_timestamp bigint, _args jsonb) RETURNS bigint
    LANGUAGE plpgsql
    AS $$
DECLARE
_job_id bigint;
	_defer_id bigint;
BEGIN

INSERT
INTO procrastinate_periodic_defers (task_name, periodic_id, defer_timestamp)
VALUES (_task_name, _periodic_id, _defer_timestamp)
    ON CONFLICT DO NOTHING
        RETURNING id into _defer_id;

IF _defer_id IS NULL THEN
        RETURN NULL;
END IF;

UPDATE procrastinate_periodic_defers
SET job_id = procrastinate_defer_job(
        _queue_name,
        _task_name,
        _lock,
        _queueing_lock,
        _args,
        NULL
             )
WHERE id = _defer_id
        RETURNING job_id INTO _job_id;

DELETE
FROM procrastinate_periodic_defers
    USING (
            SELECT id
            FROM procrastinate_periodic_defers
            WHERE procrastinate_periodic_defers.task_name = _task_name
            AND procrastinate_periodic_defers.periodic_id = _periodic_id
            AND procrastinate_periodic_defers.defer_timestamp < _defer_timestamp
            ORDER BY id
            FOR UPDATE
        ) to_delete
WHERE procrastinate_periodic_defers.id = to_delete.id;

RETURN _job_id;
END;
$$;


ALTER FUNCTION public.procrastinate_defer_periodic_job(_queue_name character varying, _lock character varying, _queueing_lock character varying, _task_name character varying, _periodic_id character varying, _defer_timestamp bigint, _args jsonb) OWNER TO bread_bot;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: procrastinate_jobs; Type: TABLE; Schema: public; Owner: bread_bot
--

CREATE TABLE public.procrastinate_jobs (
                                           id bigint NOT NULL,
                                           queue_name character varying(128) NOT NULL,
                                           task_name character varying(128) NOT NULL,
                                           lock text,
                                           queueing_lock text,
                                           args jsonb DEFAULT '{}'::jsonb NOT NULL,
                                           status public.procrastinate_job_status DEFAULT 'todo'::public.procrastinate_job_status NOT NULL,
                                           scheduled_at timestamp with time zone,
                                           attempts integer DEFAULT 0 NOT NULL
);


ALTER TABLE public.procrastinate_jobs OWNER TO bread_bot;

--
-- Name: procrastinate_fetch_job(character varying[]); Type: FUNCTION; Schema: public; Owner: bread_bot
--

CREATE FUNCTION public.procrastinate_fetch_job(target_queue_names character varying[]) RETURNS public.procrastinate_jobs
    LANGUAGE plpgsql
    AS $$
DECLARE
found_jobs procrastinate_jobs;
BEGIN
WITH candidate AS (
    SELECT jobs.*
    FROM procrastinate_jobs AS jobs
    WHERE
      -- reject the job if its lock has earlier jobs
        NOT EXISTS (
            SELECT 1
            FROM procrastinate_jobs AS earlier_jobs
            WHERE
                jobs.lock IS NOT NULL
              AND earlier_jobs.lock = jobs.lock
              AND earlier_jobs.status IN ('todo', 'doing')
              AND earlier_jobs.id < jobs.id)
      AND jobs.status = 'todo'
      AND (target_queue_names IS NULL OR jobs.queue_name = ANY( target_queue_names ))
      AND (jobs.scheduled_at IS NULL OR jobs.scheduled_at <= now())
    ORDER BY jobs.id ASC LIMIT 1
    FOR UPDATE OF jobs SKIP LOCKED
            )
UPDATE procrastinate_jobs
SET status = 'doing'
    FROM candidate
WHERE procrastinate_jobs.id = candidate.id
    RETURNING procrastinate_jobs.* INTO found_jobs;

RETURN found_jobs;
END;
$$;


ALTER FUNCTION public.procrastinate_fetch_job(target_queue_names character varying[]) OWNER TO bread_bot;

--
-- Name: procrastinate_finish_job(integer, public.procrastinate_job_status); Type: FUNCTION; Schema: public; Owner: bread_bot
--

CREATE FUNCTION public.procrastinate_finish_job(job_id integer, end_status public.procrastinate_job_status) RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
UPDATE procrastinate_jobs
SET status = end_status,
    attempts = attempts + 1
WHERE id = job_id;
END;
$$;


ALTER FUNCTION public.procrastinate_finish_job(job_id integer, end_status public.procrastinate_job_status) OWNER TO bread_bot;

--
-- Name: procrastinate_finish_job(integer, public.procrastinate_job_status, timestamp with time zone); Type: FUNCTION; Schema: public; Owner: bread_bot
--

CREATE FUNCTION public.procrastinate_finish_job(job_id integer, end_status public.procrastinate_job_status, next_scheduled_at timestamp with time zone) RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
UPDATE procrastinate_jobs
SET status = end_status,
    attempts = attempts + 1,
    scheduled_at = COALESCE(next_scheduled_at, scheduled_at)
WHERE id = job_id;
END;
$$;


ALTER FUNCTION public.procrastinate_finish_job(job_id integer, end_status public.procrastinate_job_status, next_scheduled_at timestamp with time zone) OWNER TO bread_bot;

--
-- Name: procrastinate_finish_job(integer, public.procrastinate_job_status, timestamp with time zone, boolean); Type: FUNCTION; Schema: public; Owner: bread_bot
--

CREATE FUNCTION public.procrastinate_finish_job(job_id integer, end_status public.procrastinate_job_status, next_scheduled_at timestamp with time zone, delete_job boolean) RETURNS void
    LANGUAGE plpgsql
    AS $$
DECLARE
_job_id bigint;
BEGIN
    IF end_status NOT IN ('succeeded', 'failed') THEN
        RAISE 'End status should be either "succeeded" or "failed" (job id: %)', job_id;
END IF;
    IF delete_job THEN
DELETE FROM procrastinate_jobs
WHERE id = job_id AND status IN ('todo', 'doing')
    RETURNING id INTO _job_id;
ELSE
UPDATE procrastinate_jobs
SET status = end_status,
    attempts =
        CASE
            WHEN status = 'doing' THEN attempts + 1
            ELSE attempts
            END
WHERE id = job_id AND status IN ('todo', 'doing')
    RETURNING id INTO _job_id;
END IF;
    IF _job_id IS NULL THEN
        RAISE 'Job was not found or not in "doing" or "todo" status (job id: %)', job_id;
END IF;
END;
$$;


ALTER FUNCTION public.procrastinate_finish_job(job_id integer, end_status public.procrastinate_job_status, next_scheduled_at timestamp with time zone, delete_job boolean) OWNER TO bread_bot;

--
-- Name: procrastinate_notify_queue(); Type: FUNCTION; Schema: public; Owner: bread_bot
--

CREATE FUNCTION public.procrastinate_notify_queue() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
	PERFORM pg_notify('procrastinate_queue#' || NEW.queue_name, NEW.task_name);
	PERFORM pg_notify('procrastinate_any_queue', NEW.task_name);
RETURN NEW;
END;
$$;


ALTER FUNCTION public.procrastinate_notify_queue() OWNER TO bread_bot;

--
-- Name: procrastinate_retry_job(integer, timestamp with time zone); Type: FUNCTION; Schema: public; Owner: bread_bot
--

CREATE FUNCTION public.procrastinate_retry_job(job_id integer, retry_at timestamp with time zone) RETURNS void
    LANGUAGE plpgsql
    AS $$
DECLARE
_job_id bigint;
BEGIN
UPDATE procrastinate_jobs
SET status = 'todo',
    attempts = attempts + 1,
    scheduled_at = retry_at
WHERE id = job_id AND status = 'doing'
    RETURNING id INTO _job_id;
IF _job_id IS NULL THEN
        RAISE 'Job was not found or not in "doing" status (job id: %)', job_id;
END IF;
END;
$$;


ALTER FUNCTION public.procrastinate_retry_job(job_id integer, retry_at timestamp with time zone) OWNER TO bread_bot;

--
-- Name: procrastinate_trigger_scheduled_events_procedure(); Type: FUNCTION; Schema: public; Owner: bread_bot
--

CREATE FUNCTION public.procrastinate_trigger_scheduled_events_procedure() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
INSERT INTO procrastinate_events(job_id, type, at)
VALUES (NEW.id, 'scheduled'::procrastinate_job_event_type, NEW.scheduled_at);

RETURN NEW;
END;
$$;


ALTER FUNCTION public.procrastinate_trigger_scheduled_events_procedure() OWNER TO bread_bot;

--
-- Name: procrastinate_trigger_status_events_procedure_insert(); Type: FUNCTION; Schema: public; Owner: bread_bot
--

CREATE FUNCTION public.procrastinate_trigger_status_events_procedure_insert() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
INSERT INTO procrastinate_events(job_id, type)
VALUES (NEW.id, 'deferred'::procrastinate_job_event_type);
RETURN NEW;
END;
$$;


ALTER FUNCTION public.procrastinate_trigger_status_events_procedure_insert() OWNER TO bread_bot;

--
-- Name: procrastinate_trigger_status_events_procedure_update(); Type: FUNCTION; Schema: public; Owner: bread_bot
--

CREATE FUNCTION public.procrastinate_trigger_status_events_procedure_update() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
WITH t AS (
    SELECT CASE
               WHEN OLD.status = 'todo'::procrastinate_job_status
               AND NEW.status = 'doing'::procrastinate_job_status
    THEN 'started'::procrastinate_job_event_type
    WHEN OLD.status = 'doing'::procrastinate_job_status
    AND NEW.status = 'todo'::procrastinate_job_status
    THEN 'deferred_for_retry'::procrastinate_job_event_type
    WHEN OLD.status = 'doing'::procrastinate_job_status
    AND NEW.status = 'failed'::procrastinate_job_status
    THEN 'failed'::procrastinate_job_event_type
    WHEN OLD.status = 'doing'::procrastinate_job_status
    AND NEW.status = 'succeeded'::procrastinate_job_status
    THEN 'succeeded'::procrastinate_job_event_type
    WHEN OLD.status = 'todo'::procrastinate_job_status
    AND (
    NEW.status = 'failed'::procrastinate_job_status
    OR NEW.status = 'succeeded'::procrastinate_job_status
    )
    THEN 'cancelled'::procrastinate_job_event_type
    ELSE NULL
END as event_type
    )
    INSERT INTO procrastinate_events(job_id, type)
SELECT NEW.id, t.event_type
FROM t
WHERE t.event_type IS NOT NULL;
RETURN NEW;
END;
$$;


ALTER FUNCTION public.procrastinate_trigger_status_events_procedure_update() OWNER TO bread_bot;

--
-- Name: procrastinate_unlink_periodic_defers(); Type: FUNCTION; Schema: public; Owner: bread_bot
--

CREATE FUNCTION public.procrastinate_unlink_periodic_defers() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
UPDATE procrastinate_periodic_defers
SET job_id = NULL
WHERE job_id = OLD.id;
RETURN OLD;
END;
$$;


ALTER FUNCTION public.procrastinate_unlink_periodic_defers() OWNER TO bread_bot;

--
-- Name: _sqlx_migrations; Type: TABLE; Schema: public; Owner: bread_bot
--

CREATE TABLE public._sqlx_migrations (
                                         version bigint NOT NULL,
                                         description text NOT NULL,
                                         installed_on timestamp with time zone DEFAULT now() NOT NULL,
                                         success boolean NOT NULL,
                                         checksum bytea NOT NULL,
                                         execution_time bigint NOT NULL
);


ALTER TABLE public._sqlx_migrations OWNER TO bread_bot;

--
-- Name: alembic_version; Type: TABLE; Schema: public; Owner: bread_bot
--

CREATE TABLE public.alembic_version (
                                        version_num character varying(32) NOT NULL
);


ALTER TABLE public.alembic_version OWNER TO bread_bot;

--
-- Name: answer_entities; Type: TABLE; Schema: public; Owner: bread_bot
--

CREATE TABLE public.answer_entities (
                                        is_active boolean NOT NULL,
                                        id integer NOT NULL,
                                        created_at timestamp without time zone NOT NULL,
                                        updated_at timestamp without time zone NOT NULL,
                                        key character varying(255) NOT NULL,
                                        value text NOT NULL,
                                        reaction_type public.answerentitytypesenum NOT NULL,
                                        content_type public.answerentitycontenttypesenum NOT NULL,
                                        description text,
                                        pack_id integer,
                                        file_unique_id character varying(255)
);


ALTER TABLE public.answer_entities OWNER TO bread_bot;

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


ALTER TABLE public.answer_entities_id_seq OWNER TO bread_bot;

--
-- Name: answer_entities_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: bread_bot
--

ALTER SEQUENCE public.answer_entities_id_seq OWNED BY public.answer_entities.id;


--
-- Name: answer_packs; Type: TABLE; Schema: public; Owner: bread_bot
--

CREATE TABLE public.answer_packs (
                                     is_active boolean NOT NULL,
                                     id integer NOT NULL,
                                     created_at timestamp without time zone NOT NULL,
                                     updated_at timestamp without time zone NOT NULL,
                                     name character varying(255) NOT NULL,
                                     is_private boolean NOT NULL,
                                     author integer,
                                     answer_chance smallint NOT NULL
);


ALTER TABLE public.answer_packs OWNER TO bread_bot;

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


ALTER TABLE public.answer_packs_id_seq OWNER TO bread_bot;

--
-- Name: answer_packs_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: bread_bot
--

ALTER SEQUENCE public.answer_packs_id_seq OWNED BY public.answer_packs.id;


--
-- Name: answer_packs_to_chats; Type: TABLE; Schema: public; Owner: bread_bot
--

CREATE TABLE public.answer_packs_to_chats (
                                              is_active boolean NOT NULL,
                                              id integer NOT NULL,
                                              created_at timestamp without time zone NOT NULL,
                                              updated_at timestamp without time zone NOT NULL,
                                              pack_id integer,
                                              chat_id integer
);


ALTER TABLE public.answer_packs_to_chats OWNER TO bread_bot;

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


ALTER TABLE public.answer_packs_to_chats_id_seq OWNER TO bread_bot;

--
-- Name: answer_packs_to_chats_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: bread_bot
--

ALTER SEQUENCE public.answer_packs_to_chats_id_seq OWNED BY public.answer_packs_to_chats.id;


--
-- Name: chats; Type: TABLE; Schema: public; Owner: bread_bot
--

CREATE TABLE public.chats (
                              is_active boolean NOT NULL,
                              id integer NOT NULL,
                              created_at timestamp without time zone NOT NULL,
                              updated_at timestamp without time zone NOT NULL,
                              chat_id bigint NOT NULL,
                              name character varying(255),
                              morph_answer_chance smallint NOT NULL,
                              is_openai_enabled boolean NOT NULL
);


ALTER TABLE public.chats OWNER TO bread_bot;

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


ALTER TABLE public.chats_id_seq OWNER TO bread_bot;

--
-- Name: chats_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: bread_bot
--

ALTER SEQUENCE public.chats_id_seq OWNED BY public.chats.id;


--
-- Name: chats_to_members; Type: TABLE; Schema: public; Owner: bread_bot
--

CREATE TABLE public.chats_to_members (
                                         is_active boolean NOT NULL,
                                         id integer NOT NULL,
                                         created_at timestamp without time zone NOT NULL,
                                         updated_at timestamp without time zone NOT NULL,
                                         member_id integer,
                                         chat_id integer
);


ALTER TABLE public.chats_to_members OWNER TO bread_bot;

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


ALTER TABLE public.chats_to_members_id_seq OWNER TO bread_bot;

--
-- Name: chats_to_members_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: bread_bot
--

ALTER SEQUENCE public.chats_to_members_id_seq OWNED BY public.chats_to_members.id;


--
-- Name: dictionary_entities; Type: TABLE; Schema: public; Owner: bread_bot
--

CREATE TABLE public.dictionary_entities (
                                            is_active boolean NOT NULL,
                                            id integer NOT NULL,
                                            created_at timestamp without time zone NOT NULL,
                                            updated_at timestamp without time zone NOT NULL,
                                            value text NOT NULL,
                                            chat_id integer
);


ALTER TABLE public.dictionary_entities OWNER TO bread_bot;

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


ALTER TABLE public.dictionary_entities_id_seq OWNER TO bread_bot;

--
-- Name: dictionary_entities_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: bread_bot
--

ALTER SEQUENCE public.dictionary_entities_id_seq OWNED BY public.dictionary_entities.id;


--
-- Name: members; Type: TABLE; Schema: public; Owner: bread_bot
--

CREATE TABLE public.members (
                                is_active boolean NOT NULL,
                                id integer NOT NULL,
                                created_at timestamp without time zone NOT NULL,
                                updated_at timestamp without time zone NOT NULL,
                                username character varying(255) NOT NULL,
                                first_name character varying(255),
                                last_name character varying(255),
                                is_bot boolean,
                                member_id bigint NOT NULL
);


ALTER TABLE public.members OWNER TO bread_bot;

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


ALTER TABLE public.members_id_seq OWNER TO bread_bot;

--
-- Name: members_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: bread_bot
--

ALTER SEQUENCE public.members_id_seq OWNED BY public.members.id;


--
-- Name: procrastinate_events; Type: TABLE; Schema: public; Owner: bread_bot
--

CREATE TABLE public.procrastinate_events (
                                             id bigint NOT NULL,
                                             job_id integer NOT NULL,
                                             type public.procrastinate_job_event_type,
                                             at timestamp with time zone DEFAULT now()
);


ALTER TABLE public.procrastinate_events OWNER TO bread_bot;

--
-- Name: procrastinate_events_id_seq; Type: SEQUENCE; Schema: public; Owner: bread_bot
--

CREATE SEQUENCE public.procrastinate_events_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.procrastinate_events_id_seq OWNER TO bread_bot;

--
-- Name: procrastinate_events_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: bread_bot
--

ALTER SEQUENCE public.procrastinate_events_id_seq OWNED BY public.procrastinate_events.id;


--
-- Name: procrastinate_jobs_id_seq; Type: SEQUENCE; Schema: public; Owner: bread_bot
--

CREATE SEQUENCE public.procrastinate_jobs_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.procrastinate_jobs_id_seq OWNER TO bread_bot;

--
-- Name: procrastinate_jobs_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: bread_bot
--

ALTER SEQUENCE public.procrastinate_jobs_id_seq OWNED BY public.procrastinate_jobs.id;


--
-- Name: procrastinate_periodic_defers; Type: TABLE; Schema: public; Owner: bread_bot
--

CREATE TABLE public.procrastinate_periodic_defers (
                                                      id bigint NOT NULL,
                                                      task_name character varying(128) NOT NULL,
                                                      defer_timestamp bigint,
                                                      job_id bigint,
                                                      queue_name character varying(128),
                                                      periodic_id character varying(128) DEFAULT ''::character varying NOT NULL
);


ALTER TABLE public.procrastinate_periodic_defers OWNER TO bread_bot;

--
-- Name: procrastinate_periodic_defers_id_seq; Type: SEQUENCE; Schema: public; Owner: bread_bot
--

CREATE SEQUENCE public.procrastinate_periodic_defers_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.procrastinate_periodic_defers_id_seq OWNER TO bread_bot;

--
-- Name: procrastinate_periodic_defers_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: bread_bot
--

ALTER SEQUENCE public.procrastinate_periodic_defers_id_seq OWNED BY public.procrastinate_periodic_defers.id;


--
-- Name: users; Type: TABLE; Schema: public; Owner: bread_bot
--

CREATE TABLE public.users (
                              is_active boolean NOT NULL,
                              id integer NOT NULL,
                              created_at timestamp without time zone NOT NULL,
                              updated_at timestamp without time zone NOT NULL,
                              username character varying(255) NOT NULL,
                              first_name character varying(255),
                              surname character varying(255),
                              email character varying(255) NOT NULL,
                              hashed_password character varying NOT NULL,
                              is_admin boolean NOT NULL
);


ALTER TABLE public.users OWNER TO bread_bot;

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


ALTER TABLE public.users_id_seq OWNER TO bread_bot;

--
-- Name: users_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: bread_bot
--

ALTER SEQUENCE public.users_id_seq OWNED BY public.users.id;


--
-- Name: answer_entities id; Type: DEFAULT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.answer_entities ALTER COLUMN id SET DEFAULT nextval('public.answer_entities_id_seq'::regclass);


--
-- Name: answer_packs id; Type: DEFAULT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.answer_packs ALTER COLUMN id SET DEFAULT nextval('public.answer_packs_id_seq'::regclass);


--
-- Name: answer_packs_to_chats id; Type: DEFAULT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.answer_packs_to_chats ALTER COLUMN id SET DEFAULT nextval('public.answer_packs_to_chats_id_seq'::regclass);


--
-- Name: chats id; Type: DEFAULT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.chats ALTER COLUMN id SET DEFAULT nextval('public.chats_id_seq'::regclass);


--
-- Name: chats_to_members id; Type: DEFAULT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.chats_to_members ALTER COLUMN id SET DEFAULT nextval('public.chats_to_members_id_seq'::regclass);


--
-- Name: dictionary_entities id; Type: DEFAULT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.dictionary_entities ALTER COLUMN id SET DEFAULT nextval('public.dictionary_entities_id_seq'::regclass);


--
-- Name: members id; Type: DEFAULT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.members ALTER COLUMN id SET DEFAULT nextval('public.members_id_seq'::regclass);


--
-- Name: procrastinate_events id; Type: DEFAULT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.procrastinate_events ALTER COLUMN id SET DEFAULT nextval('public.procrastinate_events_id_seq'::regclass);


--
-- Name: procrastinate_jobs id; Type: DEFAULT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.procrastinate_jobs ALTER COLUMN id SET DEFAULT nextval('public.procrastinate_jobs_id_seq'::regclass);


--
-- Name: procrastinate_periodic_defers id; Type: DEFAULT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.procrastinate_periodic_defers ALTER COLUMN id SET DEFAULT nextval('public.procrastinate_periodic_defers_id_seq'::regclass);


--
-- Name: users id; Type: DEFAULT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.users ALTER COLUMN id SET DEFAULT nextval('public.users_id_seq'::regclass);


--
-- Name: _sqlx_migrations _sqlx_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public._sqlx_migrations
    ADD CONSTRAINT _sqlx_migrations_pkey PRIMARY KEY (version);


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
-- Name: members members_username_key; Type: CONSTRAINT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.members
    ADD CONSTRAINT members_username_key UNIQUE (username);


--
-- Name: procrastinate_events procrastinate_events_pkey; Type: CONSTRAINT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.procrastinate_events
    ADD CONSTRAINT procrastinate_events_pkey PRIMARY KEY (id);


--
-- Name: procrastinate_jobs procrastinate_jobs_pkey; Type: CONSTRAINT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.procrastinate_jobs
    ADD CONSTRAINT procrastinate_jobs_pkey PRIMARY KEY (id);


--
-- Name: procrastinate_periodic_defers procrastinate_periodic_defers_pkey; Type: CONSTRAINT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.procrastinate_periodic_defers
    ADD CONSTRAINT procrastinate_periodic_defers_pkey PRIMARY KEY (id);


--
-- Name: procrastinate_periodic_defers procrastinate_periodic_defers_unique; Type: CONSTRAINT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.procrastinate_periodic_defers
    ADD CONSTRAINT procrastinate_periodic_defers_unique UNIQUE (task_name, periodic_id, defer_timestamp);


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
-- Name: procrastinate_events_job_id_fkey; Type: INDEX; Schema: public; Owner: bread_bot
--

CREATE INDEX procrastinate_events_job_id_fkey ON public.procrastinate_events USING btree (job_id);


--
-- Name: procrastinate_jobs_id_lock_idx; Type: INDEX; Schema: public; Owner: bread_bot
--

CREATE INDEX procrastinate_jobs_id_lock_idx ON public.procrastinate_jobs USING btree (id, lock) WHERE (status = ANY (ARRAY['todo'::public.procrastinate_job_status, 'doing'::public.procrastinate_job_status]));


--
-- Name: procrastinate_jobs_lock_idx; Type: INDEX; Schema: public; Owner: bread_bot
--

CREATE UNIQUE INDEX procrastinate_jobs_lock_idx ON public.procrastinate_jobs USING btree (lock) WHERE (status = 'doing'::public.procrastinate_job_status);


--
-- Name: procrastinate_jobs_queue_name_idx; Type: INDEX; Schema: public; Owner: bread_bot
--

CREATE INDEX procrastinate_jobs_queue_name_idx ON public.procrastinate_jobs USING btree (queue_name);


--
-- Name: procrastinate_jobs_queueing_lock_idx; Type: INDEX; Schema: public; Owner: bread_bot
--

CREATE UNIQUE INDEX procrastinate_jobs_queueing_lock_idx ON public.procrastinate_jobs USING btree (queueing_lock) WHERE (status = 'todo'::public.procrastinate_job_status);


--
-- Name: procrastinate_periodic_defers_job_id_fkey; Type: INDEX; Schema: public; Owner: bread_bot
--

CREATE INDEX procrastinate_periodic_defers_job_id_fkey ON public.procrastinate_periodic_defers USING btree (job_id);


--
-- Name: procrastinate_jobs procrastinate_jobs_notify_queue; Type: TRIGGER; Schema: public; Owner: bread_bot
--

CREATE TRIGGER procrastinate_jobs_notify_queue AFTER INSERT ON public.procrastinate_jobs FOR EACH ROW WHEN ((new.status = 'todo'::public.procrastinate_job_status)) EXECUTE FUNCTION public.procrastinate_notify_queue();


--
-- Name: procrastinate_jobs procrastinate_trigger_delete_jobs; Type: TRIGGER; Schema: public; Owner: bread_bot
--

CREATE TRIGGER procrastinate_trigger_delete_jobs BEFORE DELETE ON public.procrastinate_jobs FOR EACH ROW EXECUTE FUNCTION public.procrastinate_unlink_periodic_defers();


--
-- Name: procrastinate_jobs procrastinate_trigger_scheduled_events; Type: TRIGGER; Schema: public; Owner: bread_bot
--

CREATE TRIGGER procrastinate_trigger_scheduled_events AFTER INSERT OR UPDATE ON public.procrastinate_jobs FOR EACH ROW WHEN (((new.scheduled_at IS NOT NULL) AND (new.status = 'todo'::public.procrastinate_job_status))) EXECUTE FUNCTION public.procrastinate_trigger_scheduled_events_procedure();


--
-- Name: procrastinate_jobs procrastinate_trigger_status_events_insert; Type: TRIGGER; Schema: public; Owner: bread_bot
--

CREATE TRIGGER procrastinate_trigger_status_events_insert AFTER INSERT ON public.procrastinate_jobs FOR EACH ROW WHEN ((new.status = 'todo'::public.procrastinate_job_status)) EXECUTE FUNCTION public.procrastinate_trigger_status_events_procedure_insert();


--
-- Name: procrastinate_jobs procrastinate_trigger_status_events_update; Type: TRIGGER; Schema: public; Owner: bread_bot
--

CREATE TRIGGER procrastinate_trigger_status_events_update AFTER UPDATE OF status ON public.procrastinate_jobs FOR EACH ROW EXECUTE FUNCTION public.procrastinate_trigger_status_events_procedure_update();


--
-- Name: answer_entities answer_entities_pack_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.answer_entities
    ADD CONSTRAINT answer_entities_pack_id_fkey FOREIGN KEY (pack_id) REFERENCES public.answer_packs(id) ON DELETE CASCADE;


--
-- Name: answer_packs answer_packs_author_fkey; Type: FK CONSTRAINT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.answer_packs
    ADD CONSTRAINT answer_packs_author_fkey FOREIGN KEY (author) REFERENCES public.members(id) ON DELETE CASCADE;


--
-- Name: answer_packs_to_chats answer_packs_to_chats_chat_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.answer_packs_to_chats
    ADD CONSTRAINT answer_packs_to_chats_chat_id_fkey FOREIGN KEY (chat_id) REFERENCES public.chats(id) ON DELETE CASCADE;


--
-- Name: answer_packs_to_chats answer_packs_to_chats_pack_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.answer_packs_to_chats
    ADD CONSTRAINT answer_packs_to_chats_pack_id_fkey FOREIGN KEY (pack_id) REFERENCES public.answer_packs(id) ON DELETE CASCADE;


--
-- Name: chats_to_members chats_to_members_chat_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.chats_to_members
    ADD CONSTRAINT chats_to_members_chat_id_fkey FOREIGN KEY (chat_id) REFERENCES public.chats(id) ON DELETE CASCADE;


--
-- Name: chats_to_members chats_to_members_member_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.chats_to_members
    ADD CONSTRAINT chats_to_members_member_id_fkey FOREIGN KEY (member_id) REFERENCES public.members(id) ON DELETE CASCADE;


--
-- Name: dictionary_entities dictionary_entities_chat_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.dictionary_entities
    ADD CONSTRAINT dictionary_entities_chat_id_fkey FOREIGN KEY (chat_id) REFERENCES public.chats(id) ON DELETE CASCADE;


--
-- Name: procrastinate_events procrastinate_events_job_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.procrastinate_events
    ADD CONSTRAINT procrastinate_events_job_id_fkey FOREIGN KEY (job_id) REFERENCES public.procrastinate_jobs(id) ON DELETE CASCADE;


--
-- Name: procrastinate_periodic_defers procrastinate_periodic_defers_job_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: bread_bot
--

ALTER TABLE ONLY public.procrastinate_periodic_defers
    ADD CONSTRAINT procrastinate_periodic_defers_job_id_fkey FOREIGN KEY (job_id) REFERENCES public.procrastinate_jobs(id);


--
-- PostgreSQL database dump complete
--

