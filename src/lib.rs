use j4rs::{ClasspathEntry, JvmBuilder};

fn hmm() {
    let entry = ClasspathEntry::new("java/");
    let jvm = JvmBuilder::new()
        // .with_default_classloader()
        .classpath_entry(entry)
        .build()
        .unwrap();
    let instance = jvm.create_instance("Hmm", &[]).unwrap();
    jvm.invoke(&instance, "hmm", &[]).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        hmm();
    }

    #[test]
    fn test2() {
        hmm();
    }
}
