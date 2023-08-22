-- Your SQL goes here
create table attraction_rating_aggregate
(
    id                 serial
        constraint attraction_rating_aggregate_pk
            primary key,
    attraction_id integer not null
        constraint attraction_rating_aggregate_attraction_id_fk
            references attraction,
    at timestamp not null,
    average decimal not null,
    ninety_five_percentile decimal not null,
    ninety_nine_percentile decimal not null
);

alter table attraction_rating_aggregate
    owner to postgres;
