create table pokemons (
    number integer primary key,
    name text
);

create table types (
    pokemon_number integer,
    name text,
    foreign key (pokemon_number) references pokemons (number) on delete cascade,
    primary key (pokemon_number, name)
);