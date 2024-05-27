-- This file should undo anything in `up.sql`
ALTER TABLE users ALTER verified DROP DEFAULT;
DROP TABLE users;
DROP TABLE saved_automata;
DROP TABLE saved_states;
DROP TABLE saved_connections