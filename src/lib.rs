use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::log;
use near_sdk::near_bindgen;
use near_sdk::serde::{Deserialize, Serialize};
use std::collections::HashMap;
// #[near_bindgen]
#[derive(Debug, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Student {
  pub name: String,
  pub gender: String,
  pub age: u32,
}

// #[near_bindgen]
impl Student {
  pub fn new_student(name: String, gender: String, age: u32) -> Student {
    Self {
      name: name,
      gender: gender,
      age: age,
    }
  }
}
// #[near_bindgen]
#[derive(Debug, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Course {
  pub course_name: String,
  pub units: u32,
  pub students: Vec<Student>,
}
// #[near_bindgen]
impl Course {
  pub fn new_course(course_name: String, units: u32) -> Course {
    Self {
      course_name: course_name,
      units: units,
      students: Vec::new(),
    }
  }

  fn add_student_to_course(&mut self, student: Student) {
    self.students.push(student)
  }
  fn view_students_in_course(&self) -> Vec<Student> {
    self.students.to_vec()
  }
}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Department {
  courses: HashMap<String, Option<Course>>,
}
impl Default for Department {
  fn default() -> Self {
    Self {
      courses: HashMap::new(),
    }
  }
}
#[near_bindgen]
impl Department {
  pub fn new_dep(dep_name: String) -> Result<String, String> {
    let mut dep = Self {
      courses: HashMap::new(),
    };
    let name = dep_name.clone();
    match dep.courses.insert(dep_name, None) {
      Some(_) => Ok(format!("{}department created succefully ", &name)),
      None => Err(format!("failed to create {} department ", &name)),
    }
  }

  pub fn add_course_to_dep(&mut self, course: Course) {
    self
      .courses
      .insert(course.course_name.clone(), Some(course));
  }
  pub fn view_courses(&mut self) -> Vec<&Course> {
    let mut v = Vec::new();
    let courses = &mut self.courses;
    courses.into_iter().for_each(|c| match c.1.as_ref() {
      Some(c) => {
        v.push(c.clone());
      }
      None => println!("no courses that  exist"),
    });

    v
  }
  pub fn add_new_student_to_dep_course(
    &mut self,
    student: Student,
    course_name: &String,
  ) -> Result<(), String> {
    match self.courses.get_mut(course_name) {
      Some(course) => {
        course.as_mut().unwrap().add_student_to_course(student);
        // let mut my_course = course.unwrap().add_student_to_course(student);
        // my_course.students.push(student);
        Ok(())
      }
      None => Err(format!("no such course ")),
    }
  }

  pub fn view_students_in_dep_course(
    &mut self,
    course_name: &String,
  ) -> Result<Vec<Student>, String> {
    match self.courses.get(course_name) {
      Some(c) => {
        let students_in_course = c.as_ref().unwrap().view_students_in_course();
        Ok(students_in_course)
      }
      None => Err(String::from("no results")),
    }
  }
}

mod tests{
  // use ::*;
}