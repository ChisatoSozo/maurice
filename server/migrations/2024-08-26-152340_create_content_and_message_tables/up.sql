-- Create content table (unchanged)
CREATE TABLE contents (
    id SERIAL PRIMARY KEY,
    type TEXT NOT NULL,
    text_content TEXT,
    binary_data BYTEA,
    mime_type VARCHAR(255) NOT NULL
);

-- Create user table (modified)
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

-- Create user_wake_word_samples table (unchanged)
CREATE TABLE user_wake_word_samples (
    user_id INTEGER REFERENCES "users"(id),
    content_id INTEGER REFERENCES contents(id),
    PRIMARY KEY (user_id, content_id)
);

-- Create user_facts_to_remember table (new)
CREATE TABLE user_facts (
    user_id INTEGER REFERENCES "users"(id),
    content_id INTEGER REFERENCES contents(id),
    PRIMARY KEY (user_id, content_id)
);

-- Create message table (unchanged)
CREATE TABLE messages (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES "users"(id),
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Create message_content join table (unchanged)
CREATE TABLE message_contents (
    message_id INTEGER REFERENCES messages(id),
    content_id INTEGER REFERENCES contents(id),
    PRIMARY KEY (message_id, content_id)
);

-- Create index for faster lookups on message content (unchanged)
CREATE INDEX idx_message_contents ON message_contents (message_id, content_id);

-- Create index for content type (unchanged)
CREATE INDEX idx_content_types ON contents (type);

-- Create index for user wake word samples (unchanged)
CREATE INDEX idx_user_wake_word_samples ON user_wake_word_samples (user_id, content_id);

-- Create index for user facts to remember (new)
CREATE INDEX idx_user_facts ON user_facts (user_id, content_id);