use minijinja::Environment;

pub fn add_template(env: &mut Environment<'_>) {
    env.add_template("main", include_str!("./layout/main.html"))
        .unwrap();
    env.add_template("menu", include_str!("./layout/menu.html"))
        .unwrap();
    env.add_template("task.index", include_str!("./task/index.html"))
    .unwrap();
    env.add_template("home", include_str!("./home.html"))
        .unwrap();
}
