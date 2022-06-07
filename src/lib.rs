use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
// use near_sdk::bs58::decode::Result;
use near_sdk::near_bindgen;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, log};
use std::collections::HashMap;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ContractErr(String);
//creating a variable for gender
#[derive(Debug, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum Gender {
  Male,
  Female,
  Undefined,
}
#[derive(Debug, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum EducationLevel {
  Phd,
  Masters,
  Bachelors,
  Diploma,
  Certificate,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Student {
  pub name: String,
  pub gender: Gender,
  pub age: u32,
  pub education_level: EducationLevel,
}
#[derive(Debug, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Students(Vec<Student>);
impl Student {
  // this a function to create  new usr
  fn new_student(
    name: String,
    gender: Gender,
    age: u32,
    education_level: EducationLevel,
  ) -> Student {
    Self {
      name: name,
      gender: gender,
      age: age,
      education_level: education_level,
    }
  }
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Course {
  pub course_name: String,
  pub units: u32,
  pub students: Vec<Student>,
}

impl Course {
  //creating a new course
  fn new_course(course_name: String, units: u32) -> Course {
    Self {
      course_name: course_name,
      units: units,
      students: Vec::new(),
    }
  }
  //function to add a stuent to a course
  fn add_student_to_course(&mut self, student: Student) {
    self.students.push(student)
  }

  ///viewing studenteds taking in a current course
  fn view_students_in_course(&self) -> Vec<Student> {
    self.students.to_vec()
  }
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Serialize, Deserialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Department {
  pub department_name: String,
  pub courses: HashMap<String, Course>,
}

impl Department {
  //new department
  fn new_department(dep_name: String) -> Self {
    Self {
      department_name: dep_name,
      courses: HashMap::new(),
    }
  }
  //adding a new department to a department
  fn add_course(&mut self, course: Course) -> Result<(), ContractErr> {
    match self.courses.insert(course.course_name.clone(), course) {
      Some(_) => {
        //logs out if was succefull
        log!("Courese added succefully");
        Ok(())
      }
      None => Err(ContractErr("Faild to add Course".to_string())),
    }
  }
  //view  all courses in a department
  fn view_courses(&self) -> Vec<&Course> {
    let c = &self.courses;
    //collects all courses in a deparment
    let courses: Vec<&Course> = c.into_iter().map(|x| x.1).collect();
    courses
  }
  fn add_new_student_to_dep_course(
    &mut self,
    course_name: &String,
    student_name: String,
    gender: Gender,
    age: u32,
    education: EducationLevel,
  ) -> Result<(), ContractErr> {
    match self.courses.get_mut(course_name) {
      Some(course) => {
        let student = Student::new_student(student_name, gender, age, education);
        course.add_student_to_course(student);
        log!("student added succefully ");
        Ok(())
      }
      None => Err(ContractErr(format!("no such course "))),
    }
  }
  fn view_students_in_dep_course(&mut self, course_name: &String) -> Result<Students, ContractErr> {
    match self.courses.get(course_name) {
      Some(c) => {
        let students_in_course = c.view_students_in_course();
        // let students_in_course = c.as_ref().unwrap().view_students_in_course();
        Ok(Students(students_in_course))
      }
      None => Err(ContractErr(String::from("no results"))),
    }
  }
  fn view_all_student_in_deps(&mut self) -> Result<Students, ContractErr> {
    let mut students = Students(Vec::new());
    let c = &self.courses;
    let coureses: Vec<Course> = c.into_iter().map(|d| d.1.clone()).collect();
    // let students: Vec<Students> = coureses.into_iter().map(|s| s.students)
    for course in coureses.into_iter() {
      let mut students_in_course = course.students.clone();
      students.0.append(&mut students_in_course)
    }

    Ok(students)
  }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct University {
  university: String,
  university_account_id: String,
  // courses: HashMap<String, Option<Course>>,
  departments: HashMap<String, Department>,
}
// //implementing default for the main contract
impl Default for University {
  fn default() -> Self {
    Self {
      university: "Neural university".to_string(),
      university_account_id: env::current_account_id().to_string(),

      departments: HashMap::new(),
    }
  }
}
#[near_bindgen]
impl University {
  pub fn create_university(unversity_name: String) -> Self {
    //creating a new intance of of the university

    Self {
      university: unversity_name,
      university_account_id: env::signer_account_id().to_string(),
      departments: HashMap::new(),
    }
  }
  pub fn add_new_department(&mut self, department: String) -> Result<(), ContractErr> {
    let dept = Department::new_department(department.clone());

    match self.departments.insert(dept.department_name.clone(), dept) {
      Some(_) => {
        log!(format!(
          "{} department has  been succefully added",
          department
        ));
        Ok(())
      }
      None => Err(ContractErr("failed to add department".to_string())),
    }
  }

