use core::f64;
use russcip::ProblemOrSolving;
use russcip::{prelude::Model, ObjSense, VarType, Variable, WithSolutions};
use std::collections::HashMap;
use std::rc::Rc;

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

#[derive(Debug)]
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
    // 従業員集合定義
    let num_employee: usize = 3;
    let employee_codes: Vec<i32> = vec![11, 12, 13];
    let employee_names: Vec<&str> = vec!["A", "B", "C"];
    let mut employees: Vec<Employee> = Vec::with_capacity(num_employee);
    for k in 0..num_employee {
        let c: &i32 = employee_codes.get(k).unwrap();
        let n: &str = employee_names.get(k).unwrap();
        let e: Employee = Employee::new(*c, n);
        employees.push(e);
    }

    // 仕事集合定義
    let num_jobs: usize = 2;
    let job_codes: Vec<i32> = vec![201, 202];
    let job_names: Vec<&str> = vec!["内勤(東京)", "出張(大阪)"];
    let mut jobs: Vec<Job> = Vec::with_capacity(num_jobs);
    for k in 0..num_jobs {
        let c: &i32 = job_codes.get(k).unwrap();
        let n: &str = job_names.get(k).unwrap();
        let j: Job = Job::new(*c, n);
        jobs.push(j);
    }

    // コスト集合定義
    let mut costs: HashMap<(i32, i32), i32> = HashMap::new();
    costs.insert((11, 201), 7);
    costs.insert((11, 202), 2);
    costs.insert((12, 201), 10);
    costs.insert((12, 202), 13);
    costs.insert((13, 201), 3);
    costs.insert((13, 202), 6);

    // 問題定義
    let mut model = Model::new()
        .hide_output()
        .include_default_plugins()
        .create_prob("job-allocation-problem")
        .set_obj_sense(ObjSense::Minimize);

    // 変数定義
    let mut variables: HashMap<(i32, i32), Rc<Variable>> = HashMap::new(); // 変数の集合を定義する
    for e in &employees {
        for j in &jobs {
            let e_code = e.code;
            let j_code = j.code;
            let &c = costs.get(&(e_code, j_code)).unwrap();
            let x: Rc<Variable> = model.add_var(
                0.,
                1.,
                c as f64,
                format!("x({e_code}, {j_code})").as_str(),
                VarType::Binary,
            );
            variables.insert((e.code, j.code), x);
        }
    }

    // 制約条件(1): 任意の従業員について、割り当てられる仕事は1つまで
    for e in &employees {
        let vars: Vec<Rc<Variable>> = jobs
            .iter()
            .filter_map(|j| variables.get(&(e.code, j.code)))
            .cloned()
            .collect();
        let coefs: Vec<f64> = jobs.iter().map(|_| 1.).collect();
        model.add_cons(
            vars,
            &coefs,
            -f64::INFINITY,
            1.0,
            format!("constraint_1({})", e.code).as_str(),
        );
    }

    // 制約条件(2): 任意の仕事について、割り当たる人は1人まで
    for j in &jobs {
        let vars: Vec<Rc<Variable>> = employees
            .iter()
            .filter_map(|e| variables.get(&(e.code, j.code)))
            .cloned()
            .collect();
        let coefs: Vec<f64> = employees.iter().map(|_| 1.).collect();
        model.add_cons(
            vars,
            &coefs,
            1.0,
            f64::INFINITY,
            format!("constraint_2({})", j.code).as_str(),
        );
    }

    // 求解
    let solved_model = model.solve();

    let status = solved_model.status();
    println!("求解ステータス {:?}", status);
    println!("目的関数値: {}", solved_model.obj_val());

    let solution = solved_model.best_sol().unwrap();
    for e in &employees {
        for j in &jobs {
            let v = variables.get(&(e.code, j.code)).unwrap();
            let sol = solution.val(v.clone());
            if sol > 0. {
                println!("従業員{}が{}を担当します。", e.name, j.name);
            }
        }
    }
}
