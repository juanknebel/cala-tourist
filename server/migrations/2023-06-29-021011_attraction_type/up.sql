create table attraction_type
(
    id          serial
        constraint attraction_type_pk
            primary key,
    code        varchar not null,
    description varchar not null
);

alter table attraction_type
    owner to postgres;