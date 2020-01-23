-- Your SQL goes here

alter table items alter column link drop default;
UPDATE items SET link = concat(link, '?ref=', id);
ALTER TABLE items ADD CONSTRAINT items_link_key UNIQUE (link);