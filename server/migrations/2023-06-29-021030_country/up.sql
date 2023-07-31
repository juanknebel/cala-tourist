create table country
(
    id          serial
        constraint country_pk
            primary key,
    iso_code    varchar not null
        constraint country_pk2
            unique,
    description varchar not null
);

alter table country
    owner to postgres;