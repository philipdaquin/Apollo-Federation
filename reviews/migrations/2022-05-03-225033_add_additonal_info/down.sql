-- This file should undo anything in `up.sql`

ALTER TABLE review DROP COLUMN  heading;
ALTER TABLE review DROP COLUMN  updated_at;
ALTER TABLE review DROP COLUMN  media;
ALTER TABLE review DROP COLUMN  is_edited;