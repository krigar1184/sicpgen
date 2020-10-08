use itertools::Itertools;
#[macro_use] extern crate lazy_static;
use tera::{Context, Tera};
use std::fmt;
use std::path::PathBuf;
use std::fs::create_dir_all;
use failure::Fail;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        match Tera::new("src/templates/**/*.tpl") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                std::process::exit(1);
            },
        }
    };
}

#[derive(Debug, Fail)]
pub enum GenerationError {
    RootDoesNotExistError(PathBuf),
    ExerciseExists(PathBuf)
}

impl fmt::Display for GenerationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            GenerationError::RootDoesNotExistError(v) => format!("Path provided ({}) doesn\'t exist", v.to_str().unwrap()),
            GenerationError::ExerciseExists(v) => format!("Exercise {} already exists", v.display()),
        };

        write!(f, "{}", msg)
    }
}

#[derive(Debug, Fail)]
pub struct TestingError {
}

impl fmt::Display for TestingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to run test suite")
    }
}

#[derive(Debug)]
pub struct Exercise {
    chapter: String,
    exercise: String,
    path: PathBuf,
}

impl fmt::Display for Exercise {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "chapter {} exercise {}", self.chapter, self.exercise)
    }
}

impl Exercise {
    fn new(root: &PathBuf, number: &str) -> Exercise {
        let (chapter, exercise_number) = number.split('-').collect_tuple().unwrap();
        let path = root.join(format!("chapter-{}", chapter)).join(format!("exercise-{}", exercise_number));
        Exercise { chapter: String::from(chapter), exercise: String::from(exercise_number), path }
    }

    fn generate(&self) -> Result<(), GenerationError> {
        if self.path.exists() {
            return Err(GenerationError::ExerciseExists((*self.path).to_path_buf()));
        }

        if let Some(v) = self.path.to_str() {
            create_dir_all(v).unwrap();
        }

        self.generate_readme().unwrap();
        self.generate_test().unwrap();

        Ok(())
    }

    fn generate_readme(&self) -> std::io::Result<()> {
        let mut context = Context::new();
        context.insert("chapter", &self.chapter);
        context.insert("exercise", &self.exercise);
        let content = TEMPLATES.render("readme.md.tpl", &context).unwrap();
        let filename = self.path.join("README.md");
        std::fs::write(filename, content)
    }

    fn generate_test(&self) -> std::io::Result<()> {
        let context = Context::new();
        let content = TEMPLATES.render("test.rkt.tpl", &context).unwrap();
        let filename = self.path.join("test.rkt");
        std::fs::write(filename, content)
    }
}

pub fn generate(root: &PathBuf, number: &str) -> Result<(), GenerationError> {
    if !root.exists() {
        return Err(GenerationError::RootDoesNotExistError(root.to_owned()));
    }

    Exercise::new(root, number).generate()
}

pub fn test(_root: &PathBuf, _number: &str) -> Result<(), TestingError> {
    Ok(())
}

#[test]
fn test_generate_exercise() -> Result<(), std::io::Error> {
    use tempfile::tempdir;
    let root = tempdir()?.into_path();

    generate(&root, "1-1").unwrap();

    let exercise_root = root.join("chapter-1").join("exercise-1");
    assert_eq!(exercise_root.exists(), true);
    assert_eq!(exercise_root.join("README.md").exists(), true);
    assert_eq!(exercise_root.join("test.rkt").exists(), true);

    Ok(())
}
