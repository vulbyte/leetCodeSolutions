// SELECT 
//   query_name,
//   ROUND(AVG(rating / position), 2) AS quality,
//   ROUND(AVG(rating < 3) * 100, 2) AS poor_query_percentage
// FROM
//   Queries
// GROUP BY
//   query_name

SELECT query_name,
       ROUND(AVG(CAST(rating AS DECIMAL) / position), 2) AS quality,
       ROUND((SUM(CASE WHEN rating < 3 THEN 1 ELSE 0 END) / COUNT(*)) * 100, 2) AS poor_query_percentage
FROM Queries
WHERE query_name IS NOT NULL
GROUP BY query_name;
