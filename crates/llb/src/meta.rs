fn add_envf(key: String, value: String, replace: bool, v: &[impl std::fmt::Display]) -> StateOption {
    if replace {
        let value = format!(value, v);
    }
    Box::new(move |s: State| -> State {
        s.with_value(key_env, move |ctx: context::Context, c: &mut Constraints| -> Result<_, _> {
            let env = get_env(s)(ctx, c)?;
            let new_env = env.add_or_replace(&key, &value);
            Ok(new_env)
        })
    })
}