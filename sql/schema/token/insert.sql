INSERT INTO token 
(id, user_id, time_created, last_updated, auth_token, refresh_token)
values (NULL, ?, NOW(), NOW(), ?, ?)