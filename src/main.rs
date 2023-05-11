enum Gender {
    Male,
    Female,
}

struct Member {
    name: String,
    gender: Gender,
    age: u8,
}

impl Member {
    fn new(name: String, gender: Gender, age: u8) -> Member {
        Member { name, gender, age }
    }
}

fn main() {
    let members: Vec<Member> = vec![
        Member::new(String::from("WY"), Gender::Male, 31),
        Member::new(String::from("TCX"), Gender::Female, 27),
    ];

    let scores = vec![10, 20];

    match gender_to_win(&members, &scores, 30) {
        Gender::Male => println!("Boys!"),
        Gender::Female => println!("Girls!"),
    }
}

fn gender_to_win(members: &[Member], scores: &[i8], age_threshold: u8) -> Gender {
    let mut offset = 0;

    for (i, member) in members.iter().enumerate() {
        println!("{}: {}", member.name, scores[i]);
        if member.age < age_threshold {
            println!("{} is too young to score", member.name);
            continue;
        }

        match member.gender {
            Gender::Male => offset += scores[i],
            Gender::Female => offset -= scores[i],
        }
    }

    if offset > 0 {
        Gender::Male
    } else {
        Gender::Female
    }
}
