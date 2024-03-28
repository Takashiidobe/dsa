use std::collections::{HashMap, VecDeque};

pub fn rooted_topological_sort(chart: &[(u32, u32, String)]) -> Option<Vec<(u64, String)>> {
    let mut emp_to_mgr: HashMap<u32, u32> = HashMap::new();
    let mut mgr_to_emp: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut emp_to_name: HashMap<u32, String> = HashMap::new();

    let mut ceo = None;

    for (emp_id, mgr_id, name) in chart {
        if emp_id == mgr_id {
            ceo = Some(emp_id);
        } else {
            mgr_to_emp.entry(*mgr_id).or_default().push(*emp_id);
            emp_to_mgr.insert(*emp_id, *mgr_id);
        }
        emp_to_name.insert(*emp_id, name.to_string());
    }

    ceo?;

    let mut q = VecDeque::new();
    q.push_back((ceo.unwrap(), 0));

    let mut res = vec![];

    while let Some((emp_id, depth)) = q.pop_front() {
        res.push((depth, emp_to_name[emp_id].clone()));

        if let Some(reports) = mgr_to_emp.get(emp_id) {
            for report_id in reports {
                q.push_front((report_id, depth + 1));
            }
        }
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_yaml_snapshot;

    #[test_fuzz::test_fuzz]
    fn _rooted_topological_sort(chart: &[(u32, u32, String)]) -> Option<Vec<(u64, String)>> {
        rooted_topological_sort(chart)
    }

    #[test]
    fn example() {
        let chart = vec![
            (6, 5, "acolyte1".to_string()),
            (7, 4, "acolyte2".to_string()),
            (2, 1, "sylvanas".to_string()),
            (3, 1, "anubarak".to_string()),
            (4, 2, "commander1".to_string()),
            (1, 1, "arthas".to_string()),
            (5, 3, "commander2".to_string()),
        ];

        let result = rooted_topological_sort(&chart);
        assert_yaml_snapshot!(result);
    }
}
