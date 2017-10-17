--
-- PostgreSQL database dump
--

-- Dumped from database version 10.0
-- Dumped by pg_dump version 10.0

-- Started on 2017-10-17 15:51:00

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
-- TOC entry 2811 (class 0 OID 16407)
-- Dependencies: 199
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: postgres
--

INSERT INTO users (username, display_name, email, show_email, admin, joined, userid, password) VALUES ('alex', 'Pawb', 'alex@fldcr.com', true, true, '2017-10-17 13:20:00-05', 3, '4135aa9dc1b842a653dea846903ddb95bfb8c5a10c504a7fa16e10bc31d1fdf0');
INSERT INTO users (username, display_name, email, show_email, admin, joined, userid, password) VALUES ('jason', 'Jason Smith', 'jesmith@blueplanetsolutions.net', true, true, '2017-10-17 16:00:00-05', 5, '06b9a6eacd7a77b9361123fd19776455eb16b9c83426a1abbf514a414792b73f');
INSERT INTO users (username, display_name, email, show_email, admin, joined, userid, password) VALUES ('andrew', 'Andrew Prindle', 'prindle.andrew@gmail.com', true, true, '2017-10-17 15:27:00-05', 1, 'd979885447a413abb6d606a5d0f45c3b7809e6fde2c83f0df3426f1fc9bfed97');
INSERT INTO users (username, display_name, email, show_email, admin, joined, userid, password) VALUES ('antonio', 'Antonio', 'anto91guzman@hotmail.com', true, true, '2017-10-17 17:00:00-05', 8, '4ee3679892e6ac5a5b513eba7fd529d363d7a96508421c5dbd44b01b349cf514');


--
-- TOC entry 2819 (class 0 OID 0)
-- Dependencies: 200
-- Name: users_userid_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('users_userid_seq', 8, true);


-- Completed on 2017-10-17 15:51:00

--
-- PostgreSQL database dump complete
--

