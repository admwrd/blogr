--
-- PostgreSQL database dump
--

-- Dumped from database version 10.0
-- Dumped by pg_dump version 10.0

-- Started on 2017-10-27 05:11:10

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SET check_function_bodies = false;
SET client_min_messages = warning;
SET row_security = off;

SET search_path = public, pg_catalog;

SET default_tablespace = '';

SET default_with_oids = false;

--
-- TOC entry 198 (class 1259 OID 16471)
-- Name: articles; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE articles (
    aid oid NOT NULL,
    title character varying NOT NULL,
    posted timestamp without time zone NOT NULL,
    body text NOT NULL,
    description character varying,
    tag2 character varying,
    tag character varying[]
);


ALTER TABLE articles OWNER TO postgres;

--
-- TOC entry 197 (class 1259 OID 16469)
-- Name: articles_aid_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE articles_aid_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE articles_aid_seq OWNER TO postgres;

--
-- TOC entry 2804 (class 0 OID 0)
-- Dependencies: 197
-- Name: articles_aid_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE articles_aid_seq OWNED BY articles.aid;


--
-- TOC entry 2674 (class 2604 OID 16482)
-- Name: articles aid; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY articles ALTER COLUMN aid SET DEFAULT nextval('articles_aid_seq'::regclass);


--
-- TOC entry 2799 (class 0 OID 16471)
-- Dependencies: 198
-- Data for Name: articles; Type: TABLE DATA; Schema: public; Owner: postgres
--

