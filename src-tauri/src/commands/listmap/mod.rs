use crate::dto::Student;
use crate::dto::Teacher;
use chrono::{ Local, NaiveDateTime };
use std::{ collections::HashMap, sync::Mutex };
use tauri::State;

// 类型别名简化状态管理
pub type TeacherList = Vec<Teacher>;
// 类型别名简化状态管理
pub type StudentMap = HashMap<i32, Student>;

// 初始化学生列表
#[tauri::command]
pub fn init_students(state: State<Mutex<StudentMap>>) {
    let mut map = StudentMap::new();
    let mut state_lock = state.lock().unwrap();

    for i in 1..=4 {
        let student = Student {
            id: i,
            name: format!("Item{}", i),
            age: i,
            height: 170.0 + (i as f64) * 0.01,
            birthday: Local::now().naive_local(),
        };
        map.insert(i, student);
    }

    *state_lock = map;
}

// 添加学生
#[tauri::command]
pub fn add_student(state: State<Mutex<StudentMap>>, name: String, age: i32, height: f64, birthday: NaiveDateTime) -> Option<Student> {
    let mut state_lock = state.lock().unwrap();

    // 生成新ID
    let new_id = state_lock.keys().max().unwrap_or(&0) + 1;

    let student = Student {
        id: new_id,
        name,
        age,
        height,
        birthday,
    };

    state_lock.insert(new_id, student.clone());
    Some(student)
}

// 删除学生
#[tauri::command]
pub fn delete_student(state: State<Mutex<StudentMap>>, id: i32) -> bool {
    let mut state_lock = state.lock().unwrap();
    state_lock.remove(&id).is_some()
}

// 更新学生
#[tauri::command]
pub fn update_student(state: State<Mutex<StudentMap>>, id: i32, name: Option<String>, age: Option<i32>, height: Option<f64>, birthday: Option<NaiveDateTime>) -> Option<Student> {
    let mut state_lock = state.lock().unwrap();

    if let Some(student) = state_lock.get_mut(&id) {
        if let Some(name) = name {
            student.name = name;
        }
        if let Some(age) = age {
            student.age = age;
        }
        if let Some(height) = height {
            student.height = height;
        }
        if let Some(birthday) = birthday {
            student.birthday = birthday;
        }
        return Some(student.clone());
    }

    None
}

// 查询学生（支持多种条件）
#[tauri::command]
pub fn query_students(state: State<Mutex<StudentMap>>, name_filter: Option<String>, age_min: Option<i32>, age_max: Option<i32>, birthday_start: Option<NaiveDateTime>, birthday_end: Option<NaiveDateTime>) -> Vec<Student> {
    let state_lock = state.lock().unwrap();
    state_lock
        .values()
        .filter(|s| {
            // 姓名模糊查询
            if let Some(filter) = &name_filter {
                if !s.name.to_lowercase().contains(&filter.to_lowercase()) {
                    return false;
                }
            }

            // 年龄区间查询
            if let Some(min) = age_min {
                if s.age < min {
                    return false;
                }
            }
            if let Some(max) = age_max {
                if s.age > max {
                    return false;
                }
            }

            // 生日区间查询
            if let Some(start) = birthday_start {
                if s.birthday < start {
                    return false;
                }
            }
            if let Some(end) = birthday_end {
                if s.birthday > end {
                    return false;
                }
            }

            true
        })
        .cloned()
        .collect()
}

// 获取单个学生
#[tauri::command]
pub fn get_student(state: State<Mutex<StudentMap>>, id: i32) -> Option<Student> {
    let state_lock = state.lock().unwrap();
    state_lock.get(&id).cloned()
}

// 初始化教师列表
#[tauri::command]
pub fn init_teachers(state: State<Mutex<TeacherList>>) {
    let mut teachers = TeacherList::new();
    let mut state_lock = state.lock().unwrap();

    for i in 1..=4 {
        let teacher = Teacher {
            id: i,
            name: format!("Teacher{}", i),
            age: 30 + i, // 教师年龄通常较大
            height: 170.0 + (i as f64) * 0.01,
            birthday: Local::now().naive_local(),
        };
        teachers.push(teacher);
    }

    *state_lock = teachers;
}

// 添加教师
#[tauri::command]
pub fn add_teacher(state: State<Mutex<TeacherList>>, name: String, age: i32, height: f64, birthday: NaiveDateTime) -> Option<Teacher> {
    let mut state_lock = state.lock().unwrap();

    // 生成新ID (当前最大ID + 1)
    let new_id =
        state_lock
            .iter()
            .map(|t| t.id)
            .max()
            .unwrap_or(0) + 1;

    let teacher = Teacher {
        id: new_id,
        name,
        age,
        height,
        birthday,
    };

    state_lock.push(teacher.clone());
    Some(teacher)
}

// 删除教师
#[tauri::command]
pub fn delete_teacher(state: State<Mutex<TeacherList>>, id: i32) -> bool {
    let mut state_lock = state.lock().unwrap();

    if let Some(pos) = state_lock.iter().position(|t| t.id == id) {
        state_lock.remove(pos);
        true
    } else {
        false
    }
}

// 更新教师
#[tauri::command]
pub fn update_teacher(state: State<Mutex<TeacherList>>, id: i32, name: Option<String>, age: Option<i32>, height: Option<f64>, birthday: Option<NaiveDateTime>) -> Option<Teacher> {
    let mut state_lock = state.lock().unwrap();

    if let Some(teacher) = state_lock.iter_mut().find(|t| t.id == id) {
        if let Some(name) = name {
            teacher.name = name;
        }
        if let Some(age) = age {
            teacher.age = age;
        }
        if let Some(height) = height {
            teacher.height = height;
        }
        if let Some(birthday) = birthday {
            teacher.birthday = birthday;
        }
        return Some(teacher.clone());
    }

    None
}

// 查询教师（支持多种条件）
#[tauri::command]
pub fn query_teachers(state: State<Mutex<TeacherList>>, name_filter: Option<String>, age_min: Option<i32>, age_max: Option<i32>, birthday_start: Option<NaiveDateTime>, birthday_end: Option<NaiveDateTime>) -> Vec<Teacher> {
    let state_lock = state.lock().unwrap();
    state_lock
        .iter()
        .filter(|t| {
            // 姓名模糊查询
            if let Some(filter) = &name_filter {
                if !t.name.to_lowercase().contains(&filter.to_lowercase()) {
                    return false;
                }
            }

            // 年龄区间查询
            if let Some(min) = age_min {
                if t.age < min {
                    return false;
                }
            }
            if let Some(max) = age_max {
                if t.age > max {
                    return false;
                }
            }

            // 生日区间查询
            if let Some(start) = birthday_start {
                if t.birthday < start {
                    return false;
                }
            }
            if let Some(end) = birthday_end {
                if t.birthday > end {
                    return false;
                }
            }

            true
        })
        .cloned()
        .collect()
}

// 获取单个教师
#[tauri::command]
pub fn get_teacher(state: State<Mutex<TeacherList>>, id: i32) -> Option<Teacher> {
    let state_lock = state.lock().unwrap();
    state_lock
        .iter()
        .find(|t| t.id == id)
        .cloned()
}
