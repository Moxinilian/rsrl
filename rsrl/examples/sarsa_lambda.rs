extern crate rsrl;
#[macro_use]
extern crate slog;

use rsrl::{
    control::td::SARSALambda,
    domains::{Domain, MountainCar, State},
    fa::{
        linear::{
            basis::{Fourier, Projector},
            optim::SGD,
            LFA,
        },
        DifferentiableStateActionFunction,
        Parameterised,
    },
    logging,
    make_shared,
    policies::{EpsilonGreedy, Greedy, Random},
    run,
    spaces::Space,
    traces,
    Evaluation,
    SerialExperiment,
};

fn main() {
    let domain = MountainCar::default();
    let mut agent = {
        let n_actions = domain.action_space().card().into();

        let bases = Fourier::from_space(3, domain.state_space()).with_constant();
        let q_func = make_shared(LFA::vector(bases, SGD(1.0), n_actions));
        let trace = traces::Replacing::zeros(q_func.weights_dim());

        let policy = EpsilonGreedy::new(Greedy::new(q_func.clone()), Random::new(n_actions), 0.2);

        SARSALambda::new(q_func, policy, trace, 0.001, 0.99, 0.7)
    };

    let logger = logging::root(logging::stdout());
    let domain_builder = Box::new(MountainCar::default);

    // Training phase:
    let _training_result = {
        // Start a serial learning experiment up to 1000 steps per episode.
        let e = SerialExperiment::new(&mut agent, domain_builder.clone(), 1000);

        // Realise 1000 episodes of the experiment generator.
        run(e, 1000, Some(logger.clone()))
    };

    // Testing phase:
    let testing_result = Evaluation::new(&mut agent, domain_builder).next().unwrap();

    info!(logger, "solution"; testing_result);
}
