-- This file should undo anything in `up.sql`
DROP TABLE users;
ALTER TABLE users ALTER verified DROP DEFAULT;