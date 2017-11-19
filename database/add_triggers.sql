
CREATE EXTENSION IF NOT EXISTS pgcrypto WITH SCHEMA public;


CREATE OR REPLACE FUNCTION proc_blog_users_insert() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
    -- Hash the password with a newly generated salt
    -- crypt() will store the hash and salt (and the algorithm and iterations) in the column
    new.hash_salt := crypt(new.hash_salt, gen_salt('bf', 8));
  return new;
end
$$;


CREATE OR REPLACE FUNCTION proc_blog_users_update() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
  IF new.hash_salt = NULL THEN
    new.hash_salt := old.hash_salt;
  ELSE
    new.hash_salt := crypt(new.hash_salt, old.hash_salt);
  END IF;
  return new;
end
$$;


CREATE TRIGGER trigger_blog_users_insert BEFORE INSERT ON users FOR EACH ROW EXECUTE PROCEDURE proc_blog_users_insert();
CREATE TRIGGER trigger_blog_users_update BEFORE UPDATE ON users FOR EACH ROW EXECUTE PROCEDURE proc_blog_users_update();









