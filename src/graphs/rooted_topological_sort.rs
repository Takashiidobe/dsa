use std::collections::{HashMap, VecDeque};

//@ This function prints out a topological sort of a company.
//@ This function works for any DAG where there is one root node which can have many children.
pub fn rooted_topological_sort(chart: &[(u32, u32, String)]) -> Option<Vec<(u64, String)>> {
    let mut emp_to_mgr: HashMap<u32, u32> = HashMap::new();
    let mut mgr_to_emp: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut emp_to_name: HashMap<u32, String> = HashMap::new();

    //@ We start out without having found the CEO.
    let mut ceo = None;

    //@ For every employee in the chart
    for (emp_id, mgr_id, name) in chart {
        //@ If they manage themselves, they're the CEO.
        if emp_id == mgr_id {
            ceo = Some(emp_id);
        } else {
            //@ Otherwise, we add the employee as a subordinate to their manager
            mgr_to_emp.entry(*mgr_id).or_default().push(*emp_id);
            //@ And add their manager to the employee.
            emp_to_mgr.insert(*emp_id, *mgr_id);
        }
        //@ We then maintain a mapping of employee to name for faster lookup.
        emp_to_name.insert(*emp_id, name.to_string());
    }

    //@ If we can't find a CEO, then there's no way to continue
    ceo?;

    //@ Next, we bfs through the org chart, starting with the CEO
    let mut q = VecDeque::new();
    q.push_back((ceo.unwrap(), 0));

    let mut res = vec![];

    //@ While we have employees to process
    while let Some((emp_id, depth)) = q.pop_front() {
        //@ we push the employee and its depth to the result
        res.push((depth, emp_to_name[emp_id].clone()));

        //@ then, for this employee's subordinates (if there are any)
        if let Some(reports) = mgr_to_emp.get(emp_id) {
            //@ We add them to the front of the queue, since we want to process them before other
            //@ peer employees.
            for report_id in reports {
                q.push_front((report_id, depth + 1));
            }
        }
    }

    //@ And then we return the collection.
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_yaml_snapshot;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn verify(chart: Vec<(u32, u32, String)>) -> bool {
        if chart.len() > 100 {
            return true;
        }
        rooted_topological_sort(&chart);
        true
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
