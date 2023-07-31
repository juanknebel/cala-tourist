create table attraction
(
    id                 serial
        constraint attraction_pk
            primary key,
    description        varchar not null,
    city_id            integer not null
        constraint attraction_city_id_fk
            references city,
    latitude        varchar,
    longitude       varchar,
    attraction_type_id integer not null
        constraint attraction_attraction_type_id_fk
            references attraction_type
);

alter table attraction
    owner to postgres;
