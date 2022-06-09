use personnel::AstronautJob;
use personnel::Candidate;

fn main() {
    fn test_find_job_score() {
        let made_up_candidate = Candidate {
            primary_job: AstronautJob::Scientist,
            secondary_job: Some(AstronautJob::Biogeochemist),
            age: 20,
            health: 10,
        };
        println!(
            "Primary job score for this candidate should be {}: {}",
            283,
            find_job_score(made_up_candidate.primary_job)
        );
    }

    fn test_calculate_job_score_with_secondary() {
        let made_up_candidate = Candidate {
            primary_job: AstronautJob::Scientist,
            secondary_job: Some(AstronautJob::Biogeochemist),
            age: 20,
            health: 10,
        };
        println!(
            "Job score for this candidate should be {}: {}",
            185,
            calculate_job_score(&made_up_candidate)
        );
    }

    fn test_calculate_job_score_without_secondary() {
        let made_up_candidate = Candidate {
            primary_job: AstronautJob::Mechanic,
            secondary_job: None,
            age: 30,
            health: 10,
        };
        println!(
            "Job score for this candidate should be {}: {}",
            289,
            calculate_job_score(&made_up_candidate)
        );
    }

    fn test_calculate_candidate_score() {
        let made_up_candidate = Candidate {
            primary_job: AstronautJob::Mechanic,
            secondary_job: Some(AstronautJob::RoverOp),
            age: 20,
            health: 40,
        };
        println!(
            "Candidate score for this candidate should be {}: {}",
            3180,
            calculate_candidate_score(&made_up_candidate)
        );
    }

    fn test_calculate_sorting_candidates() {
        let modified_candidate_list: Vec<Candidate> =
            rank_candidates(Candidate::load_candidate_file());

        let mut counter: u32 = 0;
        let mut fast_candidate_score: u32 = 0;

        for temp_candidate in modified_candidate_list.iter() {
            if counter == 0 {
                fast_candidate_score = calculate_candidate_score(temp_candidate);
                counter += 1;
            }
            if fast_candidate_score < calculate_candidate_score(temp_candidate) {
                panic!("Not sorted correctly");
            } else {
                fast_candidate_score = calculate_candidate_score(temp_candidate);
            }
        }

        println!("Candidate list is sorted correctly, largest candidate score being at the top and lowest being at the bottom of the vector!");
    }

    test_find_job_score();
    test_calculate_job_score_with_secondary();
    test_calculate_job_score_without_secondary();
    test_calculate_candidate_score();
    test_calculate_sorting_candidates();
}

fn find_job_score(temp_job: AstronautJob) -> u32 {
    match temp_job {
        AstronautJob::Biogeochemist => 251,
        AstronautJob::Biologist => 257,
        AstronautJob::Engineer => 263,
        AstronautJob::Geologist => 269,
        AstronautJob::Mechanic => 271,
        AstronautJob::Medic => 277,
        AstronautJob::RoverOp => 281,
        AstronautJob::Scientist => 283,
    }
}

fn calculate_job_score(temp_candidate: &Candidate) -> u32 {
    (find_job_score((temp_candidate).primary_job)
        * match temp_candidate.secondary_job {
            Some(x) => find_job_score(x),
            None => find_job_score((temp_candidate).primary_job),
        })
        % 576
}

fn calculate_candidate_score(temp_candidate: &Candidate) -> u32 {
    ((calculate_job_score(temp_candidate) + temp_candidate.health as u32)
        * temp_candidate.age as u32)
        % 3928
}

fn rank_candidates(mut vec_candidate: Vec<Candidate>) -> Vec<Candidate> {
    vec_candidate.sort_by(|a, b| calculate_candidate_score(b).cmp(&calculate_candidate_score(a)));

    vec_candidate
}
