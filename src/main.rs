// Desc: Main entry point for the program
use rand::Rng;

const PARTICLES: usize = 15;    // number of particles
const ITERATIONS: usize = 1000; // number of iterations
const C1: f64 = 1.0;            // acceleration constant c1 (cognitive component) 
const C2: f64 = 1.0;            // acceleration constant c2 (social component)
const W: f64 = 0.9;             // inertia weight
const X_MIN: f64 = -5.0;        // minimum value of x
const X_MAX: f64 = 5.0;         // maximum value of x
const Y_MIN: f64 = -5.0;        // minimum value of y
const Y_MAX: f64 = 5.0;         // maximum value of y
const LOWER_BOUND: f64 = -5.0;  // lower bound of the search space
const UPPER_BOUND: f64 = 5.0;   // upper bound of the search space

struct Particle {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    pbest_x: f64,
    pbest_y: f64,
    pbest: f64,
}

struct Swarm {
    particles: Vec<Particle>,
    gbest_x: f64,
    gbest_y: f64,
    gbest: f64,
}

fn run_pso(swarm: &mut Swarm) {
    let mut rng = rand::thread_rng();

    for it in 0..ITERATIONS {
        for i in 0..PARTICLES {
            let r1: f64 = rng.gen();
            let r2: f64 = rng.gen();

            let p = &mut swarm.particles[i];

            // update velocity
            p.vx = W * p.vx + C1 * r1 * (p.pbest_x - p.x) + C2 * r2 * (swarm.gbest_x - p.x);
            p.vy = W * p.vy + C1 * r1 * (p.pbest_y - p.y) + C2 * r2 * (swarm.gbest_y - p.y);

            // update position
            p.x += p.vx;
            p.y += p.vy;

            // check if new position is better than previous position
            let fitness = rosenbrock(p.x, p.y);
            if fitness < p.pbest {
                p.pbest_x = p.x;
                p.pbest_y = p.y;
                p.pbest = fitness;
            }

            // check if new position is better than global best
            if fitness < swarm.gbest {
                swarm.gbest_x = p.x;
                swarm.gbest_y = p.y;
                swarm.gbest = fitness;
            } 
        }
        
        if it % 100 == 0 {
            println!("Iteration: {}, gbest: {}", it, swarm.gbest);
        }

        save_fitness_to_csv(it, swarm.gbest);
    }

    println!("Best solution found at: x = {}, y = {}, fitness = {}", swarm.gbest_x, swarm.gbest_y, swarm.gbest);
}

fn rosenbrock(x: f64, y: f64) -> f64 {
    (1.0 - x).powi(2) + 100.0 * (y - x.powi(2)).powi(2)
}

fn main() {
    println!("Rosenbrock using Particle Swarm Optimization");
    println!("===========================================\n");

    print_params();

    let mut swarm = Swarm {
        particles: Vec::new(),
        gbest_x: 0.0,
        gbest_y: 0.0,
        gbest: 0.0,
    };

    init_swarm(&mut swarm);  
    run_pso(&mut swarm);    
}

fn init_swarm(s: &mut Swarm) {
    let mut rng = rand::thread_rng();    

    // Initialize particles
    for _ in 0..PARTICLES {
        let (x, y) = (rng.gen_range(X_MIN..X_MAX), rng.gen_range(Y_MIN..Y_MAX));
        let p = Particle {
            x,
            y,
            vx: rng.gen_range(LOWER_BOUND..UPPER_BOUND),
            vy: rng.gen_range(LOWER_BOUND..UPPER_BOUND),
            pbest_x: x,
            pbest_y: y,
            pbest: rosenbrock(x, y),
        };  

        s.particles.push(p);
    }

    // Set the global best to the first particle
    s.gbest_x = s.particles[0].x;
    s.gbest_y = s.particles[0].y;
    s.gbest = s.particles[0].pbest;
}

fn print_params() {
    println!("Parameters:");
    println!("  Number of particles: {}", PARTICLES);
    println!("  Number of iterations: {}", ITERATIONS);
    println!("  Inertia weight: {}", W);
    println!("  Cognitive weight: {}", C1);
    println!("  Social weight: {}", C2);
    println!("  X range: [{}, {}]", X_MIN, X_MAX);
    println!("  Y range: [{}, {}]", Y_MIN, Y_MAX);
    println!("  Lower and Upper bounds: [{}, {}]", LOWER_BOUND, UPPER_BOUND);
    println!("");
}

fn save_fitness_to_csv(iteration: usize, fitness: f64) {
    let filename = format!("test3.csv");
    
    let file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&filename)
        .expect("Error opening CSV file");
    
    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);

    writer.serialize((iteration, fitness)).expect("Error writing to CSV file");
}

