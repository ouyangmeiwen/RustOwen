-- public.libitem definition

-- Drop table

-- DROP TABLE public.libitem;

CREATE TABLE public.libitem (
	id varchar(32) NOT NULL,
	creation_time timestamp(6) NOT NULL,
	creator_user_id int8 NULL,
	last_modification_time timestamp(6) DEFAULT NULL::timestamp without time zone NULL,
	last_modifier_user_id int8 NULL,
	is_deleted bool NOT NULL,
	deleter_user_id int8 NULL,
	deletion_time timestamp(6) DEFAULT NULL::timestamp without time zone NULL,
	info_id varchar(32) DEFAULT NULL::character varying NULL,
	title varchar(512) NOT NULL,
	author varchar(512) DEFAULT NULL::character varying NULL,
	barcode varchar(32) NOT NULL,
	is_enable bool NOT NULL,
	call_no varchar(64) DEFAULT NULL::character varying NULL,
	pre_call_no varchar(64) DEFAULT NULL::character varying NULL,
	catalog_code varchar(32) DEFAULT NULL::character varying NULL,
	item_state int2 NOT NULL,
	pressmark_id varchar(32) DEFAULT NULL::character varying NULL,
	pressmark_name varchar(64) DEFAULT NULL::character varying NULL,
	location_id varchar(32) DEFAULT NULL::character varying NULL,
	location_name varchar(128) DEFAULT NULL::character varying NULL,
	book_barcode varchar(32) DEFAULT NULL::character varying NULL,
	isbn varchar(32) DEFAULT NULL::character varying NULL,
	pub_no int2 NULL,
	publisher varchar(512) DEFAULT NULL::character varying NULL,
	pub_date varchar(512) DEFAULT NULL::character varying NULL,
	price varchar(32) DEFAULT NULL::character varying NULL,
	pages varchar(32) DEFAULT NULL::character varying NULL,
	summary text NULL,
	item_type int2 NOT NULL,
	remark varchar(256) DEFAULT NULL::character varying NULL,
	origin_type int2 NOT NULL,
	create_type int2 NOT NULL,
	tenant_id int4 NOT NULL,
	CONSTRAINT ix_libitem_tenantid_isdeleted_barcode UNIQUE (tenant_id, is_deleted, barcode),
	CONSTRAINT ix_libitem_tenantid_isdeleted_creationtime UNIQUE (tenant_id, is_deleted, creation_time),
	CONSTRAINT libitem_pkey PRIMARY KEY (id)
);