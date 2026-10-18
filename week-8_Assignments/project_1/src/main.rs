// I used AI to  understand what an APS checker does before attempting this project.
fn main() {

    // This is a menu that introduces a user to a list of what they are expected to know before inputing anything. 
    println!("==================================================================================================");
    println!(" WELCOME TO THE FEDERAL GOVERNMENT OF NIGERIA PUBLIC SERVICE APS LEVEL CHECKER (FOR WORKERS ONLY) ");
    println!("==================================================================================================");
    println!("This Branch checks and approves only the following professions: ");
    println!("1. Office Admin");
    println!("2. Academic");
    println!("3. Lawyer");
    println!("4. Teacher");
    println!("Please NOTE: THE INPUTS ARE CASE-SENSITIVE. Please enter professions spelling with caution");
    println!("===========================================================================================");
    println!("");


    let professions = vec!["Public Servant", "Office Admin", "Academic", "Lawyer", "Teacher"];
    let levels = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1", "EL2", "SES"];
    let office_admin = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    let academics = vec!["-", "Research Assistant", "Phd Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    let lawyer = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    let teacher = vec!["Placement", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"]; 

    //using user input to request the users name, profesion and role etc
    use std::io;

    let mut name = String::new();
    let mut profession = String::new();
    let mut years_of_experience = String::new();
    let mut role  = String::new();

    println!("");
    println!("Please enter your name: ");
    io::stdin().read_line(&mut name).expect("Failed to read name");
    println!("Hi {}",name.trim());
    let name_clean = name.trim();


    println!("What is your Profession? ");
    io::stdin().read_line(&mut profession).expect("Failed to read profession");
    let professions_clean = profession.trim();
    if professions.contains(&professions_clean) {
        println!("Profession Found In this branch. Proceed");
    }
    else {
        println!("Profession not found here. Please visit Our Alternative APS checker Branch At Ibeju Lekki Island");
        return; // This prevents continuation when a wrong proffession is entered
    }
    println!("{} is a/an {}", name.trim(), professions_clean);


    println!("Your years of Experience in {}", profession);
    io::stdin().read_line(&mut years_of_experience).expect("Failed to read input");
    let workers_experience:i32 = years_of_experience.trim().parse().expect("invalid input");

    println!("What is your specific Job title/ Role under Your Profession?:");
    io::stdin().read_line(&mut role).expect("Failed to read role");
    println!("Your role is {}",role);

    let role_clean = role.trim();

    //using or || to give alternative for different spelling 
    if professions_clean == "Office Admin" || professions_clean == "office admin" || professions_clean == "offices admin" {
        for z in 0..office_admin.len() { 
            //Rust you should look inside the vector using the loop index operator
            if office_admin[z] == role_clean {
                if z == 0 && workers_experience > 0 && workers_experience <= 2 {
                    println!("");
                    println!("{}, is an intern with {}, years of work experience. level: {}", name_clean, workers_experience, levels[z]);
                    println!("Your Specialty Matches this branch list. ");

                }
                else if z == 1 && workers_experience >=3 && workers_experience <=5 {
                    println!("{}, is an Administrator, with {} years of work experience. Level: {}", name_clean, workers_experience, levels[z]);
                    println!("Your Specialty Matches this branch list. ");

                }
                else if z == 2 && workers_experience >=5 && workers_experience <= 8 {
                    println!("{}, is a Senior Administrator with {} years of experience. Level: {}", name_clean, workers_experience, levels[z]);
                    println!("Your Specialty Matches this branch list. ");

                }
                else if z == 3 && workers_experience >=8 && workers_experience <= 10 {
                    println!("{}, is an Office Manager, with {} years of experience. Level: {}", name_clean, workers_experience, levels[z]);
                    println!("Your Specialty Matches this branch list. ");

                }
                else if z ==  4 && workers_experience >=10 && workers_experience <= 13 {
                    println!("{}, is a Director with {} years of experience. Level: {}", name_clean, workers_experience, levels [z]);
                    println!("Your Specialty Matches this branch list. ");

                }
                else if z == 5 && workers_experience >13 {
                    println!("{} is a CEO Hurrayyyyyyyyy, with {} years of experience. His Level is  {}", name_clean, workers_experience, levels[z]);
                    println!("Your Specialty Matches this branch list. ");

                } 
                else {
                    // This will catch a person who fakes his role and it doesn't match his work experience
                    println!("Validation Failed because your years of experience {} doesn't match the requirement for {} role", workers_experience, role_clean);
                }

            }
            else {
                println!("Failed to read role for office admin ");
            }
            println!("");
        }
    }
    else if professions_clean == "Academic" || professions_clean == "academics" || professions_clean == "academic" || professions_clean == "Academics" {
        for o in 0..academics.len() {
            //Confirm like you did before
            if academics[o] == role_clean {
                if o == 0 && workers_experience > 0 && workers_experience <= 2 {
                    println!("");
                    println!("{}, doesn't have a title with {}, years of work experience. level: {}", name_clean, workers_experience, levels[o]);

                }
                else if o == 1 && workers_experience >=3 && workers_experience <=5 {
                    println!("{}, is a Research Assistant, with {} years of work experience. Level: {}", name_clean, workers_experience, levels[o]);

                }
                else if o == 2 && workers_experience >=5 && workers_experience <= 8 {
                    println!("{}, is a Phd Candidate with {} years of experience. Level: {}", name_clean, workers_experience, levels[o]);

                }
                else if o == 3 && workers_experience >=8 && workers_experience <= 10 {
                    println!("{}, is a Post-Doc Researcher, with {} years of experience. Level: {}", name_clean, workers_experience, levels[o]);

                }
                else if o ==  4 && workers_experience >=10 && workers_experience <= 13 {
                    println!("{}, is a Senior Lecturer with {} years of experience. Level: {}", name_clean, workers_experience, levels [o]);

                }
                else if o == 5 && workers_experience >13 {
                    println!("{} is a Dean Hurrayyyyyyyyy, with {} years of experience. His Level is  {}", name_clean, workers_experience, levels[o]);

                } 
                else {
                    // This will catch a person who fakes his role and it doesn't match his work experience
                    println!("Validation Failed because your years of experience {} doesn't match the requirement for {} role", workers_experience, role_clean);
                }
            }
        }
    }
    // this handles different spellings
    else if professions_clean == "Lawyer" || professions_clean == "lawyer" || professions_clean == "lawyers" {
        for l in 0..lawyer.len() {
            if lawyer[l] == role_clean {
                if l == 0 && workers_experience > 0 && workers_experience <= 2 {
                    println!("");
                    println!("{}, is a Paralegal with {}, years of work experience. level: {}", name_clean, workers_experience, levels[l]);
                    println!("Your Role under this Profession is Found in this branch.");
                }
                else if l == 1 && workers_experience >=3 && workers_experience <=5 {
                    println!("{}, is a Junior Associate, with {} years of work experience. Level: {}", name_clean, workers_experience, levels[l]);
                    println!("Your Role under this Profession is Found in this branch. ");
                }
                else if l == 2 && workers_experience >=5 && workers_experience <= 8 {
                    println!("{}, is a Associate with {} years of experience. Level: {}", name_clean, workers_experience, levels[l]);
                    println!("Your Role under this Profession is Found in this branch. ");
                }
                else if l == 3 && workers_experience >=8 && workers_experience <= 10 {
                    println!("{}, is a Senior Assiociate 1-2, with {} years of experience. Level: {}", name_clean, workers_experience, levels[l]);
                    println!("Your Role under this Profession is Found in this branch. ");
                }
                else if l ==  4 && workers_experience >=10 && workers_experience <= 13 {
                    println!("{}, is a Senior Associate 3-4 with {} years of experience. Level: {}", name_clean, workers_experience, levels [l]);
                    println!("Your Role under this Profession is Found in this branch. ");
                }
                else if l == 5 && workers_experience >13 {
                    println!("{} is a Partner Hurrayyyyyyyyy, with {} years of experience. His Level is  {}", name_clean, workers_experience, levels[l]);
                    println!("Your Role under this Profession is Found in this branch. ");
                } 
                else {
                    // This will catch a person who fakes his role and it doesn't match his work experience
                    println!("Validation Failed because your years of experience {} doesn't match the requirement for {} role", workers_experience, role_clean);
                }
            }
            else {
                println!("Failed to read role for Lawyer");
            }
        }
    }
    else if professions_clean == "Teacher" || professions_clean == "teacher" || professions_clean == "teachers" {
        for t in 0..teacher.len() {
            if teacher[t] == role_clean {
                if t == 0 && workers_experience > 0 && workers_experience <= 2 {
                    println!("");
                    println!("{}, is a Placement with {}, years of work experience. level: {}", name_clean, workers_experience, levels[t]);
                }
                else if t == 1 && workers_experience >=3 && workers_experience <=5 {
                    println!("{}, is a Classroom Teacher, with {} years of work experience. Level: {}", name_clean, workers_experience, levels[t]);
                }
                else if t == 2 && workers_experience >=5 && workers_experience <= 8 {
                    println!("{}, is a Senior Teacher with {} years of experience. Level: {}", name_clean, workers_experience, levels[t]);
                }
                else if t == 3 && workers_experience >=8 && workers_experience <= 10 {
                    println!("{}, is a Leading Teacher, with {} years of experience. Level: {}", name_clean, workers_experience, levels[t]);
                }
                else if t ==  4 && workers_experience >=10 && workers_experience <= 13 {
                    println!("{}, is a Deputy Principal with {} years of experience. Level: {}", name_clean, workers_experience, levels [t]);
                }
                else if t == 5 && workers_experience >13 {
                    println!("{} is a Principal, with {} years of experience. His Level is  {}", name_clean, workers_experience, levels[t]);
                } 
                else {
                    // This will catch a person who fakes his role and it doesn't match his work experience
                    println!("Validation Failed because your years of experience {} doesn't match the requirement for {} role", workers_experience, role_clean);
                }
            }
            else {
                println!("Failed to read role for Teacher");
            }
        }   
    }
    else {
        println!("Role/ Specialty not found in this branch. Please visit our nearby branch At Ibeju Lekki Island");
    }

}