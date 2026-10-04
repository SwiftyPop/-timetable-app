// Timetable Data Models and Schedule

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Subject {
    pub key: &'static str,
    pub name: &'static str,
    pub code: &'static str,
    pub hue: f32, // HSL hue (0..360)
}

pub static SUBJECTS: &[Subject] = &[
    Subject {
        key: "ctrl",
        name: "Control System Technology",
        code: "EMK22003 / EMK31103",
        hue: 34.0,
    },
    Subject {
        key: "ect",
        name: "Electric Circuit Theory 2",
        code: "EMK21203",
        hue: 268.0,
    },
    Subject {
        key: "el2",
        name: "Electronics 2",
        code: "EMK21303",
        hue: 196.0,
    },
    Subject {
        key: "mi",
        name: "Measurement & Instrumentation",
        code: "EMK21103",
        hue: 152.0,
    },
    Subject {
        key: "mt",
        name: "Mathematics for Eng. Technology 3",
        code: "IMQ21303",
        hue: 345.0,
    },
];

pub fn get_subject(key: &str) -> &'static Subject {
    SUBJECTS.iter().find(|s| s.key == key).unwrap_or(&SUBJECTS[0])
}

#[derive(Clone, Copy, Debug)]
pub struct ClassEvent {
    pub day: u8, // 1 = Mon, 2 = Tue, 3 = Wed, 4 = Thu, 5 = Fri
    pub start: u8,
    pub end: u8,
    pub subject_key: &'static str,
    pub class_type: &'static str,
    pub location: &'static str,
    pub lecturers: &'static str,
}

impl ClassEvent {
    pub fn duration(&self) -> u8 {
        self.end - self.start
    }

    pub fn subject(&self) -> &'static Subject {
        get_subject(self.subject_key)
    }

    pub fn room_short(&self) -> &'static str {
        match self.location {
            l if l.contains("Industrial Control Lab") => "IC Lab",
            l if l.contains("Electrical Machines Simulator Lab") => "EM Sim Lab",
            l if l.contains("Power Electronics Lab") => "Power Elec Lab",
            l if l.contains("Electrical Technology Lab") => "Elec Tech Lab",
            l => {
                if let Some(r) = l.split(" · ").last() {
                    r
                } else {
                    l
                }
            }
        }
    }
}

pub static EVENTS: &[ClassEvent] = &[
    ClassEvent {
        day: 1,
        start: 8,
        end: 11,
        subject_key: "ctrl",
        class_type: "Lab",
        location: "FKTE 2 · Industrial Control Lab",
        lecturers: "Mazwin Binti Mazlan, Mardianaliza Binti Othman",
    },
    ClassEvent {
        day: 2,
        start: 14,
        end: 17,
        subject_key: "ect",
        class_type: "Lab",
        location: "FKTE 2 · Electrical Machines Simulator Lab",
        lecturers: "Mohamad Zhafran Bin Zakariya, Junaidah Binti Ali Mohd Jobran",
    },
    ClassEvent {
        day: 2,
        start: 17,
        end: 19,
        subject_key: "ect",
        class_type: "Lecture C",
        location: "FKTE 2 · BK 1",
        lecturers: "Mohamad Zhafran Bin Zakariya",
    },
    ClassEvent {
        day: 3,
        start: 8,
        end: 11,
        subject_key: "el2",
        class_type: "Lab",
        location: "FKTE 2 · Power Electronics Lab",
        lecturers: "Chanuri A/P Charin, Abdul Rahim Bin Abdul Razak",
    },
    ClassEvent {
        day: 3,
        start: 15,
        end: 17,
        subject_key: "mi",
        class_type: "Lecture A",
        location: "Pauh Putra · DK 3",
        lecturers: "Muhammad Shakir Bin Laili",
    },
    ClassEvent {
        day: 3,
        start: 18,
        end: 19,
        subject_key: "mt",
        class_type: "Lecture · Online",
        location: "Online",
        lecturers: "Zabidi Bin Abu Hasan",
    },
    ClassEvent {
        day: 4,
        start: 12,
        end: 14,
        subject_key: "mt",
        class_type: "Lecture",
        location: "Pauh Putra · DK 8",
        lecturers: "Zabidi Bin Abu Hasan",
    },
    ClassEvent {
        day: 4,
        start: 14,
        end: 17,
        subject_key: "mi",
        class_type: "Lab",
        location: "FKTE 2 · Electrical Technology Lab",
        lecturers: "Muhammad Shakir Bin Laili, Chanuri A/P Charin",
    },
    ClassEvent {
        day: 5,
        start: 8,
        end: 10,
        subject_key: "ctrl",
        class_type: "Lecture A",
        location: "Pauh Putra · DK 1",
        lecturers: "Mazwin Binti Mazlan",
    },
    ClassEvent {
        day: 5,
        start: 15,
        end: 17,
        subject_key: "el2",
        class_type: "Lecture A",
        location: "Pauh Putra · DK 3",
        lecturers: "Chanuri A/P Charin, Abdul Rahim Bin Abdul Razak",
    },
];

pub static DAYS_SHORT: &[&str] = &["Mon", "Tue", "Wed", "Thu", "Fri"];
pub static DAYS_FULL: &[&str] = &["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];

pub fn total_class_hours() -> u32 {
    EVENTS.iter().map(|e| (e.end - e.start) as u32).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_hours() {
        assert_eq!(total_class_hours(), 23);
    }

    #[test]
    fn test_subject_lookup() {
        assert_eq!(SUBJECTS.len(), 5);
        let ctrl = get_subject("ctrl");
        assert_eq!(ctrl.name, "Control System Technology");
        assert_eq!(ctrl.hue, 34.0);
    }

    #[test]
    fn test_events_integrity() {
        assert_eq!(EVENTS.len(), 10);
        for ev in EVENTS {
            assert!(ev.day >= 1 && ev.day <= 5);
            assert!(ev.start >= 8 && ev.end <= 19);
            assert!(ev.end > ev.start);
            let s = ev.subject();
            assert_eq!(s.key, ev.subject_key);
        }
    }

    #[test]
    fn test_room_shortening() {
        let ev = &EVENTS[0];
        assert_eq!(ev.room_short(), "IC Lab");
    }
}
