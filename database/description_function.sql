-- Use:
-- SELECT aid, title, description(300, body, description), description, body as description FROM articles

CREATE OR REPLACE FUNCTION description(chars int, body text, short text = NULL) RETURNS text AS $$
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
$$ LANGUAGE plpgsql;