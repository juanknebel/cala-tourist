create table city
(
    id          serial
        constraint city_pk
            primary key,
    description varchar not null,
    country_id  integer not null
        constraint city_country_id_fk
            references country
);

alter table city
    owner to postgres;

