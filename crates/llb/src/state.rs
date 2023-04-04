use buildkit_rs_proto::pb;

use crate::azync::AsyncState;
use crate::sourcemap::SourceLocation;
use crate::Digest;
use crate::Error;

pub trait Output {
    fn to_input(&self, constraints: &Constraints) -> Result<pb::Input, Error>;
    fn vertex(&self, constraints: &Constraints) -> dyn Vertex;
}

pub trait Vertex {
    fn validate(&self, constraints: &Constraints) -> Result<(), Error>;
    fn marshal(
        &self,
        constraints: &Constraints,
    ) -> Result<(Digest, Vec<u8>, pb::OpMetadata, Vec<&SourceLocation>), Error>;
    fn output(&self) -> dyn Output;
    fn inputs(&self) -> Vec<Box<dyn Output>>;
}

pub struct State {
    out: Box<dyn Output>,
    prev: Option<Box<State>>,
    key: Option<Box<dyn std::any::Any>>,
    // value: Box<dyn Fn(&Constraints) -> Result<Option<std::any::Any>, Error>>,
    opts: Vec<Box<dyn ConstraintsOpt>>,
    async_state: Option<Box<AsyncState>>,
}

impl State {
    fn with_value(self) -> Self {
        Self {
            out: self.out,
            prev: Some(Box::new(self)),
            key: todo!(),
            opts: todo!(),
            async_state: todo!(),
        }
    }

    fn add_env(self, key: String, value: String) -> Self {
        return s.withValue(keyEnv, func(ctx context.Context, c *Constraints) (interface{}, error) {
            env, err := getEnv(s)(ctx, c)
            if err != nil {
                return nil, err
            }
            return env.AddOrReplace(key, value), nil
        })
    }
}

trait ConstraintsOpt {
    // stuff
}

pub struct Constraints {
    // platform: Option<&'a ocispecs::Platform>,
    worker_constraints: Vec<String>,
    metadata: pb::OpMetadata,
    local_unique_id: String,
    // caps: Option<&'a apicaps::CapSet>,
    source_locations: Vec<SourceLocation>,
}

#[cfg(test)]
mod tests {
    use crate::source::Image;

    use super::*;

    #[test]
    fn test_state_meta() {
        let mut s = Image::new("foo".to_owned());
        s = s.add_env("BAR", "abc").dir("/foo/bar");

        let (v, ok) = get_env_helper(&s, "BAR");
        assert!(ok);
        assert_eq!("abc", v);

        assert_eq!("/foo/bar", get_dir_helper(&s));

        let mut s2 = Image::new("foo2".to_owned());
        s2 = s2.add_env("BAZ", "def").reset(&s);

        let (_, ok) = get_env_helper(&s2, "BAZ");
        assert!(!ok);

        let (v, ok) = get_env_helper(&s2, "BAR");
        assert!(ok);
        assert_eq!("abc", v);
    }

    #[test]
    fn test_formatting_patterns() {
        let s = Image::new("foo".to_owned())
            .add_env("FOO", "ab%sc")
            .dir("/foo/bar%d");

        let (v, ok) = get_env_helper(&s, "FOO");
        assert!(ok);
        assert_eq!("ab%sc", v);

        assert_eq!("/foo/bar%d", get_dir_helper(&s));

        let mut s2 = Image::new("foo".to_owned());
        s2 = s2.add_envf("FOO", "ab%sc", "__")
            .dirf("/foo/bar%d", 1);

        let (v, ok) = get_env_helper(&s2, "FOO");
        assert!(ok);
        assert_eq!("ab__c", v);

        assert_eq!("/foo/bar1", get_dir_helper(&s2));
    }

    fn get_env_helper(k: &str, s: &State) -> (String, bool) {
        s.get_env(k).unwrap_or((String::new(), false))
    }

    fn get_dir_helper(s: &State) -> &str {
        s.get_dir().unwrap()
    }
}
