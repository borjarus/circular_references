
struct Student {
    name: String
}

impl Student {
    fn new(name: &str) -> Student
    {
        Student {
            name: name.into()
        }
    }
}

struct Course {
    name: String
}


struct Enrollment<'a> 
{
    student: &'a Student,
    course: &'a Course
}

fn main() {

}
