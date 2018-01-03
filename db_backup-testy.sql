--
-- PostgreSQL database dump
--

-- Dumped from database version 10.0
-- Dumped by pg_dump version 10.0

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SET check_function_bodies = false;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: blog; Type: DATABASE; Schema: -; Owner: -
--

CREATE DATABASE blog WITH TEMPLATE = template0 ENCODING = 'UTF8' LC_COLLATE = 'English_United States.1252' LC_CTYPE = 'English_United States.1252';


\connect blog

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SET check_function_bodies = false;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: blog; Type: COMMENT; Schema: -; Owner: -
--

COMMENT ON DATABASE blog IS 'Second attempt at a blog database.  More simplified.';


--
-- Name: plpgsql; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS plpgsql WITH SCHEMA pg_catalog;


--
-- Name: EXTENSION plpgsql; Type: COMMENT; Schema: -; Owner: -
--

COMMENT ON EXTENSION plpgsql IS 'PL/pgSQL procedural language';


--
-- Name: pgcrypto; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS pgcrypto WITH SCHEMA public;


--
-- Name: EXTENSION pgcrypto; Type: COMMENT; Schema: -; Owner: -
--

COMMENT ON EXTENSION pgcrypto IS 'cryptographic functions';


SET search_path = public, pg_catalog;

--
-- Name: array_unique(anyarray); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION array_unique(arr anyarray) RETURNS anyarray
    LANGUAGE sql
    AS $_$
    select array( select distinct unnest($1) )
$_$;


--
-- Name: description(integer, text, text); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION description(chars integer, body text, short text DEFAULT NULL::text) RETURNS text
    LANGUAGE plpgsql
    AS $$
-- AS 'function body text'
DECLARE
    rst text;
BEGIN

CASE WHEN (short) IS NOT NULL THEN rst:= short;
     ELSE rst:= LEFT(body, chars); END CASE;

-- CASE short WHEN NOT NULL THEN rst := short;
-- ELSE rst := LEFT(body, chars); END CASE;
return rst;
END
$$;


