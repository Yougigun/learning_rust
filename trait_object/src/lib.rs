use std::{error::Error, process::Command};

pub type BoxedError = Box<dyn Error + Send + Sync>;

pub trait Executor {
    fn run(&self) -> Result<Option<i32>, BoxedError>;
}

pub struct Shell<'a, 'b> {
    cmd: &'a str,
    args: &'b [&'a str],
}

impl<'a, 'b> Shell<'a, 'b> {
    pub fn new(cmd: &'a str, args: &'b [&'a str]) -> Self {
        Self { cmd, args }
    }
}

impl<'a, 'b> Executor for Shell<'a, 'b> {
    fn run(&self) -> Result<Option<i32>, BoxedError> {
        let output = Command::new(self.cmd).args(self.args).output()?;
        Ok(output.status.code())
    }
}

/// 使用泛型参数
pub fn execute_generics(cmd: &impl Executor) -> Result<Option<i32>, BoxedError> {
    cmd.run()
}

/// 使用 trait object: &dyn T <- reference to trait object. data might still be on stack
pub fn execute_trait_object(cmd: &dyn Executor) -> Result<Option<i32>, BoxedError> {
    cmd.run()
}

/// 使用 trait object: Box<dyn T> <- boxed trait object. data is on heap
pub fn execute_boxed_trait_object(cmd: Box<dyn Executor>) -> Result<Option<i32>, BoxedError> {
    cmd.run()
}

#[cfg(test)]
mod tests {
    use std::error;

    use super::*;

    #[test]
    fn shell_shall_work() {
        let cmd = Shell::new("ls", &[]);
        let result = cmd.run().unwrap();
        assert_eq!(result, Some(0));
    }

    #[test]
    fn execute_shall_work() {
        let cmd = Shell::new("ls", &[]);
        let result = execute_generics(&cmd).unwrap();
        assert_eq!(result, Some(0));
        let result = execute_trait_object(&cmd).unwrap();
        assert_eq!(result, Some(0));
        let boxed = Box::new(cmd);
        let result = execute_boxed_trait_object(boxed).unwrap();
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test2() {
        trait Turn {
            fn turn(self);
        }
        #[derive(Debug)]
        struct Circle;

        // impl trait for borrow type so that we dont consume the value even if signature is not &self
        impl Turn for &Circle {
            fn turn(self) {
                println!("Circle");
            }
        }

        let c1 = Circle;
        // let c2 = &c1;
        Turn::turn(&c1);
        <&Circle as Turn>::turn(&c1);
        println!("{:?}", c1)
    }

    #[test]
    #[allow(dead_code)]
    fn kv_server_example() {
        use std::{error::Error, sync::Arc};

        // 定义类型，让 KV server 里的 trait 可以被编译通过
        pub type KvError = Box<dyn Error + Send + Sync>;
        pub struct Value(i32);
        pub struct Kvpair(i32, i32);

        /// 对存储的抽象，我们`不关心数据存在哪儿`，但需要定义`外界如何和存储打交道``
        /// why static lifetime? because we dont know how long the storage will live, 
        /// so the type implementing this trait should be able to live forever
        pub trait Storage: Send + Sync + 'static {
            fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
            fn set(&self, table: &str, key: String, value: Value)
                -> Result<Option<Value>, KvError>;
            fn contains(&self, table: &str, key: &str) -> Result<bool, KvError>;
            fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
            fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError>;
            fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError>;
        }

        // 使用 trait object，不需要泛型参数，也不需要 ServiceInner 了
        pub struct Service {
            pub store: Arc<dyn Storage>,
        }

        // impl 的代码略微简单一些
        impl Service {
            pub fn new<S: Storage>(store: S) -> Self {
                Self {
                    store: Arc::new(store),
                }
            }
        }

        // 实现 trait 时也不需要带着泛型参数
        impl Clone for Service {
            fn clone(&self) -> Self {
                Self {
                    store: Arc::clone(&self.store),
                }
            }
        }
    }
}
