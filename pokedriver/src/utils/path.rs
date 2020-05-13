pub fn resolve_sprite_path(pokemon: &String) -> String {
    "sprites/".to_owned() + pokemon.to_string().as_ref() + &"/".to_owned()
}


