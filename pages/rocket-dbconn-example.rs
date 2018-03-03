#[get("/database")]
pub fn database(conn: DbConn) -> Html&lt;String&gt; {
    
    // For queries returning multiple rows:
    let query = conn.query("SELECT cog FROM widgets", &[]);
    let mut output = String::new();
    if let Ok(qry) = query {
        for row in &qry {
            let cog = row.get(0);
            let html = format!("Cog: {}<br>", cog);
            output.push(&html);
        }
    }
    
    // For queries returning a single row:
    let query = conn.query("SELECT username FROM users WHERE user = 'phillip'", &[]);
    if let Ok(qry) = query {
        if !qry.is_empty() && qry.len() == 1 {
            let result = qry.get(0); // Grab the first row
            format!("Hello {}", result.get(0)) // Grab the first field of the row
        } else {
            "Database error: invalid results".to_owned()
        }
    } else {
        "Database error: query failed".to_owned()
    }
    
    
}