--
-- Name: fulltxt_articles_update(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION fulltxt_articles_update() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
  new.fulltxt := setweight(to_tsvector('pg_catalog.english', new.title), 'A') || 
		 setweight(to_tsvector('pg_catalog.english', coalesce(new.description,'')), 'B') || 
		 setweight(to_tsvector('pg_catalog.english', new.body), 'C');
  return new;
end
$$;


--
-- Name: proc_blog_users_insert(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION proc_blog_users_insert() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
    -- Hash the password with a newly generated salt
    -- crypt() will store the hash and salt (and the algorithm and iterations) in the column
    new.hash_salt := crypt(new.hash_salt, gen_salt('bf', 8));
  return new;
end
$$;


--
-- Name: proc_blog_users_update(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION proc_blog_users_update() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
  IF NEW.hash_salt IS NULL OR NEW.hash_salt = '' THEN
    new.hash_salt := old.hash_salt;
--     new.attempts := 99;
  ELSE 
    -- new.hash_salt := crypt(new.hash_salt, gen_salt('bf', 8));
    new.hash_salt := crypt(new.hash_salt, old.hash_salt);
--     new.attempts := 66;
  END IF;
  return new;
end
$$;


SET default_tablespace = '';

SET default_with_oids = false;

--
-- Name: articles; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE articles (
    aid oid NOT NULL,
    title character varying NOT NULL,
    posted timestamp without time zone NOT NULL,
    body text NOT NULL,
    description character varying,
    tag2 character varying,
    tag character varying[],
    fulltxt tsvector,
    author oid,
    markdown text
);


--
-- Name: articles_aid_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE articles_aid_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: articles_aid_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE articles_aid_seq OWNED BY articles.aid;


--
-- Name: users; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE users (
    userid oid NOT NULL,
    username character varying(30) NOT NULL,
    display character varying(60) NOT NULL,
    is_admin boolean NOT NULL,
    hash_salt text NOT NULL,
    attempts smallint DEFAULT 0 NOT NULL,
    lockout timestamp without time zone
);


--
-- Name: users_old; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE users_old (
    userid oid NOT NULL,
    username character varying(30) NOT NULL,
    display character varying(45),
    email character varying(60),
    password character varying(64) NOT NULL,
    is_admin boolean,
    is_public boolean
);


--
-- Name: users_userid_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE users_userid_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: users_userid_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE users_userid_seq OWNED BY users.userid;


--
-- Name: articles aid; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY articles ALTER COLUMN aid SET DEFAULT nextval('articles_aid_seq'::regclass);


--
-- Name: users userid; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY users ALTER COLUMN userid SET DEFAULT nextval('users_userid_seq'::regclass);


--
-- Data for Name: articles; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO articles (aid, title, posted, body, description, tag2, tag, fulltxt, author, markdown) VALUES (2, 'my insert title', '2017-10-20 13:45:00', 'this is a body', NULL, '{"\"awesome ness\"",cool,article}', '{article,lipsum,admin}', '''bodi'':7C ''insert'':2A ''titl'':3A', 1, NULL);
INSERT INTO articles (aid, title, posted, body, description, tag2, tag, fulltxt, author, markdown) VALUES (1, 'An Awesome Article', '2017-10-19 14:00:00', 'This is the contents of a very very awesome article.', NULL, '{"\"awesome ness\"",cool,article,admin}', '{article,cool}', '''articl'':3A,13C ''awesom'':2A,12C ''content'':7C', 1, NULL);
INSERT INTO articles (aid, title, posted, body, description, tag2, tag, fulltxt, author, markdown) VALUES (4, 'I+submitted+this', '2017-10-20 13:57:50.204625', 'This+is+some+text+I+came+up+with+for+this+submitted+article.', NULL, '{article,admin}', '{cool,article,code}', '''articl'':14C ''came'':8C ''submit'':2A,13C ''text'':6C', 1, NULL);
INSERT INTO articles (aid, title, posted, body, description, tag2, tag, fulltxt, author, markdown) VALUES (12, 'Hello handlebars', '2017-10-26 00:14:01.313954', 'Handlebars!!

Nunc condimentum rhoncus justo, eu vestibulum orci lobortis et. Nam finibus nisi id dui finibus, at egestas ipsum dignissim. Nulla sodales urna at condimentum luctus. Mauris interdum quam ut purus ornare, sed tempus justo consectetur. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Maecenas sit amet nulla libero. Sed convallis pulvinar viverra. Nunc a elit bibendum, lobortis ante non, posuere est.

Phasellus quis eros lacus. Ut tincidunt sit amet mi non facilisis. Praesent feugiat ante orci, vel gravida risus consectetur elementum. Curabitur volutpat magna a semper imperdiet. Nulla laoreet auctor dignissim. Nunc at ante a ligula luctus viverra vitae eget mi. Quisque congue ligula leo, sit amet congue odio ultricies vel. Nunc bibendum ex sed diam dapibus fringilla. In lacus lacus, facilisis sed consequat pretium, placerat quis enim. Cras vel pulvinar dolor, in faucibus massa. Praesent tristique vulputate purus. Duis ullamcorper elit sed lacus dapibus molestie. Vestibulum convallis leo volutpat, bibendum ipsum sit amet, tincidunt metus. Maecenas fringilla dapibus massa, ut elementum est luctus quis. Ut in feugiat orci, in tincidunt dolor.

Fusce consectetur sollicitudin magna, vel hendrerit massa bibendum in. Nulla facilisi. Mauris sagittis euismod tortor, fermentum imperdiet neque porta at. Nunc ac quam et libero ullamcorper accumsan. Maecenas pretium semper pulvinar. Aliquam efficitur a sapien a ultrices. Proin vulputate elit sapien, mattis fringilla augue varius vel. Etiam sit amet arcu risus. Nullam laoreet erat nisi. Maecenas eget sapien eu erat feugiat elementum sed id risus. Integer eget mi massa.', 'I switched the blog from using custom templating functions to using a custom function as a wrapper for all the information', '{lorem,ipsum,cool,code,programming,"\"awesome ness\"",admin}', '{cool,code,programming,admin}', '''ac'':222C ''accumsan'':227C ''aliquam'':232C ''amet'':74C,97C,135C,182C,249C ''ant'':86C,103C,122C ''arcu'':250C ''auctor'':118C ''augu'':244C ''bibendum'':84C,141C,179C,208C ''blog'':6B ''condimentum'':26C,48C ''congu'':131C,136C ''consectetur'':59C,108C,202C ''consequat'':152C ''conval'':78C,176C ''cras'':157C ''curabitur'':110C ''custom'':9B,15B ''dapibus'':145C,173C,187C ''diam'':144C ''dignissim'':43C,119C ''dis'':66C ''dolor'':160C,200C ''dui'':38C,168C ''efficitur'':233C ''egesta'':41C ''eget'':128C,257C,267C ''elementum'':109C,190C,262C ''elit'':83C,170C,240C ''enim'':156C ''erat'':254C,260C ''ero'':92C ''est'':89C,191C ''et'':33C,64C,224C ''etiam'':247C ''eu'':29C,259C ''euismod'':214C ''ex'':142C ''facilisi'':100C,150C,211C ''faucibus'':162C ''fermentum'':216C ''feugiat'':102C,196C,261C ''finibus'':35C,39C ''fringilla'':146C,186C,243C ''function'':11B,16B ''fusc'':201C ''gravida'':106C ''handlebar'':2A,24C ''hello'':1A ''hendrerit'':206C ''id'':37C,264C ''imperdiet'':115C,217C ''inform'':23B ''integ'':266C ''interdum'':51C ''ipsum'':42C,180C ''justo'':28C,58C ''lacus'':93C,148C,149C,172C ''laoreet'':117C,253C ''leo'':133C,177C ''libero'':76C,225C ''ligula'':124C,132C ''loborti'':32C,85C ''luctus'':49C,125C,192C ''maecena'':72C,185C,228C,256C ''magna'':112C,204C ''magni'':65C ''massa'':163C,188C,207C,269C ''matti'':242C ''mauri'':50C,212C ''metus'':184C ''mi'':98C,129C,268C ''molesti'':174C ''mont'':68C ''mus'':71C ''nam'':34C ''nascetur'':69C ''natoqu'':62C ''nequ'':218C ''nisi'':36C,255C ''non'':87C,99C ''nulla'':44C,75C,116C,210C ''nullam'':252C ''nunc'':25C,81C,120C,140C,221C ''odio'':137C ''orci'':31C,60C,104C,197C ''ornar'':55C ''parturi'':67C ''penatibus'':63C ''phasellus'':90C ''placerat'':154C ''porta'':219C ''posuer'':88C ''praesent'':101C,164C ''pretium'':153C,229C ''proin'':238C ''pulvinar'':79C,159C,231C ''purus'':54C,167C ''quam'':52C,223C ''qui'':91C,155C,193C ''quisqu'':130C ''rhoncus'':27C ''ridiculus'':70C ''risus'':107C,251C,265C ''sagitti'':213C ''sapien'':235C,241C,258C ''sed'':56C,77C,143C,151C,171C,263C ''semper'':114C,230C ''sit'':73C,96C,134C,181C,248C ''sodal'':45C ''sollicitudin'':203C ''switch'':4B ''templat'':10B ''tempus'':57C ''tincidunt'':95C,183C,199C ''tortor'':215C ''tristiqu'':165C ''ullamcorp'':169C,226C ''ultric'':237C ''ultrici'':138C ''urna'':46C ''use'':8B,13B ''ut'':53C,94C,189C,194C ''varius'':61C,245C ''vel'':105C,139C,158C,205C,246C ''vestibulum'':30C,175C ''vita'':127C ''viverra'':80C,126C ''volutpat'':111C,178C ''vulput'':166C,239C ''wrapper'':19B', 1, NULL);
INSERT INTO articles (aid, title, posted, body, description, tag2, tag, fulltxt, author, markdown) VALUES (11, 'Tag Array Article 2', '2017-10-23 18:19:18.999871', 'Sed quis ligula quis massa hendrerit tristique. Nulla consectetur tincidunt tellus. Quisque mattis libero in neque consequat, sed tincidunt risus consequat. Sed tincidunt orci odio, vitae iaculis diam congue ut. Donec quis urna justo. Etiam finibus sit amet risus at convallis. Mauris tortor libero, euismod sit amet est a, luctus vulputate sapien. Nulla dictum molestie enim vel rhoncus. Suspendisse sagittis tincidunt justo. Cras pellentesque nisl elit, non luctus velit cursus nec. Aliquam elit purus, interdum vitae suscipit sed, semper vel mauris. Nullam dolor ipsum, suscipit eget ex in, viverra pulvinar nulla. Cras sit amet nibh suscipit, egestas eros ac, tincidunt nisi.

Pellentesque sollicitudin massa id odio vulputate dapibus. Fusce pharetra maximus dictum. Maecenas dapibus pharetra metus. In rhoncus turpis venenatis lobortis tristique. Nulla eget interdum sem. Donec sed egestas sapien. Fusce ultrices sodales ex condimentum imperdiet. Suspendisse porta tellus in enim posuere vulputate. Cras rutrum massa ut dolor efficitur, sed euismod augue aliquam.

Fusce lacinia gravida augue et rhoncus. Aenean eleifend nulla eget nisl venenatis, id commodo mi auctor. Suspendisse at dolor est. Nullam consequat venenatis mollis. Vestibulum ultricies et nisi sit amet tempus. Sed porta turpis ut dolor lobortis, ultricies cursus ipsum iaculis. Phasellus non vulputate augue.

Ut varius id lacus ac dictum. Aenean rhoncus fermentum sollicitudin. Duis in quam eget diam egestas condimentum. Sed vel mattis dui. Nullam vel erat ipsum. Sed vel molestie dui, at tristique nisi. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla non orci in magna pellentesque iaculis. Mauris at est nisl. In quam metus, luctus non tempus eget, sagittis non sem. Donec tempor egestas sapien vel dignissim. Ut posuere velit eget risus tincidunt laoreet. Aliquam lorem lectus, ultricies mollis ultricies at, pellentesque et ex. Vestibulum pretium nisl in neque suscipit fringilla in vitae erat. Donec fermentum, mauris non ultricies venenatis, felis diam efficitur arcu, quis vestibulum sapien enim eu odio.

Donec semper malesuada mattis. Donec vitae egestas lacus. Vivamus aliquet, odio et cursus molestie, mi nisi vulputate purus, at auctor nibh ante ac risus. Cras luctus vehicula quam ac rhoncus. Etiam placerat, quam venenatis consequat finibus, leo quam tempus magna, quis congue nulla nisi vitae mauris. Fusce mattis egestas lobortis. Integer a enim nunc. Nunc sagittis ligula urna, et gravida libero scelerisque at.', 'This is a short description of the tag array article 2.  Again just a short description.', '{admin,cool,"\"awesome ness\"",code,lipsum,tags}', '{code,admin,lipsum,"''awesome ness''"}', '''2'':4A,15B ''ac'':119C,223C,351C,357C ''adipisc'':257C ''aenean'':180C,225C ''aliquam'':92C,173C,293C ''aliquet'':338C ''amet'':58C,67C,114C,203C,255C ''ant'':350C ''arcu'':322C ''array'':2A,13B ''articl'':3A,14B ''auctor'':189C,348C ''augu'':172C,177C,218C ''commodo'':187C ''condimentum'':155C,235C ''congu'':49C,370C ''consectetur'':29C,256C ''consequat'':37C,41C,195C,363C ''conval'':61C ''cras'':83C,112C,164C,353C ''cursus'':90C,212C,341C ''dapibus'':128C,134C ''descript'':9B,20B ''diam'':48C,233C,320C ''dictum'':74C,132C,224C ''dignissim'':285C ''dolor'':103C,168C,192C,209C,253C ''donec'':51C,147C,280C,313C,329C,333C ''dui'':229C,239C,247C ''efficitur'':169C,321C ''egesta'':117C,149C,234C,282C,335C,377C ''eget'':106C,144C,183C,232C,276C,289C ''eleifend'':181C ''elit'':86C,93C,258C ''enim'':76C,161C,326C,381C ''erat'':242C,312C ''ero'':118C ''est'':68C,193C,268C ''et'':178C,200C,301C,340C,387C ''etiam'':55C,359C ''eu'':327C ''euismod'':65C,171C ''ex'':107C,154C,302C ''feli'':319C ''fermentum'':227C,314C ''finibus'':56C,364C ''fringilla'':309C ''fusc'':129C,151C,174C,375C ''gravida'':176C,388C ''hendrerit'':26C ''iaculi'':47C,214C,265C ''id'':125C,186C,221C ''imperdiet'':156C ''integ'':379C ''interdum'':95C,145C ''ipsum'':104C,213C,243C,252C ''justo'':54C,82C ''lacinia'':175C ''lacus'':222C,336C ''laoreet'':292C ''lectus'':295C ''leo'':365C ''libero'':34C,64C,389C ''ligula'':23C,385C ''loborti'':141C,210C,378C ''lorem'':251C,294C ''luctus'':70C,88C,273C,354C ''maecena'':133C ''magna'':263C,368C ''malesuada'':331C ''massa'':25C,124C,166C ''matti'':33C,238C,332C,376C ''mauri'':62C,101C,266C,315C,374C ''maximus'':131C ''metus'':136C,272C ''mi'':188C,343C ''molesti'':75C,246C,342C ''molli'':197C,297C ''nec'':91C ''nequ'':36C,307C ''nibh'':115C,349C ''nisi'':121C,201C,250C,344C,372C ''nisl'':85C,184C,269C,305C ''non'':87C,216C,260C,274C,278C,316C ''nulla'':28C,73C,111C,143C,182C,259C,371C ''nullam'':102C,194C,240C ''nunc'':382C,383C ''odio'':45C,126C,328C,339C ''orci'':44C,261C ''pellentesqu'':84C,122C,264C,300C ''pharetra'':130C,135C ''phasellus'':215C ''placerat'':360C ''porta'':158C,206C ''posuer'':162C,287C ''pretium'':304C ''pulvinar'':110C ''purus'':94C,346C ''quam'':231C,271C,356C,361C,366C ''qui'':22C,24C,52C,323C,369C ''quisqu'':32C ''rhoncus'':78C,138C,179C,226C,358C ''risus'':40C,59C,290C,352C ''rutrum'':165C ''sagitti'':80C,277C,384C ''sapien'':72C,150C,283C,325C ''scelerisqu'':390C ''sed'':21C,38C,42C,98C,148C,170C,205C,236C,244C ''sem'':146C,279C ''semper'':99C,330C ''short'':8B,19B ''sit'':57C,66C,113C,202C,254C ''sodal'':153C ''sollicitudin'':123C,228C ''suscipit'':97C,105C,116C,308C ''suspendiss'':79C,157C,190C ''tag'':1A,12B ''tellus'':31C,159C ''tempor'':281C ''tempus'':204C,275C,367C ''tincidunt'':30C,39C,43C,81C,120C,291C ''tortor'':63C ''tristiqu'':27C,142C,249C ''turpi'':139C,207C ''ultric'':152C ''ultrici'':199C,211C,296C,298C,317C ''urna'':53C,386C ''ut'':50C,167C,208C,219C,286C ''varius'':220C ''vehicula'':355C ''vel'':77C,100C,237C,241C,245C,284C ''velit'':89C,288C ''venenati'':140C,185C,196C,318C,362C ''vestibulum'':198C,303C,324C ''vita'':46C,96C,311C,334C,373C ''vivamus'':337C ''viverra'':109C ''vulput'':71C,127C,163C,217C,345C', 1, NULL);
INSERT INTO articles (aid, title, posted, body, description, tag2, tag, fulltxt, author, markdown) VALUES (3, 'I+submitted+this', '2017-10-20 13:55:06.81759', 'This+is+some+text+I+came+up+with+for+this+submitted+article.', NULL, '{awesome,cool,admin}', '{article,lipsum,admin,cool,"''awesome ness''"}', '''articl'':14C ''came'':8C ''submit'':2A,13C ''text'':6C', 1, NULL);
INSERT INTO articles (aid, title, posted, body, description, tag2, tag, fulltxt, author, markdown) VALUES (9, 'Descriptive Article', '2017-10-23 16:37:48.479308', 'Maecenas vitae libero sit amet nisl blandit molestie eget et eros. Quisque turpis erat, convallis ac vulputate ut, placerat quis mi. Quisque ut laoreet magna. Nullam nec dolor ultrices, cursus diam et, dictum dui. Maecenas porta ipsum mi, quis placerat mi molestie nec. Integer id pretium orci. Pellentesque sit amet enim pulvinar, eleifend ligula sed, pulvinar velit. Nam vitae magna gravida, egestas arcu eu, tempor lorem. Nulla lacinia lobortis libero, convallis laoreet tellus.

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean tincidunt vestibulum tortor vitae consectetur. Etiam ac sem ultricies, placerat nibh eget, iaculis enim. Praesent congue porta neque eu elementum. Nunc varius metus ex, id finibus nibh bibendum et. Pellentesque a blandit felis. Morbi at nibh ut tortor semper tristique vitae ut orci. Aenean id laoreet lorem, a auctor nisi. Donec sed augue sed eros pretium dictum.

Aliquam cursus vitae diam non pellentesque. Fusce dolor mi, placerat sed nibh ut, maximus aliquam tellus. Maecenas mollis cursus fringilla. Vestibulum in orci ac nibh elementum egestas non viverra sem. Etiam justo nisi, condimentum eget tortor ac, eleifend semper leo. Fusce eu metus elit. Fusce commodo, risus nec porttitor condimentum, sapien quam porttitor lacus, maximus sagittis nulla dui ut nibh. Nam nisi felis, aliquam id lobortis eu, aliquet sed elit. Curabitur nisl dui, suscipit vel odio sed, vehicula placerat ex. Maecenas at vehicula odio. Donec laoreet vitae eros nec suscipit. Donec neque lacus, placerat et convallis sed, malesuada in diam. Nam in sodales neque, quis vehicula orci. Donec imperdiet neque arcu, vel fermentum magna consequat quis. Suspendisse potenti. Ut at orci in augue pretium blandit eu at nisi.

Aenean sagittis dolor ac felis porttitor, sit amet bibendum nisi aliquam. Etiam lobortis nunc et scelerisque euismod. Integer tristique quam in nulla pretium, eu viverra ex ornare. Duis a odio id lectus hendrerit sodales in non augue. Quisque iaculis posuere nibh, id feugiat ante. Fusce blandit est ac elit pharetra, nec rhoncus tellus fringilla. Pellentesque porttitor ultrices libero vel sollicitudin. Morbi quam ante, commodo a neque sed, sagittis vestibulum nisl. Donec pretium egestas turpis, id dignissim dui molestie ac. Nullam id tellus non mi dignissim fringilla quis in tortor. Nunc mollis arcu a elit eleifend, eu accumsan lorem dictum. Curabitur justo nisl, venenatis a lacus eu, dictum aliquet quam.

Phasellus ante nibh, efficitur interdum lorem eu, cursus porttitor purus. Fusce lacinia sed purus in semper. Pellentesque molestie facilisis commodo. Nulla maximus sollicitudin imperdiet. Sed vitae nisi vitae nunc egestas hendrerit ut nec lectus. Donec auctor facilisis tincidunt. Phasellus mattis turpis ac nunc mattis lobortis. Vivamus tortor purus, facilisis eget lectus eu, vehicula sodales neque.', ' Nunc ut molestie elit. Suspendisse tempus est quis leo elementum, eu ornare justo eleifend. Integer et massa vel erat maximus auctor. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Fusce eu pretium nunc. Sed euismod faucibus metus, at amet.', '{"\"awesome ness\"",cool,article,admin,programming,lipsum,code}', '{programming,code,cool}', '''ac'':57C,130C,204C,217C,312C,356C,387C,459C ''accumsan'':405C ''adipisc'':30B,121C ''aenean'':123C,167C,309C ''aliquam'':181C,195C,244C,319C ''aliquet'':248C,416C ''amet'':28B,41B,46C,91C,119C,316C ''ant'':352C,371C,419C ''arcu'':104C,291C,400C ''articl'':2A ''auctor'':23B,172C,453C ''augu'':176C,303C,345C ''bibendum'':151C,317C ''blandit'':48C,155C,305C,354C ''commodo'':226C,372C,437C ''condimentum'':214C,230C ''congu'':139C ''consectetur'':29B,120C,128C ''consequat'':295C ''conval'':56C,112C,276C ''curabitur'':251C,408C ''cursus'':71C,182C,199C,425C ''descript'':1A ''diam'':72C,184C,280C ''dictum'':74C,180C,407C,415C ''dignissim'':384C,393C ''dolor'':26B,69C,117C,188C,311C ''donec'':174C,265C,271C,288C,379C,452C ''dui'':75C,238C,253C,336C,385C ''efficitur'':421C ''egesta'':103C,207C,381C,447C ''eget'':50C,135C,215C,467C ''eleifend'':16B,94C,218C,403C ''elementum'':12B,143C,206C ''elit'':6B,31B,122C,224C,250C,357C,402C ''enim'':92C,137C ''erat'':21B,55C ''ero'':52C,178C,268C ''est'':9B,355C ''et'':18B,51C,73C,152C,275C,323C ''etiam'':129C,211C,320C ''eu'':13B,33B,105C,142C,222C,247C,306C,332C,404C,414C,424C,469C ''euismod'':37B,325C ''ex'':147C,260C,334C ''facilisi'':436C,454C,466C ''faucibus'':38B ''feli'':156C,243C,313C ''fermentum'':293C ''feugiat'':351C ''finibus'':149C ''fringilla'':200C,362C,394C ''fusc'':32B,187C,221C,225C,353C,428C ''gravida'':102C ''hendrerit'':341C,448C ''iaculi'':136C,347C ''id'':86C,148C,168C,245C,339C,350C,383C,389C ''imperdiet'':289C,441C ''integ'':17B,85C,326C ''interdum'':422C ''ipsum'':25B,78C,116C ''justo'':15B,212C,409C ''lacinia'':109C,429C ''lacus'':234C,273C,413C ''laoreet'':65C,113C,169C,266C ''lectus'':340C,451C,468C ''leo'':11B,220C ''libero'':44C,111C,366C ''ligula'':95C ''loborti'':110C,246C,321C,462C ''lorem'':24B,107C,115C,170C,406C,423C ''maecena'':42C,76C,197C,261C ''magna'':66C,101C,294C ''malesuada'':278C ''massa'':19B ''matti'':457C,461C ''maximus'':22B,194C,235C,439C ''metus'':39B,146C,223C ''mi'':62C,79C,82C,189C,392C ''molesti'':5B,49C,83C,386C,435C ''molli'':198C,399C ''morbi'':157C,369C ''nam'':99C,241C,281C ''nec'':68C,84C,228C,269C,359C,450C ''nequ'':141C,272C,284C,290C,374C,472C ''nibh'':134C,150C,159C,192C,205C,240C,349C,420C ''nisi'':173C,213C,242C,308C,318C,444C ''nisl'':47C,252C,378C,410C ''non'':185C,208C,344C,391C ''nulla'':108C,237C,330C,438C ''nullam'':67C,388C ''nunc'':3B,35B,144C,322C,398C,446C,460C ''odio'':256C,264C,338C ''orci'':88C,166C,203C,287C,301C ''ornar'':14B,335C ''pellentesqu'':89C,153C,186C,363C,434C ''pharetra'':358C ''phasellus'':418C,456C ''placerat'':60C,81C,133C,190C,259C,274C ''porta'':77C,140C ''porttitor'':229C,233C,314C,364C,426C ''posuer'':348C ''potenti'':298C ''praesent'':138C ''pretium'':34B,87C,179C,304C,331C,380C ''pulvinar'':93C,97C ''purus'':427C,431C,465C ''quam'':232C,328C,370C,417C ''qui'':10B,61C,80C,285C,296C,395C ''quisqu'':53C,63C,346C ''rhoncus'':360C ''risus'':227C ''sagitti'':236C,310C,376C ''sapien'':231C ''scelerisqu'':324C ''sed'':36B,96C,175C,177C,191C,249C,257C,277C,375C,430C,442C ''sem'':131C,210C ''semper'':162C,219C,433C ''sit'':27B,45C,90C,118C,315C ''sodal'':283C,342C,471C ''sollicitudin'':368C,440C ''suscipit'':254C,270C ''suspendiss'':7B,297C ''tellus'':114C,196C,361C,390C ''tempor'':106C ''tempus'':8B ''tincidunt'':124C,455C ''tortor'':126C,161C,216C,397C,464C ''tristiqu'':163C,327C ''turpi'':54C,382C,458C ''ultric'':70C,365C ''ultrici'':132C ''ut'':4B,59C,64C,160C,165C,193C,239C,299C,449C ''varius'':145C ''vehicula'':258C,263C,286C,470C ''vel'':20B,255C,292C,367C ''velit'':98C ''venenati'':411C ''vestibulum'':125C,201C,377C ''vita'':43C,100C,127C,164C,183C,267C,443C,445C ''vivamus'':463C ''viverra'':209C,333C ''vulput'':58C', 1, NULL);
INSERT INTO articles (aid, title, posted, body, description, tag2, tag, fulltxt, author, markdown) VALUES (5, 'I submitted this', '2017-10-20 17:15:02.170895', 'This is some text I came up with for this submitted article.', NULL, '{"\"awesome ness\"",cool,article,admin}', '{"''awesome ness''",code,admin,lipsum,cool}', '''articl'':14C ''came'':8C ''submit'':2A,13C ''text'':6C', 1, NULL);
INSERT INTO articles (aid, title, posted, body, description, tag2, tag, fulltxt, author, markdown) VALUES (8, 'Another article!', '2017-10-23 11:40:52.832981', 'Maecenas consectetur dui molestie enim tempor sodales in sed justo. Suspendisse commodo id turpis non iaculis. Nulla tempor auctor suscipit. Donec sed mi pharetra, fringilla erat et, aliquam nisi. Ut id metus in ex rhoncus porta. In et feugiat tellus. Fusce diam risus, finibus vitae dolor at, ornare lobortis nisi. Cras mattis diam in nunc posuere, euismod dapibus mi pharetra. Quisque magna est, porttitor non nulla eget, sodales tempus sem.

Nulla luctus dignissim libero, viverra interdum quam venenatis sed. Vivamus id cursus urna. Donec rutrum pulvinar nisl vel consequat. Sed eget lacus id elit convallis venenatis. Maecenas sit amet mollis enim. Nam iaculis ex sit amet metus fermentum, vitae dignissim neque suscipit. Quisque eu elit lorem. Maecenas ut tincidunt sem. Praesent quis velit in nulla hendrerit mattis ut et velit.

Cras varius urna interdum, aliquet lacus ut, egestas ex. Etiam sit amet nulla sapien. Aliquam tortor lectus, hendrerit non facilisis vitae, finibus et sem. Maecenas nec mi interdum, molestie turpis eu, porttitor tellus. Donec vel ex tortor. Integer ligula dui, sagittis hendrerit posuere sit amet, rhoncus eu velit. Curabitur ultrices est sed purus rutrum, at volutpat augue laoreet.

Nunc varius leo vitae tellus consequat scelerisque. Nunc nulla lacus, aliquet vitae vestibulum sed, eleifend nec est. Donec sit amet tellus euismod, tempor sem non, iaculis velit. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Duis quam diam, interdum id hendrerit eu, fermentum vitae nisi. Nam tincidunt urna sit amet justo pharetra porttitor. Etiam accumsan fringilla dolor, ac condimentum massa ullamcorper ut. Fusce pretium ipsum in ornare mollis. Praesent consectetur ligula eget urna fermentum efficitur. Vestibulum ut sem at neque ultrices posuere.

Cras eleifend metus ac auctor molestie. Mauris placerat ante ex, non vestibulum justo tristique id. Mauris ac vestibulum felis. Sed placerat lorem eget risus elementum pretium. Nam tristique felis a purus sagittis ornare. Nam non congue velit, non facilisis est. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Integer congue vel lorem in ornare. Ut eu ultricies risus, ut vestibulum diam. Cras efficitur tempor odio vitae ultrices. Maecenas at vehicula metus. Nam a vestibulum risus.', NULL, '{"\"awesome ness\"",cool,article,admin,lorem}', '{lipsum,awesome,cool,code,programming}', '''ac'':253C,281C,294C ''accumsan'':250C ''ad'':322C ''aliquam'':30C,147C ''aliquet'':137C,201C ''amet'':101C,108C,144C,177C,210C,245C ''anoth'':1A ''ant'':219C,286C ''aptent'':319C ''articl'':2A ''auctor'':21C,282C ''augu'':189C ''class'':318C ''commodo'':14C ''condimentum'':254C ''congu'':313C,332C ''consectetur'':4C,265C ''consequat'':91C,196C ''conubia'':326C ''conval'':97C ''cras'':53C,133C,278C,344C ''cubilia'':229C ''cura'':230C ''curabitur'':181C ''cursus'':84C ''dapibus'':60C ''diam'':44C,55C,233C,343C ''dignissim'':75C,112C ''dolor'':48C,252C ''donec'':23C,86C,166C,208C ''dui'':5C,172C,231C ''efficitur'':270C,345C ''egesta'':140C ''eget'':69C,93C,267C,300C ''eleifend'':205C,279C ''elementum'':302C ''elit'':96C,117C ''enim'':7C,103C ''erat'':28C ''est'':65C,183C,207C,317C ''et'':29C,40C,131C,155C,226C ''etiam'':142C,249C ''eu'':116C,163C,179C,237C,338C ''euismod'':59C,212C ''ex'':36C,106C,141C,168C,287C ''facilisi'':152C,316C ''faucibus'':223C ''feli'':296C,306C ''fermentum'':110C,238C,269C ''feugiat'':41C ''finibus'':46C,154C ''fringilla'':27C,251C ''fusc'':43C,258C ''hendrerit'':128C,150C,174C,236C ''himenaeo'':330C ''iaculi'':18C,105C,216C ''id'':15C,33C,83C,95C,235C,292C ''incepto'':329C ''integ'':170C,331C ''interdum'':78C,136C,160C,234C ''ipsum'':220C,260C ''justo'':12C,246C,290C ''lacus'':94C,138C,200C ''laoreet'':190C ''lectus'':149C ''leo'':193C ''libero'':76C ''ligula'':171C,266C ''litora'':323C ''loborti'':51C ''lorem'':118C,299C,334C ''luctus'':74C,225C ''maecena'':3C,99C,119C,157C,350C ''magna'':64C ''massa'':255C ''matti'':54C,129C ''mauri'':284C,293C ''metus'':34C,109C,280C,353C ''mi'':25C,61C,159C ''molesti'':6C,161C,283C ''molli'':102C,263C ''nam'':104C,241C,304C,311C,354C ''nec'':158C,206C ''nequ'':113C,275C ''nisi'':31C,52C,240C ''nisl'':89C ''non'':17C,67C,151C,215C,288C,312C,315C ''nostra'':327C ''nulla'':19C,68C,73C,127C,145C,199C ''nunc'':57C,191C,198C ''odio'':347C ''orci'':224C ''ornar'':50C,262C,310C,336C ''per'':325C,328C ''pharetra'':26C,62C,247C ''placerat'':285C,298C ''porta'':38C ''porttitor'':66C,164C,248C ''posuer'':58C,175C,228C,277C ''praesent'':123C,264C ''pretium'':259C,303C ''primi'':221C ''pulvinar'':88C ''purus'':185C,308C ''quam'':79C,232C ''qui'':124C ''quisqu'':63C,115C ''rhoncus'':37C,178C ''risus'':45C,301C,340C,357C ''rutrum'':87C,186C ''sagitti'':173C,309C ''sapien'':146C ''scelerisqu'':197C ''sed'':11C,24C,81C,92C,184C,204C,297C ''sem'':72C,122C,156C,214C,273C ''sit'':100C,107C,143C,176C,209C,244C ''sociosqu'':321C ''sodal'':9C,70C ''suscipit'':22C,114C ''suspendiss'':13C ''taciti'':320C ''tellus'':42C,165C,195C,211C ''tempor'':8C,20C,213C,346C ''tempus'':71C ''tincidunt'':121C,242C ''torquent'':324C ''tortor'':148C,169C ''tristiqu'':291C,305C ''turpi'':16C,162C ''ullamcorp'':256C ''ultric'':182C,227C,276C,349C ''ultrici'':339C ''urna'':85C,135C,243C,268C ''ut'':32C,120C,130C,139C,257C,272C,337C,341C ''varius'':134C,192C ''vehicula'':352C ''vel'':90C,167C,333C ''velit'':125C,132C,180C,217C,314C ''venenati'':80C,98C ''vestibulum'':203C,218C,271C,289C,295C,342C,356C ''vita'':47C,111C,153C,194C,202C,239C,348C ''vivamus'':82C ''viverra'':77C ''volutpat'':188C', 1, NULL);
INSERT INTO articles (aid, title, posted, body, description, tag2, tag, fulltxt, author, markdown) VALUES (7, 'New article!', '2017-10-23 11:34:44.099342', 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc pretium scelerisque nisl nec consectetur. Ut venenatis iaculis sem sit amet porttitor. In tincidunt molestie faucibus. Vivamus tristique suscipit varius. Phasellus gravida justo eu risus varius mollis. Mauris finibus quam in gravida pulvinar. In luctus fringilla nisl vel congue. Nulla tincidunt odio ac sapien scelerisque, at fermentum erat placerat. Praesent sed lorem eget diam faucibus ullamcorper. Aenean nec urna faucibus, porttitor eros eget, varius leo. Suspendisse vitae interdum nulla. Aenean egestas enim vel justo ultricies commodo.

Fusce tortor est, scelerisque at tempus at, malesuada id mauris. Vestibulum scelerisque tellus ac eros tristique, eu eleifend arcu imperdiet. Cras quis sem commodo lacus feugiat egestas a quis lectus. Fusce finibus dolor risus. Pellentesque aliquet, erat in fringilla molestie, leo elit eleifend felis, at mollis leo justo ac nisi. Mauris posuere scelerisque tortor, id convallis arcu accumsan a. Nam semper ligula id elit auctor aliquam eget eget felis. Maecenas pretium mauris nec ligula rutrum pellentesque. Fusce eget nulla eu nibh scelerisque interdum in eu erat. Suspendisse eget tellus vel orci tincidunt pellentesque. Nam non lorem euismod, semper nunc eu, tincidunt nibh. Vivamus sagittis est vitae enim posuere porta. Fusce mollis, arcu ac porta semper, elit lectus ornare mauris, sit amet mollis nunc enim non lorem. Etiam congue condimentum neque id congue. Proin orci ex, rhoncus quis aliquam eget, dapibus non lacus.

Integer accumsan turpis at eros ullamcorper, nec dapibus nisl fringilla. Suspendisse dignissim nibh eget lectus convallis, id semper purus suscipit. Aenean volutpat arcu eu enim egestas, ut ornare nibh euismod. Sed lobortis nulla eu porttitor euismod. Vestibulum tristique accumsan risus ut varius. Sed sapien sem, rhoncus ut vestibulum in, efficitur nec ante. Nam sapien augue, porttitor vitae mi eleifend, euismod dictum erat. Nulla vehicula aliquam elementum. Vestibulum elementum aliquam aliquam.

Maecenas vestibulum turpis sit amet lectus finibus, id semper metus venenatis. Nullam congue commodo magna. Aenean sit amet leo vitae elit eleifend condimentum. Etiam quis ex sem. In et erat eros. Pellentesque cursus arcu at ex placerat, sit amet aliquam elit lobortis. Morbi vitae mi dui. Praesent laoreet ex nec lobortis imperdiet. Maecenas viverra dapibus erat, non finibus felis molestie sit amet. Fusce euismod orci et purus ornare finibus. Sed rutrum imperdiet metus, at lacinia ligula rutrum sed. Sed tincidunt sodales nisi, at pellentesque velit. Suspendisse vel fermentum sapien. Nulla eu lobortis orci. Cras accumsan et libero eu ultricies. Pellentesque cursus sagittis augue, et accumsan quam sagittis vel.

In commodo tellus turpis, ac semper dolor fringilla vel. Donec tempus, velit a finibus luctus, justo ante pretium nunc, et efficitur urna tortor ut est. Sed non gravida dui, sed auctor ex. Fusce cursus a urna id pretium. Maecenas molestie eu turpis vel mollis. Sed convallis massa fringilla nunc maximus sagittis. Etiam vel nibh tempor nunc placerat elementum a sed metus. Integer mollis scelerisque est, non tempus libero ullamcorper vel. Maecenas faucibus elit ante, id bibendum orci fermentum vitae. Nam in metus erat. Quisque iaculis lobortis augue eu convallis. Donec et neque vitae turpis convallis molestie vitae vel tortor. Suspendisse et tincidunt libero. Aenean rhoncus neque ut risus porttitor, quis posuere sapien egestas. Aliquam erat volutpat.', NULL, '{"\"awesome ness\"",cool,article,admin}', '{awesome,lipsum,admin}', '''ac'':54C,101C,136C,200C,412C ''accumsan'':145C,231C,268C,394C,404C ''adipisc'':9C ''aenean'':68C,81C,250C,315C,511C ''aliquam'':153C,225C,294C,298C,299C,339C,521C ''aliquet'':123C ''amet'':7C,22C,208C,304C,317C,338C,361C ''ant'':281C,424C,481C ''arcu'':106C,144C,199C,252C,333C ''articl'':2A ''auctor'':152C,438C ''augu'':284C,402C,494C ''bibendum'':483C ''commodo'':87C,111C,313C,409C ''condimentum'':216C,322C ''congu'':50C,215C,219C,312C ''consectetur'':8C,16C ''conval'':143C,245C,453C,496C,502C ''cras'':108C,393C ''cursus'':332C,400C,441C ''dapibus'':227C,237C,354C ''diam'':65C ''dictum'':290C ''dignissim'':241C ''dolor'':5C,120C,414C ''donec'':417C,497C ''dui'':345C,436C ''efficitur'':279C,428C ''egesta'':82C,114C,255C,520C ''eget'':64C,74C,154C,155C,165C,175C,226C,243C ''eleifend'':105C,130C,288C,321C ''elementum'':295C,297C,465C ''elit'':10C,129C,151C,203C,320C,340C,480C ''enim'':83C,194C,211C,254C ''erat'':59C,124C,173C,291C,329C,355C,490C,522C ''ero'':73C,102C,234C,330C ''est'':90C,192C,432C,472C ''et'':328C,365C,395C,403C,427C,498C,508C ''etiam'':214C,323C,459C ''eu'':35C,104C,167C,172C,187C,253C,263C,390C,397C,448C,495C ''euismod'':184C,259C,265C,289C,363C ''ex'':222C,325C,335C,348C,439C ''faucibus'':27C,66C,71C,479C ''feli'':131C,156C,358C ''fermentum'':58C,387C,485C ''feugiat'':113C ''finibus'':40C,119C,306C,357C,368C,421C ''fringilla'':47C,126C,239C,415C,455C ''fusc'':88C,118C,164C,197C,362C,440C ''gravida'':33C,43C,435C ''iaculi'':19C,492C ''id'':96C,142C,150C,218C,246C,307C,444C,482C ''imperdiet'':107C,351C,371C ''integ'':230C,469C ''interdum'':79C,170C ''ipsum'':4C ''justo'':34C,85C,135C,423C ''lacinia'':374C ''lacus'':112C,229C ''laoreet'':347C ''lectus'':117C,204C,244C,305C ''leo'':76C,128C,134C,318C ''libero'':396C,475C,510C ''ligula'':149C,161C,375C ''loborti'':261C,341C,350C,391C,493C ''lorem'':3C,63C,183C,213C ''luctus'':46C,422C ''maecena'':157C,300C,352C,446C,478C ''magna'':314C ''malesuada'':95C ''massa'':454C ''mauri'':39C,97C,138C,159C,206C ''maximus'':457C ''metus'':309C,372C,468C,489C ''mi'':287C,344C ''molesti'':26C,127C,359C,447C,503C ''molli'':38C,133C,198C,209C,451C,470C ''morbi'':342C ''nam'':147C,181C,282C,487C ''nec'':15C,69C,160C,236C,280C,349C ''nequ'':217C,499C,513C ''new'':1A ''nibh'':168C,189C,242C,258C,461C ''nisi'':137C,381C ''nisl'':14C,48C,238C ''non'':182C,212C,228C,356C,434C,473C ''nulla'':51C,80C,166C,262C,292C,389C ''nullam'':311C ''nunc'':11C,186C,210C,426C,456C,463C ''odio'':53C ''orci'':178C,221C,364C,392C,484C ''ornar'':205C,257C,367C ''pellentesqu'':122C,163C,180C,331C,383C,399C ''phasellus'':32C ''placerat'':60C,336C,464C ''porta'':196C,201C ''porttitor'':23C,72C,264C,285C,516C ''posuer'':139C,195C,518C ''praesent'':61C,346C ''pretium'':12C,158C,425C,445C ''proin'':220C ''pulvinar'':44C ''purus'':248C,366C ''quam'':41C,405C ''qui'':109C,116C,224C,324C,517C ''quisqu'':491C ''rhoncus'':223C,275C,512C ''risus'':36C,121C,269C,515C ''rutrum'':162C,370C,376C ''sagitti'':191C,401C,406C,458C ''sapien'':55C,273C,283C,388C,519C ''scelerisqu'':13C,56C,91C,99C,140C,169C,471C ''sed'':62C,260C,272C,369C,377C,378C,433C,437C,452C,467C ''sem'':20C,110C,274C,326C ''semper'':148C,185C,202C,247C,308C,413C ''sit'':6C,21C,207C,303C,316C,337C,360C ''sodal'':380C ''suscipit'':30C,249C ''suspendiss'':77C,174C,240C,385C,507C ''tellus'':100C,176C,410C ''tempor'':462C ''tempus'':93C,418C,474C ''tincidunt'':25C,52C,179C,188C,379C,509C ''tortor'':89C,141C,430C,506C ''tristiqu'':29C,103C,267C ''turpi'':232C,302C,411C,449C,501C ''ullamcorp'':67C,235C,476C ''ultrici'':86C,398C ''urna'':70C,429C,443C ''ut'':17C,256C,270C,276C,431C,514C ''varius'':31C,37C,75C,271C ''vehicula'':293C ''vel'':49C,84C,177C,386C,407C,416C,450C,460C,477C,505C ''velit'':384C,419C ''venenati'':18C,310C ''vestibulum'':98C,266C,277C,296C,301C ''vita'':78C,193C,286C,319C,343C,486C,500C,504C ''vivamus'':28C,190C ''viverra'':353C ''volutpat'':251C,523C', 1, NULL);
INSERT INTO articles (aid, title, posted, body, description, tag2, tag, fulltxt, author, markdown) VALUES (10, 'Tag Array Article', '2017-10-23 18:04:20.779566', 'Nullam sit amet interdum mauris, at fringilla urna. Fusce porta elit orci, at molestie augue pulvinar at. Vestibulum egestas lacus eget justo commodo, tempor rutrum odio molestie. Sed elit metus, blandit nec malesuada ac, elementum a turpis. Mauris sodales dolor vitae neque mollis, lacinia venenatis libero egestas. Sed nibh massa, commodo non pretium vitae, euismod eget ante. Suspendisse vestibulum et elit in vehicula.

In ultricies leo eget nibh pretium auctor. Nullam at lorem in nibh condimentum luctus nec vel leo. Nam vitae quam semper, ultrices magna ac, aliquam dolor. Mauris facilisis et nisi ut vehicula. Aenean dui neque, rhoncus a sem vel, laoreet sollicitudin turpis. Aliquam condimentum nec odio quis vulputate. Integer ut suscipit enim. Duis porta ante vel condimentum congue. Fusce rutrum augue a libero ultricies, quis ultrices risus consectetur. Nulla interdum lacinia est ac sodales. Mauris quis ligula quis ligula ultrices tristique a sit amet metus. Proin purus odio, dignissim non nisl quis, pretium aliquam nulla. In hac habitasse platea dictumst. Nam metus ligula, vehicula eu vestibulum vel, ultrices ut elit. Nam tellus ipsum, congue sed placerat quis, vestibulum quis eros.

Nam non vehicula lectus, tristique sagittis enim. Nam malesuada odio libero, sit amet suscipit magna dictum eget. Nunc convallis rhoncus nunc fermentum bibendum. Duis non lorem mi. Aliquam erat volutpat. Curabitur cursus, mi non euismod maximus, sem libero convallis arcu, vel aliquet nunc magna quis lorem. In rhoncus arcu enim, quis porta nibh ultrices non.

Morbi nec molestie justo. Duis vitae laoreet purus, vel dignissim lorem. Proin porttitor lacus semper ipsum egestas viverra. Praesent et accumsan libero. Cras id quam vel velit semper interdum non sed justo. Donec dictum libero ac velit vehicula placerat. Praesent vel egestas orci, quis semper odio. Donec vehicula lorem iaculis neque venenatis pulvinar. Vestibulum pretium consequat vestibulum. Donec lacus ex, elementum ac nisi at, consequat feugiat est. Aliquam nec gravida urna. Praesent ante ipsum, pellentesque eget sollicitudin convallis, facilisis sit amet neque. Phasellus blandit, enim et scelerisque aliquet, augue metus suscipit enim, commodo vehicula arcu dolor sed odio. Pellentesque mollis lectus sit amet lacinia condimentum. Nunc porta ante vel diam egestas, in maximus est vulputate.

Curabitur lobortis gravida quam, non feugiat quam. Aliquam sed massa at massa euismod elementum. Integer eros erat, dapibus vitae dictum quis, feugiat a felis. Donec aliquam massa at ultricies sollicitudin. Mauris quam justo, dapibus eleifend lectus nec, mattis fermentum justo. Etiam quis mauris sed ligula tempus sagittis eu id nunc. Integer imperdiet metus luctus gravida tincidunt. Duis vehicula mi sit amet nisl elementum lacinia. Nullam vel iaculis massa, sit amet dictum nisl.', 'Donec ultricies rhoncus massa, sed tristique est vehicula ac. Maecenas aliquam feugiat orci quis congue. Pellentesque interdum eros in ex imperdiet interdum. Nunc a fermentum felis. Nam posuere vehicula nulla, in porta est mollis nec. Etiam a nullam.', '{test,admin,cool,"\"awesome ness\"",code,tags}', '{lipsum,code,admin}', '''ac'':12B,75C,128C,177C,315C,341C ''accumsan'':300C ''aenean'':137C ''aliquam'':14B,129C,147C,198C,252C,347C,402C,420C ''aliquet'':266C,367C ''amet'':44C,188C,237C,360C,382C,455C,464C ''ant'':98C,159C,352C,387C ''arcu'':264C,273C,374C ''array'':2A ''articl'':3A ''auctor'':111C ''augu'':56C,165C,368C ''bibendum'':247C ''blandit'':72C,363C ''commodo'':64C,92C,372C ''condimentum'':117C,148C,161C,384C ''congu'':18B,162C,218C ''consectetur'':172C ''consequat'':335C,344C ''conval'':243C,263C,357C ''cras'':302C ''curabitur'':255C,395C ''cursus'':256C ''dapibus'':412C,428C ''diam'':389C ''dictum'':240C,313C,414C,465C ''dictumst'':204C ''dignissim'':193C,289C ''dolor'':81C,130C,375C ''donec'':4B,312C,326C,337C,419C ''dui'':138C,157C,248C,284C,451C ''egesta'':60C,88C,296C,321C,390C ''eget'':62C,97C,108C,241C,355C ''eleifend'':429C ''elementum'':76C,340C,408C,457C ''elit'':52C,70C,102C,214C ''enim'':156C,231C,274C,364C,371C ''erat'':253C,411C ''ero'':21B,224C,410C ''est'':10B,36B,176C,346C,393C ''et'':101C,133C,299C,365C ''etiam'':39B,435C ''eu'':209C,442C ''euismod'':96C,259C,407C ''ex'':23B,339C ''facilisi'':132C,358C ''feli'':29B,418C ''fermentum'':28B,246C,433C ''feugiat'':15B,345C,400C,416C ''fringilla'':48C ''fusc'':50C,163C ''gravida'':349C,397C,449C ''habitass'':202C ''hac'':201C ''iaculi'':329C,461C ''id'':303C,443C ''imperdiet'':24B,446C ''integ'':153C,409C,445C ''interdum'':20B,25B,45C,174C,308C ''ipsum'':217C,295C,353C ''justo'':63C,283C,311C,427C,434C ''lacinia'':85C,175C,383C,458C ''lacus'':61C,293C,338C ''laoreet'':144C,286C ''lectus'':228C,380C,430C ''leo'':107C,121C ''libero'':87C,167C,235C,262C,301C,314C ''ligula'':181C,183C,207C,439C ''loborti'':396C ''lorem'':114C,250C,270C,290C,328C ''luctus'':118C,448C ''maecena'':13B ''magna'':127C,239C,268C ''malesuada'':74C,233C ''massa'':7B,91C,404C,406C,421C,462C ''matti'':432C ''mauri'':46C,79C,131C,179C,425C,437C ''maximus'':260C,392C ''metus'':71C,189C,206C,369C,447C ''mi'':251C,257C,453C ''molesti'':55C,68C,282C ''molli'':37B,84C,379C ''morbi'':280C ''nam'':30B,122C,205C,215C,225C,232C ''nec'':38B,73C,119C,149C,281C,348C,431C ''nequ'':83C,139C,330C,361C ''nibh'':90C,109C,116C,277C ''nisi'':134C,342C ''nisl'':195C,456C,466C ''non'':93C,194C,226C,249C,258C,279C,309C,399C ''nulla'':33B,173C,199C ''nullam'':41B,42C,112C,459C ''nunc'':26B,242C,245C,267C,385C,444C ''odio'':67C,150C,192C,234C,325C,377C ''orci'':16B,53C,322C ''pellentesqu'':19B,354C,378C ''phasellus'':362C ''placerat'':220C,318C ''platea'':203C ''porta'':35B,51C,158C,276C,386C ''porttitor'':292C ''posuer'':31B ''praesent'':298C,319C,351C ''pretium'':94C,110C,197C,334C ''proin'':190C,291C ''pulvinar'':57C,332C ''purus'':191C,287C ''quam'':124C,304C,398C,401C,426C ''qui'':17B,151C,169C,180C,182C,196C,221C,223C,269C,275C,323C,415C,436C ''rhoncus'':6B,140C,244C,272C ''risus'':171C ''rutrum'':66C,164C ''sagitti'':230C,441C ''scelerisqu'':366C ''sed'':8B,69C,89C,219C,310C,376C,403C,438C ''sem'':142C,261C ''semper'':125C,294C,307C,324C ''sit'':43C,187C,236C,359C,381C,454C,463C ''sodal'':80C,178C ''sollicitudin'':145C,356C,424C ''suscipit'':155C,238C,370C ''suspendiss'':99C ''tag'':1A ''tellus'':216C ''tempor'':65C ''tempus'':440C ''tincidunt'':450C ''tristiqu'':9B,185C,229C ''turpi'':78C,146C ''ultric'':126C,170C,184C,212C,278C ''ultrici'':5B,106C,168C,423C ''urna'':49C,350C ''ut'':135C,154C,213C ''vehicula'':11B,32B,104C,136C,208C,227C,317C,327C,373C,452C ''vel'':120C,143C,160C,211C,265C,288C,305C,320C,388C,460C ''velit'':306C,316C ''venenati'':86C,331C ''vestibulum'':59C,100C,210C,222C,333C,336C ''vita'':82C,95C,123C,285C,413C ''viverra'':297C ''volutpat'':254C ''vulput'':152C,394C', 1, NULL);
INSERT INTO articles (aid, title, posted, body, description, tag2, tag, fulltxt, author, markdown) VALUES (13, 'Woo All_tags Page Added 5', '2017-10-27 18:41:49.77305', '<p><b>I said good day</b></p>. Maecenas auctor felis non turpis vehicula tempor. In hac habitasse platea dictumst. Ut sollicitudin, nibh id convallis sodales, tellus justo vulputate justo, vel posuere felis ante vitae tellus. Nulla facilisis lectus non egestas scelerisque. Etiam in tortor non elit ultricies ultricies vitae at turpis. Nulla blandit diam a tristique rutrum. Nam vehicula orci eros, at consequat felis facilisis sit amet.
<p>Integer <strong>malesuada</strong> tortor et leo eleifend tincidunt. Pellentesque in tristique nulla. Nulla diam lorem, egestas nec mi sodales, vestibulum vehicula velit. In lacinia non nisi eget semper. Maecenas ut neque id tortor sodales facilisis. Suspendisse consequat felis ac leo aliquet, et porta nisl auctor. Sed elementum iaculis est id vestibulum. Cras tristique mi nec risus dignissim, eu rhoncus nunc placerat. Cras cursus augue nec ante malesuada, sed eleifend mauris posuere. Fusce fermentum dapibus ultricies. Ut tempor tortor malesuada metus scelerisque, sit amet varius turpis ornare. Donec porta porttitor euismod.</p>
<p>Donec aliquet sem a metus convallis, porttitor convallis orci feugiat. Vestibulum varius ultrices feugiat. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Donec ipsum leo, vulputate nec tempor sit amet, maximus quis neque. Etiam suscipit lobortis porta. Sed in dui at mauris laoreet lacinia placerat ac orci. Sed id dolor sed libero sodales dapibus id ac magna. Vivamus bibendum vestibulum varius. Nulla at eros eu eros laoreet suscipit ac id massa. Proin posuere consequat diam et pellentesque. Maecenas vel nisi commodo, placerat nisi quis, accumsan ante. In urna neque, congue vehicula feugiat nec, scelerisque sit amet enim. Nam tempus orci in nulla pulvinar, nec varius metus ornare.</p>
<p>Donec feugiat neque a commodo venenatis. Curabitur ornare lectus quis vulputate egestas. Aenean maximus dictum interdum. Pellentesque semper felis dolor, id condimentum sapien mollis ac. Nullam ut libero ac sem mollis fringilla vehicula eu elit. Integer eget leo lacinia, sodales elit quis, feugiat lacus. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Donec interdum at sapien a condimentum. Nunc ac tincidunt purus. Donec urna nisi, fermentum id varius sit amet, vestibulum sit amet libero. Maecenas sit amet luctus magna. Donec varius non dui vel sollicitudin. In fringilla orci non elit fringilla, eu lobortis nulla lobortis. In hac habitasse platea dictumst. Nulla facilisi. Vivamus lobortis tempus imperdiet. 5</p>
', 'Good day. Fusce id ex ut ante dignissim pretium at in lorem. Nullam ac massa a metus rhoncus consequat non at neque. Phasellus convallis ultrices facilisis. Pellentesque nec eleifend massa. Etiam bibendum, erat eu vulputate suscipit, metus ligula pellentesque est, nec tristique felis risus id eros. Pellentesque et sem magna. Phasellus facilisis tellus quis turpis ornare, id feugiat lorem maximus.5', NULL, '{superman,admin,cool,"awesome ness",tags,programming,lipsum,tag,pages,two,three,four,five}', '''5'':6A,442C ''ac'':20B,168C,270C,280C,293C,356C,360C,395C ''accumsan'':309C ''ad'':5A,238C ''aenean'':344C ''aliquet'':170C,221C ''amet'':130C,212C,254C,320C,405C,408C,412C ''ant'':13B,96C,195C,310C ''aptent'':235C ''auctor'':72C,174C ''augu'':193C ''bibendum'':38B,283C ''blandit'':116C ''class'':234C ''commodo'':305C,336C ''condimentum'':353C,393C ''congu'':314C ''consequat'':25B,126C,166C,298C ''conubia'':242C ''conval'':30B,87C,225C,227C ''cras'':181C,191C ''curabitur'':338C ''cursus'':192C ''dapibus'':203C,278C ''day'':8B,70C ''diam'':117C,143C,299C ''dictum'':346C ''dictumst'':82C,435C ''dignissim'':14B,186C ''dis'':382C ''dolor'':274C,351C ''donec'':216C,220C,247C,332C,388C,398C,415C ''dui'':264C,418C ''egesta'':103C,145C,343C ''eget'':156C,368C ''eleifend'':35B,136C,198C ''elementum'':176C ''elit'':109C,366C,372C,425C ''enim'':321C ''erat'':39B ''ero'':52B,124C,288C,290C ''est'':46B,178C ''et'':54B,134C,171C,300C,380C ''etiam'':37B,105C,258C ''eu'':40B,187C,289C,365C,427C ''euismod'':219C ''ex'':11B ''facilisi'':32B,58B,100C,128C,164C,437C ''feli'':49B,73C,95C,127C,167C,350C ''fermentum'':202C,401C ''feugiat'':64B,229C,233C,316C,333C,374C ''fringilla'':363C,422C,426C ''fusc'':9B,201C ''good'':7B,69C ''habitass'':80C,433C ''hac'':79C,432C ''himenaeo'':246C ''iaculi'':177C ''id'':10B,51B,63B,86C,161C,179C,273C,279C,294C,352C,402C ''imperdiet'':441C ''incepto'':245C ''integ'':131C,367C ''interdum'':347C,389C ''ipsum'':248C ''justo'':90C,92C ''lacinia'':153C,268C,370C ''lacus'':375C ''laoreet'':267C,291C ''lectus'':101C,340C ''leo'':135C,169C,249C,369C ''libero'':276C,359C,409C ''ligula'':44B ''litora'':239C ''loborti'':260C,428C,430C,439C ''lorem'':18B,65B,144C ''luctus'':413C ''maecena'':71C,158C,302C,410C ''magna'':56B,281C,414C ''magni'':381C ''malesuada'':132C,196C,208C ''massa'':21B,36B,295C ''mauri'':199C,266C ''maximus'':255C,345C ''maximus.5'':66B ''metus'':23B,43B,209C,224C,330C ''mi'':147C,183C ''molli'':355C,362C ''mont'':384C ''mus'':387C ''nam'':121C,322C ''nascetur'':385C ''natoqu'':378C ''nec'':34B,47B,146C,184C,194C,251C,317C,328C ''nequ'':28B,160C,257C,313C,334C ''nibh'':85C ''nisi'':155C,304C,307C,400C ''nisl'':173C ''non'':26B,74C,102C,108C,154C,417C,424C ''nostra'':243C ''nulla'':99C,115C,141C,142C,286C,326C,429C,436C ''nullam'':19B,357C ''nunc'':189C,394C ''orci'':123C,228C,271C,324C,376C,423C ''ornar'':62B,215C,331C,339C ''page'':4A ''parturi'':383C ''pellentesqu'':33B,45B,53B,138C,301C,348C ''penatibus'':379C ''per'':241C,244C ''phasellus'':29B,57B ''placerat'':190C,269C,306C ''platea'':81C,434C ''porta'':172C,217C,261C ''porttitor'':218C,226C ''posuer'':94C,200C,297C ''pretium'':15B ''proin'':296C ''pulvinar'':327C ''purus'':397C ''qui'':60B,256C,308C,341C,373C ''rhoncus'':24B,188C ''ridiculus'':386C ''risus'':50B,185C ''rutrum'':120C ''said'':68C ''sapien'':354C,391C ''scelerisqu'':104C,210C,318C ''sed'':175C,197C,262C,272C,275C ''sem'':55B,222C,361C ''semper'':157C,349C ''sit'':129C,211C,253C,319C,404C,407C,411C ''sociosqu'':237C ''sodal'':88C,148C,163C,277C,371C ''sollicitudin'':84C,420C ''suscipit'':42B,259C,292C ''suspendiss'':165C ''taciti'':236C ''tag'':3A ''tellus'':59B,89C,98C ''tempor'':77C,206C,252C ''tempus'':323C,440C ''tincidunt'':137C,396C ''torquent'':240C ''tortor'':107C,133C,162C,207C ''tristiqu'':48B,119C,140C,182C ''turpi'':61B,75C,114C,214C ''ultric'':31B,232C ''ultrici'':110C,111C,204C ''urna'':312C,399C ''ut'':12B,83C,159C,205C,358C ''varius'':213C,231C,285C,329C,377C,403C,416C ''vehicula'':76C,122C,150C,315C,364C ''vel'':93C,303C,419C ''velit'':151C ''venenati'':337C ''vestibulum'':149C,180C,230C,284C,406C ''vita'':97C,112C ''vivamus'':282C,438C ''vulput'':41B,91C,250C,342C ''woo'':1A', 1, '<p><b>I said good day</b></p>. Maecenas auctor felis non turpis vehicula tempor. In hac habitasse platea dictumst. Ut sollicitudin, nibh id convallis sodales, tellus justo vulputate justo, vel posuere felis ante vitae tellus. Nulla facilisis lectus non egestas scelerisque. Etiam in tortor non elit ultricies ultricies vitae at turpis. Nulla blandit diam a tristique rutrum. Nam vehicula orci eros, at consequat felis facilisis sit amet.


Integer **malesuada** tortor et leo eleifend tincidunt. Pellentesque in tristique nulla. Nulla diam lorem, egestas nec mi sodales, vestibulum vehicula velit. In lacinia non nisi eget semper. Maecenas ut neque id tortor sodales facilisis. Suspendisse consequat felis ac leo aliquet, et porta nisl auctor. Sed elementum iaculis est id vestibulum. Cras tristique mi nec risus dignissim, eu rhoncus nunc placerat. Cras cursus augue nec ante malesuada, sed eleifend mauris posuere. Fusce fermentum dapibus ultricies. Ut tempor tortor malesuada metus scelerisque, sit amet varius turpis ornare. Donec porta porttitor euismod.

Donec aliquet sem a metus convallis, porttitor convallis orci feugiat. Vestibulum varius ultrices feugiat. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Donec ipsum leo, vulputate nec tempor sit amet, maximus quis neque. Etiam suscipit lobortis porta. Sed in dui at mauris laoreet lacinia placerat ac orci. Sed id dolor sed libero sodales dapibus id ac magna. Vivamus bibendum vestibulum varius. Nulla at eros eu eros laoreet suscipit ac id massa. Proin posuere consequat diam et pellentesque. Maecenas vel nisi commodo, placerat nisi quis, accumsan ante. In urna neque, congue vehicula feugiat nec, scelerisque sit amet enim. Nam tempus orci in nulla pulvinar, nec varius metus ornare.

Donec feugiat neque a commodo venenatis. Curabitur ornare lectus quis vulputate egestas. Aenean maximus dictum interdum. Pellentesque semper felis dolor, id condimentum sapien mollis ac. Nullam ut libero ac sem mollis fringilla vehicula eu elit. Integer eget leo lacinia, sodales elit quis, feugiat lacus. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Donec interdum at sapien a condimentum. Nunc ac tincidunt purus. Donec urna nisi, fermentum id varius sit amet, vestibulum sit amet libero. Maecenas sit amet luctus magna. Donec varius non dui vel sollicitudin. In fringilla orci non elit fringilla, eu lobortis nulla lobortis. In hac habitasse platea dictumst. Nulla facilisi. Vivamus lobortis tempus imperdiet. 5');
INSERT INTO articles (aid, title, posted, body, description, tag2, tag, fulltxt, author, markdown) VALUES (6, 'this is an admin article', '2017-10-20 17:33:39.989516', 'This article was submitted by an admin', NULL, '{"\"awesome ness\"",cool,article,admin}', '{code,programming,lipsum,"''awesome ness''"}', '''admin'':4A,12C ''articl'':5A,7C ''submit'':9C', 1, NULL);
INSERT INTO articles (aid, title, posted, body, description, tag2, tag, fulltxt, author, markdown) VALUES (14, 'This is my article', '2017-12-19 00:57:14.999488', '<p>And a v v  <strong>cool</strong> article</p>
', 'This is a description', NULL, '{super,cool,article}', '''articl'':4A,14C ''cool'':13C ''descript'':8B ''v'':11C,12C', NULL, 'And a v v  **cool** article');
INSERT INTO articles (aid, title, posted, body, description, tag2, tag, fulltxt, author, markdown) VALUES (15, 'Cool New Article', '2017-12-19 01:10:44.388136', '<h1>Lorem ipsum dolor sit amet</h1>
<p>consectetur adipiscing elit. Praesent id facilisis lacus. Maecenas vel dignissim ipsum. Phasellus mattis luctus orci, sit <em>amet gravida</em> nunc scelerisque non. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Morbi non erat quis turpis viverra egestas dapibus et lorem. Quisque placerat est eros. Maecenas <strong>pretium lobortis</strong> orci, sit amet blandit leo porttitor ac. Suspendisse non risus et magna fermentum placerat. Donec sem leo, accumsan in sollicitudin a, imperdiet at velit. Fusce orci erat, fermentum ac neque vehicula, cursus facilisis tellus. Proin suscipit eu elit sed viverra. Phasellus lacinia enim eget libero tempus rutrum. Mauris iaculis, lectus at posuere congue, urna est efficitur tortor, eget tempor tellus <strong><em>ante et orci</em></strong>.</p>
<p>Maecenas molestie at sapien tempus feugiat. Aliquam hendrerit, magna vel dapibus posuere, nisl mi accumsan velit, sit amet consequat massa ante ut est. Maecenas nec orci dolor. In urna eros, dictum quis eleifend a, egestas ut nibh. In hac habitasse platea dictumst. Integer tempus enim ut arcu ultrices auctor. Morbi vulputate pharetra lorem, vel imperdiet ipsum.</p>
<pre><code class="language-plain"><span class="hljs-attribute">Curabitur</span> volutpat elit et iaculis luctus. 
Curabitur et purus eros. 
Nunc nibh turpis, luctus quis nulla vitae, 
</code></pre>
<p>commodo cursus leo. Morbi eu convallis urna. Suspendisse nec erat in justo dignissim lobortis eget eget libero. Integer sed ante luctus, rhoncus lectus a, iaculis felis. Curabitur id leo efficitur, facilisis nibh a, vulputate est.</p>
<p>Donec placerat dignissim leo ac tempor. Sed lacinia eros odio, eget porttitor metus iaculis sed. Aenean id magna in eros convallis dapibus at nec est. Aliquam et purus tempor, tristique magna non, gravida est. Interdum et malesuada fames ac ante ipsum primis in faucibus. Morbi aliquam egestas risus, ut finibus dui suscipit quis. Nunc a urna sit amet tortor porta interdum sit amet id dui.</p>
<p>Suspendisse gravida odio quis dictum venenatis. Cras ac vestibulum urna, at facilisis neque. Mauris porta nisi in dapibus feugiat. Pellentesque suscipit id mi et egestas. Ut auctor vel urna quis faucibus. Nunc quis sem sollicitudin, pretium tortor et, malesuada nibh. Curabitur nec pretium nunc, iaculis dapibus diam. Praesent sodales iaculis augue, non tempus massa mollis ac. Nullam euismod, neque eu blandit sollicitudin, odio nisl tempus lacus, in euismod quam velit ut diam. Integer velit nisi, accumsan at magna at, dapibus sollicitudin nisi. Fusce venenatis sed arcu eu rutrum. Phasellus aliquam tortor sed mauris malesuada, eget congue lorem gravida. Praesent eleifend magna euismod elit tempus pellentesque. Vivamus ultrices nunc metus, scelerisque molestie ante maximus vitae. Nam consequat semper lacus nec sodales.</p>
', 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut tortor augue, facilisis eu rutrum quis, viverra id velit. Morbi et ex convallis, facilisis magna vel, auctor augue. Praesent ullamcorper ultricies finibus. Aenean volutpat tincidunt eros id mollis. Sed mattis felis rutrum erat sagittis pretium. Fusce non urna vel quam posuere dapibus. Pellentesque consequat augue semper odio maximus', NULL, '{""}', '''ac'':122C,144C,291C,325C,359C,407C ''accumsan'':133C,193C,427C ''adipisc'':10B,67C ''aenean'':35B,302C ''aliquam'':185C,312C,332C,441C ''amet'':8B,65C,82C,118C,196C,344C,349C ''ant'':176C,199C,271C,326C,463C ''arcu'':225C,437C ''articl'':3A ''auctor'':29B,227C,378C ''augu'':14B,30B,57B,402C ''blandit'':119C,412C ''commodo'':252C ''congu'':168C,447C ''consectetur'':9B,66C ''consequat'':56B,197C,467C ''conval'':25B,257C,307C ''cool'':1A ''cras'':358C ''curabitur'':235C,241C,278C,392C ''cursus'':147C,253C ''dapibus'':54B,106C,189C,308C,369C,397C,431C ''diam'':398C,423C ''dictum'':209C,356C ''dictumst'':220C ''dignissim'':75C,264C,289C ''dis'':93C ''dolor'':6B,63C,205C ''donec'':130C,287C ''dui'':337C,351C ''efficitur'':171C,281C ''egesta'':105C,213C,333C,376C ''eget'':159C,173C,266C,267C,297C,446C ''eleifend'':211C,451C ''elit'':11B,68C,153C,237C,454C ''enim'':158C,223C ''erat'':45B,101C,142C,261C ''ero'':38B,112C,208C,244C,295C,306C ''est'':111C,170C,201C,286C,311C,320C ''et'':23B,91C,107C,126C,177C,238C,242C,313C,322C,375C,389C ''eu'':16B,152C,256C,411C,438C ''euismod'':409C,419C,453C ''ex'':24B ''facilisi'':15B,26B,71C,148C,282C,363C ''fame'':324C ''faucibus'':330C,382C ''feli'':43B,277C ''fermentum'':128C,143C ''feugiat'':184C,370C ''finibus'':34B,336C ''fusc'':48B,140C,434C ''gravida'':83C,319C,353C,449C ''habitass'':218C ''hac'':217C ''hendrerit'':186C ''iaculi'':164C,239C,276C,300C,396C,401C ''id'':20B,39B,70C,279C,303C,350C,373C ''imperdiet'':137C,233C ''integ'':221C,269C,424C ''interdum'':321C,347C ''ipsum'':5B,62C,76C,234C,327C ''justo'':263C ''lacinia'':157C,294C ''lacus'':72C,417C,469C ''lectus'':165C,274C ''leo'':120C,132C,254C,280C,290C ''libero'':160C,268C ''loborti'':115C,265C ''lorem'':4B,61C,108C,231C,448C ''luctus'':79C,240C,248C,272C ''maecena'':73C,113C,179C,202C ''magna'':27B,127C,187C,304C,317C,429C,452C ''magni'':92C ''malesuada'':323C,390C,445C ''massa'':198C,405C ''matti'':42B,78C ''mauri'':163C,365C,444C ''maximus'':60B,464C ''metus'':299C,460C ''mi'':192C,374C ''molesti'':180C,462C ''molli'':40B,406C ''mont'':95C ''morbi'':22B,99C,228C,255C,331C ''mus'':98C ''nam'':466C ''nascetur'':96C ''natoqu'':89C ''nec'':203C,260C,310C,393C,470C ''nequ'':145C,364C,410C ''new'':2A ''nibh'':215C,246C,283C,391C ''nisi'':367C,426C,433C ''nisl'':191C,415C ''non'':49B,86C,100C,124C,318C,403C ''nulla'':250C ''nullam'':408C ''nunc'':84C,245C,340C,383C,395C,459C ''odio'':59B,296C,354C,414C ''orci'':80C,87C,116C,141C,178C,204C ''parturi'':94C ''pellentesqu'':55B,371C,456C ''penatibus'':90C ''pharetra'':230C ''phasellus'':77C,156C,440C ''placerat'':110C,129C,288C ''platea'':219C ''porta'':346C,366C ''porttitor'':121C,298C ''posuer'':53B,167C,190C ''praesent'':31B,69C,399C,450C ''pretium'':47B,114C,387C,394C ''primi'':328C ''proin'':150C ''purus'':243C,314C ''quam'':52B,420C ''qui'':18B,102C,210C,249C,339C,355C,381C,384C ''quisqu'':109C ''rhoncus'':273C ''ridiculus'':97C ''risus'':125C,334C ''rutrum'':17B,44B,162C,439C ''sagitti'':46B ''sapien'':182C ''scelerisqu'':85C,461C ''sed'':41B,154C,270C,293C,301C,436C,443C ''sem'':131C,385C ''semper'':58B,468C ''sit'':7B,64C,81C,117C,195C,343C,348C ''sodal'':400C,471C ''sollicitudin'':135C,386C,413C,432C ''suscipit'':151C,338C,372C ''suspendiss'':123C,259C,352C ''tellus'':149C,175C ''tempor'':174C,292C,315C ''tempus'':161C,183C,222C,404C,416C,455C ''tincidunt'':37B ''tortor'':13B,172C,345C,388C,442C ''tristiqu'':316C ''turpi'':103C,247C ''ullamcorp'':32B ''ultric'':226C,458C ''ultrici'':33B ''urna'':50B,169C,207C,258C,342C,361C,380C ''ut'':12B,200C,214C,224C,335C,377C,422C ''varius'':88C ''vehicula'':146C ''vel'':28B,51B,74C,188C,232C,379C ''velit'':21B,139C,194C,421C,425C ''venenati'':357C,435C ''vestibulum'':360C ''vita'':251C,465C ''vivamus'':457C ''viverra'':19B,104C,155C ''volutpat'':36B,236C ''vulput'':229C,285C', 1, '
# Lorem ipsum dolor sit amet
consectetur adipiscing elit. Praesent id facilisis lacus. Maecenas vel dignissim ipsum. Phasellus mattis luctus orci, sit _amet gravida_ nunc scelerisque non. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Morbi non erat quis turpis viverra egestas dapibus et lorem. Quisque placerat est eros. Maecenas **pretium lobortis** orci, sit amet blandit leo porttitor ac. Suspendisse non risus et magna fermentum placerat. Donec sem leo, accumsan in sollicitudin a, imperdiet at velit. Fusce orci erat, fermentum ac neque vehicula, cursus facilisis tellus. Proin suscipit eu elit sed viverra. Phasellus lacinia enim eget libero tempus rutrum. Mauris iaculis, lectus at posuere congue, urna est efficitur tortor, eget tempor tellus ***ante et orci***.

Maecenas molestie at sapien tempus feugiat. Aliquam hendrerit, magna vel dapibus posuere, nisl mi accumsan velit, sit amet consequat massa ante ut est. Maecenas nec orci dolor. In urna eros, dictum quis eleifend a, egestas ut nibh. In hac habitasse platea dictumst. Integer tempus enim ut arcu ultrices auctor. Morbi vulputate pharetra lorem, vel imperdiet ipsum.

````plain
Curabitur volutpat elit et iaculis luctus. 
Curabitur et purus eros. 
Nunc nibh turpis, luctus quis nulla vitae, 
````

commodo cursus leo. Morbi eu convallis urna. Suspendisse nec erat in justo dignissim lobortis eget eget libero. Integer sed ante luctus, rhoncus lectus a, iaculis felis. Curabitur id leo efficitur, facilisis nibh a, vulputate est.

Donec placerat dignissim leo ac tempor. Sed lacinia eros odio, eget porttitor metus iaculis sed. Aenean id magna in eros convallis dapibus at nec est. Aliquam et purus tempor, tristique magna non, gravida est. Interdum et malesuada fames ac ante ipsum primis in faucibus. Morbi aliquam egestas risus, ut finibus dui suscipit quis. Nunc a urna sit amet tortor porta interdum sit amet id dui.

Suspendisse gravida odio quis dictum venenatis. Cras ac vestibulum urna, at facilisis neque. Mauris porta nisi in dapibus feugiat. Pellentesque suscipit id mi et egestas. Ut auctor vel urna quis faucibus. Nunc quis sem sollicitudin, pretium tortor et, malesuada nibh. Curabitur nec pretium nunc, iaculis dapibus diam. Praesent sodales iaculis augue, non tempus massa mollis ac. Nullam euismod, neque eu blandit sollicitudin, odio nisl tempus lacus, in euismod quam velit ut diam. Integer velit nisi, accumsan at magna at, dapibus sollicitudin nisi. Fusce venenatis sed arcu eu rutrum. Phasellus aliquam tortor sed mauris malesuada, eget congue lorem gravida. Praesent eleifend magna euismod elit tempus pellentesque. Vivamus ultrices nunc metus, scelerisque molestie ante maximus vitae. Nam consequat semper lacus nec sodales.');


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO users (userid, username, display, is_admin, hash_salt, attempts, lockout) VALUES (3, 'admin', 'Administrator', true, '$2a$08$PLVHtEhTeEJyrqkLcAcuI.sS2j5dnkullXj65Bzovxdcr9npNCI9O', 0, NULL);
INSERT INTO users (userid, username, display, is_admin, hash_salt, attempts, lockout) VALUES (1, 'andrew', 'Andrew Prindle', true, '$2a$08$nRdxxtl6HWTK9RuIuOQO2unb0bD4Dhiq6rS.5WVBlunHm2wQkr2Ty', 0, NULL);


--
-- Data for Name: users_old; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO users_old (userid, username, display, email, password, is_admin, is_public) VALUES (1, 'andrew', 'Andrew Prindle', 'prindle.andrew@gmail.com', 'd979885447a413abb6d606a5d0f45c3b7809e6fde2c83f0df3426f1fc9bfed97', true, true);


--
-- Name: articles_aid_seq; Type: SEQUENCE SET; Schema: public; Owner: -
--

SELECT pg_catalog.setval('articles_aid_seq', 15, true);


--
-- Name: users_userid_seq; Type: SEQUENCE SET; Schema: public; Owner: -
--

SELECT pg_catalog.setval('users_userid_seq', 3, true);


--
-- Name: articles articles_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY articles
    ADD CONSTRAINT articles_pkey PRIMARY KEY (aid);


--
-- Name: users constrait_users_username_unique; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY users
    ADD CONSTRAINT constrait_users_username_unique UNIQUE (username);


--
-- Name: users_old users_pk_userid; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY users_old
    ADD CONSTRAINT users_pk_userid PRIMARY KEY (userid);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY users
    ADD CONSTRAINT users_pkey PRIMARY KEY (userid);


--
-- Name: users_old users_unique_email; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY users_old
    ADD CONSTRAINT users_unique_email UNIQUE (email);


--
-- Name: users_old users_unique_username; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY users_old
    ADD CONSTRAINT users_unique_username UNIQUE (username);


--
-- Name: fulltxt_idx; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX fulltxt_idx ON articles USING gin (fulltxt);


--
-- Name: users trigger_blog_users_insert; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER trigger_blog_users_insert BEFORE INSERT ON users FOR EACH ROW EXECUTE PROCEDURE proc_blog_users_insert();


--
-- Name: users trigger_blog_users_update2; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER trigger_blog_users_update2 BEFORE UPDATE OF hash_salt ON users FOR EACH ROW EXECUTE PROCEDURE proc_blog_users_update();


--
-- Name: articles update_articles; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER update_articles BEFORE INSERT OR UPDATE ON articles FOR EACH ROW EXECUTE PROCEDURE fulltxt_articles_update();


--
-- PostgreSQL database dump complete
--

