-- Your SQL goes here

alter table items alter column link drop default;
alter table items drop CONSTRAINT items_title_key;

UPDATE items SET link = concat(link, '?ref=', id);
ALTER TABLE items ADD CONSTRAINT items_link_key UNIQUE (link);
