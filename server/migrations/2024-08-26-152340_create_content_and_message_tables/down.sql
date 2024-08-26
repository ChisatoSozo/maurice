DROP INDEX IF EXISTS idx_user_facts;
DROP INDEX IF EXISTS idx_user_wake_word_samples;
DROP INDEX IF EXISTS idx_content_types;
DROP INDEX IF EXISTS idx_message_contents;

DROP TABLE IF EXISTS user_facts;
DROP TABLE IF EXISTS user_wake_word_samples;
DROP TABLE IF EXISTS message_contents;
DROP TABLE IF EXISTS messages;
DROP TABLE IF EXISTS "users";
DROP TABLE IF EXISTS contents;