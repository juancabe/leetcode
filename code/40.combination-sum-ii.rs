impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();

        struct State {
            curr_solution: [usize; 50],
            curr_cand_idx: usize,
            solution_sum: i32,
        }

        let mut frequencies = [0; 50];
        for c in candidates.iter() {
            frequencies[(*c) as usize] += 1;
        }

        candidates.dedup();
        let mut solutions = Vec::new();

        fn dfs(mut state: State, candidates: &[i32], solutions: &mut Vec<Vec<i32>>, target: i32, frequencies: &[usize; 50])
            -> State {

            let cuca = candidates[state.curr_cand_idx];
            if state.curr_solution[state.curr_cand_idx] > frequencies[cuca as usize] {
                return state;
            } else if state.solution_sum == target {
                solutions.push(
                        state.curr_solution
                            .iter()
                            .enumerate()
                            .filter(|(_, t)| **t > 0)
                            .flat_map(|(i, t)| std::iter::repeat_n(candidates[i], *t))
                            .collect()
                            );
                return state;
            }


            if state.curr_cand_idx >= candidates.len() ||
                cuca > (target - state.solution_sum) ||
                state.solution_sum > target
            {
                return state;
            }

            let cuid = state.curr_cand_idx;

            for i in state.curr_cand_idx..candidates.len() {
                let cand = candidates[i];
                state.curr_solution[i] += 1;
                state.solution_sum += cand;
                state.curr_cand_idx = i;
                state = dfs(state, candidates, solutions, target, frequencies);
                state.curr_solution[i] -= 1;
                state.solution_sum -= cand;
            }

            state.curr_cand_idx = cuid;
            state
        }

        let initial_state = State {
            curr_solution: [0; 50],
            curr_cand_idx: 0,
            solution_sum: 0
        };

        let _ = dfs(initial_state, &candidates, &mut solutions, target, &frequencies);

        solutions

    }
}
