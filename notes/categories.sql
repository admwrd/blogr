--
-- PostgreSQL database dump
--

-- Dumped from database version 10.0
-- Dumped by pg_dump version 10.0

-- Started on 2017-10-17 15:53:32

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SET check_function_bodies = false;
SET client_min_messages = warning;
SET row_security = off;

SET search_path = public, pg_catalog;

--
-- TOC entry 2810 (class 0 OID 16396)
-- Dependencies: 197
-- Data for Name: categories; Type: TABLE DATA; Schema: public; Owner: postgres
--

INSERT INTO categories (name, catid) VALUES ('Rust', 1);
INSERT INTO categories (name, catid) VALUES ('Web Development', 2);
INSERT INTO categories (name, catid) VALUES ('Rust Rocket Web Framework', 3);
INSERT INTO categories (name, catid) VALUES ('Programming', 4);
INSERT INTO categories (name, catid) VALUES ('Tips & Tricks', 5);
INSERT INTO categories (name, catid) VALUES ('Concurrency', 6);
INSERT INTO categories (name, catid) VALUES ('Tutorials', 7);


--
-- TOC entry 2817 (class 0 OID 0)
-- Dependencies: 196
-- Name: categories_catid_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('categories_catid_seq', 7, true);


-- Completed on 2017-10-17 15:53:32

--
-- PostgreSQL database dump complete
--

