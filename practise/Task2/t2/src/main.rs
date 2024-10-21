
use std::collections::{hash_set, HashMap, HashSet};
#[derive(Debug)]
enum Gender {
    Male,
    Fmale,
}
#[derive(Debug)]
struct Student {
    id : u32,
    name : String,
    age  : u32,
    gender : Gender,
}

#[derive(Debug)]
struct ClassInfo {
    name :String,
    students: HashSet<u32>,
}
#[derive(Debug)]
struct Club {
    name : String,
    describe : String,
    students: HashSet<u32>,
}
#[derive(Debug)]
struct Course {
    name:String,
    teacher:String,
    enrolled_students: HashSet<u32>,
}

struct ScoolSystem {
    students: HashMap<u32,Student>,
    classes:HashMap<String,ClassInfo>,
    clubs:HashMap<String,Club>,
    course:HashMap<String,Course>,
}

impl ScoolSystem{
    fn new()-> ScoolSystem{
        ScoolSystem{
            students:HashMap::new(),
            classes:HashMap::new(),
            clubs: HashMap::new(),
            course: HashMap::new(),
        }
    }

    fn add_student(&mut self,s :Student) {
        self.students.insert(s.id, s);
    }

    fn get_student(&self,id:u32)->Option<&Student>{
        self.students.get(&id)
    }  

    fn update_student(&mut self,id:u32,new_name:Option<String>,new_age:Option<u32>,new_gender:Option<Gender>,new_class:Option<ClassInfo>) {
       if let Some(stu) = self.students.get_mut(&id){
            if let Some(name) = new_name{
                stu.name = name;
            }
            if let Some(age) = new_age {
                stu.age = age;
            }
            if let Some(Gender) = new_gender {
                stu.gender = Gender;
            }
       }
    }

    fn del_student(&mut self,id : u32) {
        self.students.remove(&id);
    }

    fn add_class(&mut self,class :ClassInfo) {
        self.classes.insert(class.name.clone(), class);
    }
    
    fn add_student_to_class(&mut self,id:u32,class_name:&str) {
        if let Some(ClassInfo) = self.classes.get_mut(class_name){
            ClassInfo.students.insert(id);
        }
    }

    fn add_course(&mut self,course:Course) {
        self.course.insert(course.name.clone(), course);
    }

    fn add_student_to_course(&mut self,id :u32, course_name:&str) {
        if let Some(Course) = self.course.get_mut(course_name){
            Course.enrolled_students.insert(id);
        }
    }

    fn add_club(&mut self,club: Club) {
        self.clubs.insert(club.name.clone(), club);
    }

    fn add_student_to_club(&mut self,id:u32,club_name:&str) {
        if let Some(Club) = self.clubs.get_mut(club_name){
            Club.students.insert(id);
        }
    }

    fn print_student(&self) {
        for student in self.students.values() {
            println!("{:?}",student);
        }
    }

}


fn main() {
    let mut systom = ScoolSystem::new();
    let s1 = Student{
        id:1,
        name:String::from("al"),
        age:20,
        gender:Gender::Male,
    };
    systom.add_student(s1);

    let class1 = ClassInfo{
        name:String::from("Match"),
        students:HashSet::new(),
    };
    systom.add_class(class1);

}