-- Add migration script here
CREATE TYPE llm_models AS ENUM ('gpt-4', 'gemini');

CREATE TABLE api_keys (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR(100) NOT NULL,
    key TEXT NOT NULL,
    llm_model llm_models NOT NULL
);