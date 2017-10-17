SELECT 
    articles.aid, articles.created, articles.modified, articles.title, articles.body, 
    users.userid, users.display_name, 
    categories.name

FROM articles 
    INNER JOIN users ON (articles.author = users.userid)
    LEFT JOIN categories ON (articles.catid = categories.catid)