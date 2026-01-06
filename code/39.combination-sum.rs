impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();

        const MAX_LEN: usize = 40;
        type Solution = [usize; MAX_LEN];

        struct State {
            curr_solution: Solution,
            curr_idx: usize,
            solution_sum: i32,
        }

        let mut solutions = Vec::new();


        fn dfs(state: State, candidates: &[i32], solutions: &mut Vec<Vec<i32>>, target: i32) {
            
            if state.solution_sum == target {
                let sol = Vec::from_iter(
                        state.curr_solution
                        .into_iter()
                        .enumerate()
                        .filter(|(_, t)| *t > 0)
                        .flat_map(|(i, times)| std::iter::repeat_n(candidates[i], times) ));
                solutions.push(sol);
                return;
            }

            let cuid = state.curr_idx;
            if cuid >= candidates.len() ||
                candidates[cuid] > (target - state.solution_sum) ||
                state.solution_sum > target
            {
                return;
            }

            for i in state.curr_idx..candidates.len() {
                let mut new_sol = state.curr_solution;
                new_sol[i] += 1;
                let state = State {
                    curr_solution: new_sol,
                    curr_idx: i,
                    solution_sum: state.solution_sum + candidates[i]
                };
                dfs(state, candidates, solutions, target)
            }

        }

        let initial_state = State {
            curr_solution: [0; MAX_LEN],
            curr_idx: 0,
            solution_sum: 0
        };

        dfs(initial_state, &candidates, &mut solutions, target);

        solutions

    }
}
