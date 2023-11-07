-- Your SQL goes here
create table public.hf_dict
(
    id          serial
        primary key,
    "group"     smallint default 0 not null,
    code        varchar(10),
    name        varchar(100),
    remark      varchar(500),
    create_time timestamp(3)       not null
);

comment on table public.hf_dict is '字典表';

comment on column public.hf_dict.id is '主键';

comment on column public.hf_dict."group" is '分组';

comment on column public.hf_dict.code is '编号';

comment on column public.hf_dict.name is '名称';

comment on column public.hf_dict.remark is '描述';

alter table public.hf_dict
    owner to oasis_inc;

create unique index udx_group_code
    on public.hf_dict ("group", code);