INSERT INTO articles (aid, title, posted, body, description, tag2, tag) VALUES (2, 'my insert title', '2017-10-20 13:45:00', 'this is a body', NULL, '{"\"awesome ness\"",cool,article}', '{article,lipsum,admin}');
INSERT INTO articles (aid, title, posted, body, description, tag2, tag) VALUES (1, 'An Awesome Article', '2017-10-19 14:00:00', 'This is the contents of a very very awesome article.', NULL, '{"\"awesome ness\"",cool,article,admin}', '{article,cool}');
INSERT INTO articles (aid, title, posted, body, description, tag2, tag) VALUES (4, 'I+submitted+this', '2017-10-20 13:57:50.204625', 'This+is+some+text+I+came+up+with+for+this+submitted+article.', NULL, '{article,admin}', '{cool,article,code}');
INSERT INTO articles (aid, title, posted, body, description, tag2, tag) VALUES (12, 'Hello handlebars', '2017-10-26 00:14:01.313954', 'Handlebars!!

Nunc condimentum rhoncus justo, eu vestibulum orci lobortis et. Nam finibus nisi id dui finibus, at egestas ipsum dignissim. Nulla sodales urna at condimentum luctus. Mauris interdum quam ut purus ornare, sed tempus justo consectetur. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Maecenas sit amet nulla libero. Sed convallis pulvinar viverra. Nunc a elit bibendum, lobortis ante non, posuere est.

Phasellus quis eros lacus. Ut tincidunt sit amet mi non facilisis. Praesent feugiat ante orci, vel gravida risus consectetur elementum. Curabitur volutpat magna a semper imperdiet. Nulla laoreet auctor dignissim. Nunc at ante a ligula luctus viverra vitae eget mi. Quisque congue ligula leo, sit amet congue odio ultricies vel. Nunc bibendum ex sed diam dapibus fringilla. In lacus lacus, facilisis sed consequat pretium, placerat quis enim. Cras vel pulvinar dolor, in faucibus massa. Praesent tristique vulputate purus. Duis ullamcorper elit sed lacus dapibus molestie. Vestibulum convallis leo volutpat, bibendum ipsum sit amet, tincidunt metus. Maecenas fringilla dapibus massa, ut elementum est luctus quis. Ut in feugiat orci, in tincidunt dolor.

Fusce consectetur sollicitudin magna, vel hendrerit massa bibendum in. Nulla facilisi. Mauris sagittis euismod tortor, fermentum imperdiet neque porta at. Nunc ac quam et libero ullamcorper accumsan. Maecenas pretium semper pulvinar. Aliquam efficitur a sapien a ultrices. Proin vulputate elit sapien, mattis fringilla augue varius vel. Etiam sit amet arcu risus. Nullam laoreet erat nisi. Maecenas eget sapien eu erat feugiat elementum sed id risus. Integer eget mi massa.', 'I switched the blog from using custom templating functions to using a custom function as a wrapper for all the information', '{lorem,ipsum,cool,code,programming,"\"awesome ness\"",admin}', '{cool,code,programming,admin}');
INSERT INTO articles (aid, title, posted, body, description, tag2, tag) VALUES (11, 'Tag Array Article 2', '2017-10-23 18:19:18.999871', 'Sed quis ligula quis massa hendrerit tristique. Nulla consectetur tincidunt tellus. Quisque mattis libero in neque consequat, sed tincidunt risus consequat. Sed tincidunt orci odio, vitae iaculis diam congue ut. Donec quis urna justo. Etiam finibus sit amet risus at convallis. Mauris tortor libero, euismod sit amet est a, luctus vulputate sapien. Nulla dictum molestie enim vel rhoncus. Suspendisse sagittis tincidunt justo. Cras pellentesque nisl elit, non luctus velit cursus nec. Aliquam elit purus, interdum vitae suscipit sed, semper vel mauris. Nullam dolor ipsum, suscipit eget ex in, viverra pulvinar nulla. Cras sit amet nibh suscipit, egestas eros ac, tincidunt nisi.

Pellentesque sollicitudin massa id odio vulputate dapibus. Fusce pharetra maximus dictum. Maecenas dapibus pharetra metus. In rhoncus turpis venenatis lobortis tristique. Nulla eget interdum sem. Donec sed egestas sapien. Fusce ultrices sodales ex condimentum imperdiet. Suspendisse porta tellus in enim posuere vulputate. Cras rutrum massa ut dolor efficitur, sed euismod augue aliquam.

Fusce lacinia gravida augue et rhoncus. Aenean eleifend nulla eget nisl venenatis, id commodo mi auctor. Suspendisse at dolor est. Nullam consequat venenatis mollis. Vestibulum ultricies et nisi sit amet tempus. Sed porta turpis ut dolor lobortis, ultricies cursus ipsum iaculis. Phasellus non vulputate augue.

Ut varius id lacus ac dictum. Aenean rhoncus fermentum sollicitudin. Duis in quam eget diam egestas condimentum. Sed vel mattis dui. Nullam vel erat ipsum. Sed vel molestie dui, at tristique nisi. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla non orci in magna pellentesque iaculis. Mauris at est nisl. In quam metus, luctus non tempus eget, sagittis non sem. Donec tempor egestas sapien vel dignissim. Ut posuere velit eget risus tincidunt laoreet. Aliquam lorem lectus, ultricies mollis ultricies at, pellentesque et ex. Vestibulum pretium nisl in neque suscipit fringilla in vitae erat. Donec fermentum, mauris non ultricies venenatis, felis diam efficitur arcu, quis vestibulum sapien enim eu odio.

Donec semper malesuada mattis. Donec vitae egestas lacus. Vivamus aliquet, odio et cursus molestie, mi nisi vulputate purus, at auctor nibh ante ac risus. Cras luctus vehicula quam ac rhoncus. Etiam placerat, quam venenatis consequat finibus, leo quam tempus magna, quis congue nulla nisi vitae mauris. Fusce mattis egestas lobortis. Integer a enim nunc. Nunc sagittis ligula urna, et gravida libero scelerisque at.', 'This is a short description of the tag array article 2.  Again just a short description.', '{admin,cool,"\"awesome ness\"",code,lipsum,tags}', '{code,admin,lipsum,"''awesome ness''"}');
INSERT INTO articles (aid, title, posted, body, description, tag2, tag) VALUES (3, 'I+submitted+this', '2017-10-20 13:55:06.81759', 'This+is+some+text+I+came+up+with+for+this+submitted+article.', NULL, '{awesome,cool,admin}', '{article,lipsum,admin,cool,"''awesome ness''"}');
INSERT INTO articles (aid, title, posted, body, description, tag2, tag) VALUES (6, 'this is an admin article', '2017-10-20 17:33:39.989516', 'This article was submitted by an admin', NULL, '{"\"awesome ness\"",cool,article,admin}', '{code,programming,lipsum,"''awesome ness''"}');
INSERT INTO articles (aid, title, posted, body, description, tag2, tag) VALUES (5, 'I submitted this', '2017-10-20 17:15:02.170895', 'This is some text I came up with for this submitted article.', NULL, '{"\"awesome ness\"",cool,article,admin}', '{"''awesome ness''",code,admin,lipsum,cool}');
INSERT INTO articles (aid, title, posted, body, description, tag2, tag) VALUES (8, 'Another article!', '2017-10-23 11:40:52.832981', 'Maecenas consectetur dui molestie enim tempor sodales in sed justo. Suspendisse commodo id turpis non iaculis. Nulla tempor auctor suscipit. Donec sed mi pharetra, fringilla erat et, aliquam nisi. Ut id metus in ex rhoncus porta. In et feugiat tellus. Fusce diam risus, finibus vitae dolor at, ornare lobortis nisi. Cras mattis diam in nunc posuere, euismod dapibus mi pharetra. Quisque magna est, porttitor non nulla eget, sodales tempus sem.

Nulla luctus dignissim libero, viverra interdum quam venenatis sed. Vivamus id cursus urna. Donec rutrum pulvinar nisl vel consequat. Sed eget lacus id elit convallis venenatis. Maecenas sit amet mollis enim. Nam iaculis ex sit amet metus fermentum, vitae dignissim neque suscipit. Quisque eu elit lorem. Maecenas ut tincidunt sem. Praesent quis velit in nulla hendrerit mattis ut et velit.

Cras varius urna interdum, aliquet lacus ut, egestas ex. Etiam sit amet nulla sapien. Aliquam tortor lectus, hendrerit non facilisis vitae, finibus et sem. Maecenas nec mi interdum, molestie turpis eu, porttitor tellus. Donec vel ex tortor. Integer ligula dui, sagittis hendrerit posuere sit amet, rhoncus eu velit. Curabitur ultrices est sed purus rutrum, at volutpat augue laoreet.

Nunc varius leo vitae tellus consequat scelerisque. Nunc nulla lacus, aliquet vitae vestibulum sed, eleifend nec est. Donec sit amet tellus euismod, tempor sem non, iaculis velit. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Duis quam diam, interdum id hendrerit eu, fermentum vitae nisi. Nam tincidunt urna sit amet justo pharetra porttitor. Etiam accumsan fringilla dolor, ac condimentum massa ullamcorper ut. Fusce pretium ipsum in ornare mollis. Praesent consectetur ligula eget urna fermentum efficitur. Vestibulum ut sem at neque ultrices posuere.

Cras eleifend metus ac auctor molestie. Mauris placerat ante ex, non vestibulum justo tristique id. Mauris ac vestibulum felis. Sed placerat lorem eget risus elementum pretium. Nam tristique felis a purus sagittis ornare. Nam non congue velit, non facilisis est. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Integer congue vel lorem in ornare. Ut eu ultricies risus, ut vestibulum diam. Cras efficitur tempor odio vitae ultrices. Maecenas at vehicula metus. Nam a vestibulum risus.', NULL, '{"\"awesome ness\"",cool,article,admin,lorem}', '{lipsum,awesome,cool,code,programming}');
INSERT INTO articles (aid, title, posted, body, description, tag2, tag) VALUES (7, 'New article!', '2017-10-23 11:34:44.099342', 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc pretium scelerisque nisl nec consectetur. Ut venenatis iaculis sem sit amet porttitor. In tincidunt molestie faucibus. Vivamus tristique suscipit varius. Phasellus gravida justo eu risus varius mollis. Mauris finibus quam in gravida pulvinar. In luctus fringilla nisl vel congue. Nulla tincidunt odio ac sapien scelerisque, at fermentum erat placerat. Praesent sed lorem eget diam faucibus ullamcorper. Aenean nec urna faucibus, porttitor eros eget, varius leo. Suspendisse vitae interdum nulla. Aenean egestas enim vel justo ultricies commodo.

Fusce tortor est, scelerisque at tempus at, malesuada id mauris. Vestibulum scelerisque tellus ac eros tristique, eu eleifend arcu imperdiet. Cras quis sem commodo lacus feugiat egestas a quis lectus. Fusce finibus dolor risus. Pellentesque aliquet, erat in fringilla molestie, leo elit eleifend felis, at mollis leo justo ac nisi. Mauris posuere scelerisque tortor, id convallis arcu accumsan a. Nam semper ligula id elit auctor aliquam eget eget felis. Maecenas pretium mauris nec ligula rutrum pellentesque. Fusce eget nulla eu nibh scelerisque interdum in eu erat. Suspendisse eget tellus vel orci tincidunt pellentesque. Nam non lorem euismod, semper nunc eu, tincidunt nibh. Vivamus sagittis est vitae enim posuere porta. Fusce mollis, arcu ac porta semper, elit lectus ornare mauris, sit amet mollis nunc enim non lorem. Etiam congue condimentum neque id congue. Proin orci ex, rhoncus quis aliquam eget, dapibus non lacus.

Integer accumsan turpis at eros ullamcorper, nec dapibus nisl fringilla. Suspendisse dignissim nibh eget lectus convallis, id semper purus suscipit. Aenean volutpat arcu eu enim egestas, ut ornare nibh euismod. Sed lobortis nulla eu porttitor euismod. Vestibulum tristique accumsan risus ut varius. Sed sapien sem, rhoncus ut vestibulum in, efficitur nec ante. Nam sapien augue, porttitor vitae mi eleifend, euismod dictum erat. Nulla vehicula aliquam elementum. Vestibulum elementum aliquam aliquam.

Maecenas vestibulum turpis sit amet lectus finibus, id semper metus venenatis. Nullam congue commodo magna. Aenean sit amet leo vitae elit eleifend condimentum. Etiam quis ex sem. In et erat eros. Pellentesque cursus arcu at ex placerat, sit amet aliquam elit lobortis. Morbi vitae mi dui. Praesent laoreet ex nec lobortis imperdiet. Maecenas viverra dapibus erat, non finibus felis molestie sit amet. Fusce euismod orci et purus ornare finibus. Sed rutrum imperdiet metus, at lacinia ligula rutrum sed. Sed tincidunt sodales nisi, at pellentesque velit. Suspendisse vel fermentum sapien. Nulla eu lobortis orci. Cras accumsan et libero eu ultricies. Pellentesque cursus sagittis augue, et accumsan quam sagittis vel.

In commodo tellus turpis, ac semper dolor fringilla vel. Donec tempus, velit a finibus luctus, justo ante pretium nunc, et efficitur urna tortor ut est. Sed non gravida dui, sed auctor ex. Fusce cursus a urna id pretium. Maecenas molestie eu turpis vel mollis. Sed convallis massa fringilla nunc maximus sagittis. Etiam vel nibh tempor nunc placerat elementum a sed metus. Integer mollis scelerisque est, non tempus libero ullamcorper vel. Maecenas faucibus elit ante, id bibendum orci fermentum vitae. Nam in metus erat. Quisque iaculis lobortis augue eu convallis. Donec et neque vitae turpis convallis molestie vitae vel tortor. Suspendisse et tincidunt libero. Aenean rhoncus neque ut risus porttitor, quis posuere sapien egestas. Aliquam erat volutpat.', NULL, '{"\"awesome ness\"",cool,article,admin}', '{awesome,lipsum,admin}');
INSERT INTO articles (aid, title, posted, body, description, tag2, tag) VALUES (10, 'Tag Array Article', '2017-10-23 18:04:20.779566', 'Nullam sit amet interdum mauris, at fringilla urna. Fusce porta elit orci, at molestie augue pulvinar at. Vestibulum egestas lacus eget justo commodo, tempor rutrum odio molestie. Sed elit metus, blandit nec malesuada ac, elementum a turpis. Mauris sodales dolor vitae neque mollis, lacinia venenatis libero egestas. Sed nibh massa, commodo non pretium vitae, euismod eget ante. Suspendisse vestibulum et elit in vehicula.

In ultricies leo eget nibh pretium auctor. Nullam at lorem in nibh condimentum luctus nec vel leo. Nam vitae quam semper, ultrices magna ac, aliquam dolor. Mauris facilisis et nisi ut vehicula. Aenean dui neque, rhoncus a sem vel, laoreet sollicitudin turpis. Aliquam condimentum nec odio quis vulputate. Integer ut suscipit enim. Duis porta ante vel condimentum congue. Fusce rutrum augue a libero ultricies, quis ultrices risus consectetur. Nulla interdum lacinia est ac sodales. Mauris quis ligula quis ligula ultrices tristique a sit amet metus. Proin purus odio, dignissim non nisl quis, pretium aliquam nulla. In hac habitasse platea dictumst. Nam metus ligula, vehicula eu vestibulum vel, ultrices ut elit. Nam tellus ipsum, congue sed placerat quis, vestibulum quis eros.

Nam non vehicula lectus, tristique sagittis enim. Nam malesuada odio libero, sit amet suscipit magna dictum eget. Nunc convallis rhoncus nunc fermentum bibendum. Duis non lorem mi. Aliquam erat volutpat. Curabitur cursus, mi non euismod maximus, sem libero convallis arcu, vel aliquet nunc magna quis lorem. In rhoncus arcu enim, quis porta nibh ultrices non.

Morbi nec molestie justo. Duis vitae laoreet purus, vel dignissim lorem. Proin porttitor lacus semper ipsum egestas viverra. Praesent et accumsan libero. Cras id quam vel velit semper interdum non sed justo. Donec dictum libero ac velit vehicula placerat. Praesent vel egestas orci, quis semper odio. Donec vehicula lorem iaculis neque venenatis pulvinar. Vestibulum pretium consequat vestibulum. Donec lacus ex, elementum ac nisi at, consequat feugiat est. Aliquam nec gravida urna. Praesent ante ipsum, pellentesque eget sollicitudin convallis, facilisis sit amet neque. Phasellus blandit, enim et scelerisque aliquet, augue metus suscipit enim, commodo vehicula arcu dolor sed odio. Pellentesque mollis lectus sit amet lacinia condimentum. Nunc porta ante vel diam egestas, in maximus est vulputate.

Curabitur lobortis gravida quam, non feugiat quam. Aliquam sed massa at massa euismod elementum. Integer eros erat, dapibus vitae dictum quis, feugiat a felis. Donec aliquam massa at ultricies sollicitudin. Mauris quam justo, dapibus eleifend lectus nec, mattis fermentum justo. Etiam quis mauris sed ligula tempus sagittis eu id nunc. Integer imperdiet metus luctus gravida tincidunt. Duis vehicula mi sit amet nisl elementum lacinia. Nullam vel iaculis massa, sit amet dictum nisl.', 'Donec ultricies rhoncus massa, sed tristique est vehicula ac. Maecenas aliquam feugiat orci quis congue. Pellentesque interdum eros in ex imperdiet interdum. Nunc a fermentum felis. Nam posuere vehicula nulla, in porta est mollis nec. Etiam a nullam.', '{test,admin,cool,"\"awesome ness\"",code,tags}', '{lipsum,code,admin}');
INSERT INTO articles (aid, title, posted, body, description, tag2, tag) VALUES (9, 'Descriptive Article', '2017-10-23 16:37:48.479308', 'Maecenas vitae libero sit amet nisl blandit molestie eget et eros. Quisque turpis erat, convallis ac vulputate ut, placerat quis mi. Quisque ut laoreet magna. Nullam nec dolor ultrices, cursus diam et, dictum dui. Maecenas porta ipsum mi, quis placerat mi molestie nec. Integer id pretium orci. Pellentesque sit amet enim pulvinar, eleifend ligula sed, pulvinar velit. Nam vitae magna gravida, egestas arcu eu, tempor lorem. Nulla lacinia lobortis libero, convallis laoreet tellus.

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean tincidunt vestibulum tortor vitae consectetur. Etiam ac sem ultricies, placerat nibh eget, iaculis enim. Praesent congue porta neque eu elementum. Nunc varius metus ex, id finibus nibh bibendum et. Pellentesque a blandit felis. Morbi at nibh ut tortor semper tristique vitae ut orci. Aenean id laoreet lorem, a auctor nisi. Donec sed augue sed eros pretium dictum.

Aliquam cursus vitae diam non pellentesque. Fusce dolor mi, placerat sed nibh ut, maximus aliquam tellus. Maecenas mollis cursus fringilla. Vestibulum in orci ac nibh elementum egestas non viverra sem. Etiam justo nisi, condimentum eget tortor ac, eleifend semper leo. Fusce eu metus elit. Fusce commodo, risus nec porttitor condimentum, sapien quam porttitor lacus, maximus sagittis nulla dui ut nibh. Nam nisi felis, aliquam id lobortis eu, aliquet sed elit. Curabitur nisl dui, suscipit vel odio sed, vehicula placerat ex. Maecenas at vehicula odio. Donec laoreet vitae eros nec suscipit. Donec neque lacus, placerat et convallis sed, malesuada in diam. Nam in sodales neque, quis vehicula orci. Donec imperdiet neque arcu, vel fermentum magna consequat quis. Suspendisse potenti. Ut at orci in augue pretium blandit eu at nisi.

Aenean sagittis dolor ac felis porttitor, sit amet bibendum nisi aliquam. Etiam lobortis nunc et scelerisque euismod. Integer tristique quam in nulla pretium, eu viverra ex ornare. Duis a odio id lectus hendrerit sodales in non augue. Quisque iaculis posuere nibh, id feugiat ante. Fusce blandit est ac elit pharetra, nec rhoncus tellus fringilla. Pellentesque porttitor ultrices libero vel sollicitudin. Morbi quam ante, commodo a neque sed, sagittis vestibulum nisl. Donec pretium egestas turpis, id dignissim dui molestie ac. Nullam id tellus non mi dignissim fringilla quis in tortor. Nunc mollis arcu a elit eleifend, eu accumsan lorem dictum. Curabitur justo nisl, venenatis a lacus eu, dictum aliquet quam.

Phasellus ante nibh, efficitur interdum lorem eu, cursus porttitor purus. Fusce lacinia sed purus in semper. Pellentesque molestie facilisis commodo. Nulla maximus sollicitudin imperdiet. Sed vitae nisi vitae nunc egestas hendrerit ut nec lectus. Donec auctor facilisis tincidunt. Phasellus mattis turpis ac nunc mattis lobortis. Vivamus tortor purus, facilisis eget lectus eu, vehicula sodales neque.', ' Nunc ut molestie elit. Suspendisse tempus est quis leo elementum, eu ornare justo eleifend. Integer et massa vel erat maximus auctor. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Fusce eu pretium nunc. Sed euismod faucibus metus, at amet.', '{"\"awesome ness\"",cool,article,admin,programming,lipsum,code}', '{programming,code,cool}');


--
-- TOC entry 2805 (class 0 OID 0)
-- Dependencies: 197
-- Name: articles_aid_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('articles_aid_seq', 12, true);


--
-- TOC entry 2676 (class 2606 OID 16484)
-- Name: articles articles_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY articles
    ADD CONSTRAINT articles_pkey PRIMARY KEY (aid);


-- Completed on 2017-10-27 05:11:12

--
-- PostgreSQL database dump complete
--

