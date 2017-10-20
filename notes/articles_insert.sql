INSERT INTO public.articles(
	title, posted, body, tags)
	VALUES ('my insert title', '2017-10-20 1:45pm', 'this is a body', 'these are tags')
    RETURNING aid;