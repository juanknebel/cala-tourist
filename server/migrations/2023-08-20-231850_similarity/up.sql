-- Your SQL goes here
create table attraction_similarity
(
    id                 serial
        constraint attraction_similarity_pk
            primary key,
    attraction_id integer not null
        constraint attraction_similarity_attraction_id_fk
            references attraction,
    to_attraction_id integer not null
        constraint attraction_similarity_to_attraction_id_fk
            references attraction,
    similarity decimal not null,
    at timestamp not null
);

alter table attraction_similarity
    owner to postgres;
