UPDATE token
SET last_updated = NOW(), auth_token = ?, refresh_token = ?
WHERE id = ?