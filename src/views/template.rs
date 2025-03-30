use minijinja::Environment;

pub fn add_template(env: &mut Environment<'_>) {
    env.add_template("main", include_str!("./layout/main.jinja"))
        .unwrap();
    env.add_template("menu", include_str!("./layout/menu.jinja"))
        .unwrap();
    env.add_template("task", include_str!("./task/create.jinja"))
    .unwrap();
    env.add_template("home", include_str!("./home.jinja"))
        .unwrap();
}