  pub fn add_course_to_dep(
    &mut self,
    dep_name: String,
    course_name: String,
    units: u32,
  ) -> Result<(), ContractErr> {
    //inserting a course to  a certain dep
    match self.departments.get_mut(&dep_name) {
      Some(dep) => {
        let course = Course::new_course(course_name.clone(), units);
        let result = dep.add_course(course);
        result
      }
      None => Err(ContractErr("No such department".to_string())),
    }
  }

  pub fn add_new_student_to_dep_course(
    &mut self,
    dep_name: &String,
    course_name: &String,
    student_name: String,
    gender: Gender,
    age: u32,
    education: EducationLevel,
  ) -> Result<(), ContractErr> {
    match self.departments.get_mut(dep_name) {
      Some(dep) => {
        let result =
          dep.add_new_student_to_dep_course(course_name, student_name, gender, age, education);

        result
      }
      None => Err(ContractErr(format!("no such course "))),
    }
  }

  pub fn view_students_in_dep_course(
    &mut self,
    dep_name: &String,
    course_name: &String,
  ) -> Result<Students, ContractErr> {
    match self.departments.get_mut(dep_name) {
      Some(d) => {
        let students_in_course = d.view_students_in_dep_course(course_name);
        students_in_course
      }
      None => Err(ContractErr(String::from("no results"))),
    }
  }
  pub fn view_students_in_dep(&mut self, department: &String) -> Result<Students, ContractErr> {
    match self.departments.get_mut(department) {
      Some(dep) => dep.view_all_student_in_deps(),
      None => Err(ContractErr("this department is not available".to_string())),
    }
  }
  pub fn view_all_courses(&mut self) -> Option<Vec<&Course>> {
    let mut all_courses: Vec<&Course> = Vec::new();
    let deps = self.departments.values();

    for i in deps.into_iter() {
      let mut courses = i.view_courses();
      all_courses.append(&mut courses)
    }
    Some(all_courses)
  }
  pub fn view_all_students(&mut self) -> Result<Students, ContractErr> {
    let mut all_studemts = Students(Vec::new());
    let deps = &mut self.departments;
    let mut s: Vec<&mut Department> = deps.iter_mut().map(|d| d.1).collect();
    for d in s.iter_mut() {
      let mut st = d.view_all_student_in_deps()?;
      all_studemts.0.append(&mut st.0);
    }
    Ok(all_studemts)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use near_sdk::test_utils::VMContextBuilder;
  use near_sdk::{testing_env, AccountId, VMContext};

  const ONE_NEAR: u128 = u128::pow(10, 24);

  fn contract_account() -> AccountId {
    "contract.testnet".parse::<AccountId>().unwrap()
  }

  fn get_context(predecessor_account_id: AccountId) -> VMContext {
    let mut builder = VMContextBuilder::new();
    builder
      .current_account_id(contract_account())
      .account_balance(15 * ONE_NEAR)
      .signer_account_id(predecessor_account_id.clone())
      .predecessor_account_id(predecessor_account_id);
    builder.build()
  }
  #[test]
  fn test() {
    /* this test tests if the create_unknown_word function  hides the word  if the player has not yet  revealed it */
    let accountid = AccountId::new_unchecked("onchez.test".to_string());
    let context = get_context(accountid);
    testing_env!(context);
  }
  #[test]
  fn test_new_student() {
    /* this test tests if the create_unknown_word function  hides the word  if the player has not yet  revealed it */
    let accountid = AccountId::new_unchecked("onchez.test".to_string());
    let context = get_context(accountid);
    testing_env!(context);

    let student = Student {
      name: "onchez".to_string(),
      gender: Gender::Male,
      age: 32,
      education_level: EducationLevel::Bachelors,
    };

    assert_eq!(
      student,
      Student::new_student(
        "onchez".to_string(),
        Gender::Male,
        32,
        EducationLevel::Bachelors
      )
    )
  }

  #[test]
  fn test_new_course() {
    /* this test tests if the create_unknown_word function  hides the word  if the player has not yet  revealed it */
    let accountid = AccountId::new_unchecked("onchez.test".to_string());
    let context = get_context(accountid);
    testing_env!(context);
    let course = Course {
      course_name: "bit".to_string(),
      units: 20,
      students: vec![],
    };

    assert_eq!(course, Course::new_course("bit".to_string(), 20))
  }
  #[test]
  fn test_adding_course_to_university() {
    /* this test tests if the create_unknown_word function  hides the word  if the player has not yet  revealed it */
    let accountid = AccountId::new_unchecked("onchez.test".to_string());
    let context = get_context(accountid);
    testing_env!(context);
    let course = Course {
      course_name: "bit".to_string(),
      units: 20,
      students: vec![],
    };

    assert_eq!(course, Course::new_course("bit".to_string(), 20))
  }
  #[test]
  fn test_adding_new_users_to_course() {
    /* this test tests if the create_unknown_word function  hides the word  if the player has not yet  revealed it */
    let accountid = AccountId::new_unchecked("onchez.test".to_string());
    let context = get_context(accountid);
    testing_env!(context);
    let mut course = Course {
      course_name: "bit".to_string(),
      units: 20,
      students: vec![],
    };

    assert_eq!(course, Course::new_course("bit".to_string(), 20));
    let student1 =
      Student::new_student("onchez".to_string(), Gender::Male, 23, EducationLevel::Phd);

    course.add_student_to_course(student1);
    let student2 = Student::new_student(
      "james".to_string(),
      Gender::Undefined,
      29,
      EducationLevel::Masters,
    );

    course.add_student_to_course(student2);
    let student3 = Student::new_student(
      "dayo".to_string(),
      Gender::Undefined,
      18,
      EducationLevel::Bachelors,
    );

    course.add_student_to_course(student3);
    assert_eq!(course.students.len(), 3);

    assert_eq!(
      course.students.len(),
      course.view_students_in_course().len()
    )
  }

  #[test]
  fn test_department_impl() {
    /* this test tests if the create_unknown_word function  hides the word  if the player has not yet  revealed it */
    let accountid = AccountId::new_unchecked("onchez.test".to_string());
    let context = get_context(accountid);
    testing_env!(context);
    //creating a  new course instance
    let course = Course {
      course_name: "Computer Science".to_string(),
      units: 20,
      students: vec![],
    };
    let course_name = course.course_name.clone();
    //creating department instance
    let mut department = Department {
      department_name: "Sciences".to_string(),
      courses: HashMap::new(),
    };
    assert_eq!(
      department,
      Department::new_department("Sciences".to_string())
    );
    match department.add_course(course) {
      Ok(_) => println!("deparment was added succefully"),
      Err(_) => println!("Faild to add course"),
    }
    //adding new  student to a course in the deppartment
    match department.add_new_student_to_dep_course(
      &course_name,
      "Onchez".to_string(),
      Gender::Male,
      22,
      EducationLevel::Masters,
    ) {
      Ok(_) => {
        //
        assert_eq!(
          department
            .view_students_in_dep_course(&course_name)
            .unwrap()
            .0[0]
            .name,
          "Onchez".to_string()
        );
        println!("student added to deparment  succefully")
      }
      Err(_) => {
        assert_ne!(
          department
            .view_students_in_dep_course(&course_name)
            .unwrap()
            .0[0]
            .name,
          "Onchez".to_string()
        );
        println!("Faild to add student ")
      }
    }

    //getting the number of students in the Sciences department for the it course
    let students_in_course_dep = department
      .view_students_in_dep_course(&course_name)
      .unwrap();
    assert_eq!(students_in_course_dep.0.len(), 1);
    //adding new course to  the department

    assert_eq!(department.view_courses().len(), 1)
  }
}
