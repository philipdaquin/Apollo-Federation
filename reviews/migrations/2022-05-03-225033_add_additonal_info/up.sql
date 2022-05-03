-- Your SQL goes here

ALTER TABLE review 
ADD     heading VARCHAR(250),
ADD     updated_at TIMESTAMP DEFAULT NOW(),
ADD     media VARCHAR,
ADD     is_edited BOOLEAN;
 