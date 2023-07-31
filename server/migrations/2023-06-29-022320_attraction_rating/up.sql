create table attraction_rating
(
    id                 serial
        constraint attraction_rating_pk
            primary key,
    at timestamp not null,
    attraction_id            integer not null
        constraint attraction_id_fk
            references attraction,
    rate decimal not null
);

alter table attraction_rating
    owner to postgres;
