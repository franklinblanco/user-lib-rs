DELETE FROM token
where 
TIMESTAMPDIFF(DAY, NOW(), last_updated) > ? AND
TIMESTAMPDIFF(DAY, NOW(), time_created) > ?