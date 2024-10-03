use russcip::{prelude::Model, ObjSense};
use std::collections::HashMap; // いわゆる辞書

#[derive(Debug)]
pub struct Employee {
    code: i32,
    name: String,
}

impl Employee {
    fn new(code: i32, name: &str) -> Self {
        Employee {
            code: code,
            name: String::from(name),
        }
    }
}

pub struct Job {
    code: i32,
    name: String,
}

impl Job {
    fn new(code: i32, name: &str) -> Self {
        Job {
            code: code,
            name: String::from(name),
        }
    }
}

fn main() {
    // 労働者集合を作成する
    let num_employee: usize = 3;
    let employee_codes: Vec<i32> = vec![1, 2, 3];
    let employee_names: Vec<&str> = vec!["A", "B", "C"];
    let mut employees: HashMap<usize, Employee> = HashMap::new();
    for k in 0..num_employee {
        let c: &i32 = employee_codes.get(k).unwrap();
        let n: &str = employee_names.get(k).unwrap();
        let e: Employee = Employee::new(*c, n);
        employees.insert(k, e);
    }

    // 仕事集合を作成する
    let num_jobs: usize = 2;
    let job_codes: Vec<i32> = vec![1, 2];
    let job_names: Vec<&str> = vec!["内勤(東京)", "出張(大阪)"];
    let mut jobs: HashMap<usize, Job> = HashMap::new();
    for k in 0..num_jobs {
        let c: &i32 = job_codes.get(k).unwrap();
        let n: &str = job_names.get(k).unwrap();
        let j: Job = Job::new(*c, n);
        jobs.insert(k, j);
    }

    // 定式化する問題を定義する
    let mut model = Model::new()
        .include_default_plugins()
        .create_prob("allocation")
        .set_obj_sense(ObjSense::Minimize);

    let mut variables: HashMap<i32, i32> = HashMap::new(); // 変数の集合を定義する
}
