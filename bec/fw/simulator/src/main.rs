use control_loop::AlphaBetaGamma;
use nalgebra::{ComplexField, Vector2};



fn main() {
    const LOOPS: usize = 8;

    let current = 1f32;

    let aby = AlphaBetaGamma::<f32>::precompute(&());

    let theta_electrical = 0f32;
    let electrical_state = state_vector(1f32, theta_electrical);

    for i in 0..LOOPS {
        let theta = 2f32 * (i as f32 / LOOPS as f32) * std::f32::consts::PI;
        let ab = state_vector(current / 2f32.sqrt(), theta_electrical);

        println!("Iteration {} - {:03.1} deg", i, theta.to_degrees());
        let alpha_beta = aby.apply(ab);
        println!("\tα: {:04.3}\tβ: {:04.3}", alpha_beta[0], alpha_beta[1]);
        let dq = control_loop::dqz(&(), alpha_beta, theta);
        println!("\tD: {:4.3}\tQ: {:4.3}", dq[0], dq[1]);
    }
}

fn state_vector(i: f32, theta: f32) -> Vector2<f32> {
    Vector2::new(2f32.sqrt() * i * theta.cos(), 2f32.sqrt() * i * (theta - 2f32 * std::f32::consts::FRAC_PI_3).cos())
}
