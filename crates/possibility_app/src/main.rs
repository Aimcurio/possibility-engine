use clap::Parser;

mod app;
mod headless;
mod player_input;
mod scene;
mod ui;

#[derive(Parser, Debug)]
#[command(
    name = "possibility_app",
    about = "The Possibility Engine: 3D Interactive Sandbox Vertical Slice"
)]
struct CliArgs {
    /// Run verification in headless mode without initializing a graphical display window
    #[arg(long)]
    headless: bool,

    /// Run specific verification scenarios: 'a', 'b', 'c', or 'all'
    #[arg(long, default_value = "all")]
    verify: String,
}

fn main() {
    let args = CliArgs::parse();

    if args.headless || !args.verify.is_empty() && args.headless {
        println!("===============================================================");
        println!("  THE POSSIBILITY ENGINE — INDEPENDENT VERIFICATION RUNNER");
        println!("===============================================================");

        let mut results = Vec::new();
        let target = args.verify.to_lowercase();

        if target == "a" || target == "all" {
            println!("\n[RUNNING SCENARIO A: BOWL OF CEREAL]...");
            let res = headless::run_scenario_a_headless();
            println!(
                "  Result: {} | Steps: {} | Sim Time: {:.2}s",
                res.status, res.steps_taken, res.duration_sim_seconds
            );
            println!("  Details: {}", res.details);
            results.push(res);
        }

        if target == "b" || target == "all" {
            println!("\n[RUNNING SCENARIO B: ARTIFICIAL GRAVITY EXPERIMENT]...");
            let res = headless::run_scenario_b_headless();
            println!(
                "  Result: {} | Steps: {} | Sim Time: {:.2}s",
                res.status, res.steps_taken, res.duration_sim_seconds
            );
            println!("  Details: {}", res.details);
            results.push(res);
        }

        if target == "c" || target == "all" {
            println!("\n[RUNNING SCENARIO C: MULTI-AVATAR CONTROL HANDOFF]...");
            let res = headless::run_scenario_c_headless();
            println!(
                "  Result: {} | Steps: {} | Sim Time: {:.2}s",
                res.status, res.steps_taken, res.duration_sim_seconds
            );
            println!("  Details: {}", res.details);
            results.push(res);
        }

        println!("\n===============================================================");
        println!("  SUMMARY OF VERIFICATION GATES:");
        println!("===============================================================");
        let mut all_passed = true;
        for r in &results {
            println!("  Scenario {}: {} -> {}", r.scenario, r.title, r.status);
            if r.status != "PASSED" {
                all_passed = false;
            }
        }

        if all_passed {
            println!("\n>>> ALL VERIFICATION SCENARIOS PASSED WITH HIGH-FIDELITY EVIDENCE <<<");
            std::process::exit(0);
        } else {
            eprintln!("\n>>> ONE OR MORE VERIFICATION SCENARIOS FAILED <<<");
            std::process::exit(1);
        }
    }

    println!("Launching 3D graphical interactive viewport...");
    app::build_app().run();
}
