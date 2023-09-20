SELECT *
FROM user
WHERE user.credential = ? AND
user.credential_type = ? AND
user.app = ?