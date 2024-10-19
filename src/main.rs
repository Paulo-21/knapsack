use std::{default, fmt::{self, Display}, fs, time::Instant, vec};
use rand::Rng;
use colored::Colorize;
use std::env;
#[derive(Clone, Copy)]
enum Tinstance {
    NC,
    FC,
    SS
}
struct Node {
    id : usize,
    used : bool,
    cost : u64,
    weight : u64,
    limit : f64,
}
impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "id : {}, used : {}", self.id, self.used)
    }
}
struct Sacados {
    instance : Vec<Objet>,
    poids_max : u64,
    sol : Vec<bool>
}
#[derive(PartialEq, PartialOrd,  Clone)]
struct Objet {
    poids : u64,
    valeur : u64,
    ratio : f64,
    id : u64,
}
impl Display for Sacados {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for a in 0..self.instance.len() {
            writeln!(f, "poids : {}, valeur : {}", self.instance[a].poids, self.instance[a].valeur)?;
        }
        writeln!(f, "Poids Maximal du sac à dos : {}", self.poids_max)?;
        Ok(())
    }
}
impl Sacados {
    fn sort_instances (&self) -> Vec<Objet> {
        let mut res=  self.instance.clone();
        res.sort_by(|x, y| y.ratio.partial_cmp(&x.ratio).unwrap());
        res
    }
    fn glouton(&self) -> Vec<Objet> {
        let mut instances = self.sort_instances();
        let mut poidsac: u64 = 0;
        //let mut res = Vec::with_capacity(instances.capacity());
        for (i, obj) in instances.iter().enumerate() {
            if poidsac <= self.poids_max {
                instances.truncate(i);
                break;
            }
            poidsac += obj.poids;
        }
        instances
    }
    fn sol_glouton(sorted : &[Objet], poids_max: u64) -> (Vec<bool>, u64) {
        let mut poidsac = 0;
        let mut valmax = 0;
        let mut used = vec![false;sorted.len()];
        for (u, obj) in sorted.iter().enumerate() {
            if poidsac+obj.poids <= poids_max {
                poidsac += obj.poids;
                valmax += obj.valeur;
                used[u] = true;
            }
        }
        (used, valmax)
    }
    fn sol_glouton_relax(sorted : &[Objet],mut valmax : u64, poidsac: u64, poids_max: u64, level : usize, ) -> f64 {
        let mut restant = poids_max - poidsac;
        for obj in sorted[level..].iter() {
            if obj.poids > restant {
                return valmax as f64 + (restant as f64 * obj.ratio);
            }
            restant -= obj.poids;
            valmax += obj.valeur;
        }
        valmax as f64
    }
    fn get_know_instance() -> Self {
        let obj0 = Objet {valeur : 3, poids : 2, ratio : 0., id:0};
        let obj1 = Objet {valeur : 4, poids : 3, ratio : 0., id:1};
        let obj2= Objet {valeur : 8, poids : 4, ratio : 0., id:2};
        let obj3 = Objet {valeur : 8, poids : 5, ratio : 0., id:3};
        let obj4 = Objet {valeur : 10, poids : 9, ratio : 0., id:4};
        let instance = Vec::from([obj0, obj1, obj2, obj3, obj4]);
        Self { instance , poids_max: 10, sol : Vec::new()}
    }
    fn gen_rand_instances(nb_inst : u64, r: u64, type_instances : Tinstance ) -> Self {
        let mut instances = Vec::with_capacity(nb_inst as usize);
        let mut rng = rand::thread_rng();
        let mut pmax: u64 = 0;
        for id in 0..nb_inst {
            let (u, p) = match type_instances {
                Tinstance::NC => {
                    let p: u64 = rng.gen_range(1..r);
                    let u: u64 = rng.gen_range(1..r);
                    (u, p)
                },
                Tinstance::FC => {
                    let p = rng.gen_range(1..r);
                    let u = rng.gen_range(p - r/10..p + r/10);
                    (u, p)
                },
                Tinstance::SS => {
                    let p = rng.gen_range(1..r);
                    (p,p)
                }
            };
            let obj = Objet {valeur : u, poids : p, ratio : u as f64 / p as f64, id};
            pmax += p;
            instances.push(obj);
        }
        Self { instance : instances, poids_max:  pmax/2, sol : Vec::new()}
    }
    fn arborescence(&mut self) {
        let mut pile = Vec::with_capacity(self.instance.len());
        let mut used = vec![false;self.instance.len()];
        let mut sol  = vec![false; self.instance.len()];
        let mut poidsac: u64 = 0;
        let mut cost: u64 = 0;
        let mut best_score = 0;
        let mut step: u64 = 0;
        pile.push(Node{id : 0, used : false, cost :0, weight:0, limit:0.});
        pile.push(Node{id : 0, used : true, cost:0, weight:0, limit:0.});
        while let Some(node) = pile.pop() {
            step +=1;
                
                if node.used {
                    if poidsac + self.instance[node.id].poids <= self.poids_max {
                        used[node.id] = true;
                        poidsac += self.instance[node.id].poids ;
                        cost += self.instance[node.id].valeur;
                        if cost > best_score {
                            sol.copy_from_slice(&used);
                            best_score = cost;
                            //println!("{} {step}", cost);
                        }
                    }
                    else {
                        used[node.id] = false;
                        continue;
                    }
                }
                else {
                    if used[node.id] {
                        used[node.id] = false;
                        poidsac -= self.instance[node.id].poids;
                        cost -= self.instance[node.id].valeur;
                    }
                }
            
                if node.id < self.instance.len()-1 {
                    pile.push(Node{id : node.id+1, used : false, cost:0, weight:0, limit:0.});
                    pile.push(Node{id : node.id+1, used : true,cost:0, weight:0, limit:0.});
                }
        }
        //println!("{:?}", sol);
        self.sol = sol;
        println!("step : {}", step);
    }
    fn branch_and_bound(&mut self) {
        let mut pile = Vec::with_capacity(self.instance.len());
        let mut used = vec![false;self.instance.len()];
        let mut step: u64 = 0;

        let sorted = Sacados::sort_instances(self);
        pile.push(Node{id : 0, used : false, cost:0,weight:0, limit:0.});
        pile.push(Node{id : 0, used : true,cost:0,weight:0, limit:0.});
        let (mut sol, mut best_score) = Sacados::sol_glouton(&sorted, self.poids_max);
    
        println!("{}", best_score.to_string().blue());
        while let Some(node) = pile.pop() {
            step +=1;
            let mut poidsac = node.weight;
            let mut cost = node.cost;
            if node.limit != 0. && node.limit < best_score as f64 {
                continue;
            }
            used[node.id..].fill(false);
            if node.used {
                if poidsac + sorted[node.id].poids <= self.poids_max {
                    used[node.id] = true;
                    poidsac += sorted[node.id].poids;
                    cost += sorted[node.id].valeur;
                    if cost > best_score {
                        best_score = cost;
                        sol.copy_from_slice(&used);
                    }
                }
                else {
                    used[node.id] = false;
                    continue;
                }
            }
            else {
                used[node.id] = false;
            }
            if node.id < sorted.len()-1 {
                let limite  = Sacados::sol_glouton_relax(&sorted,cost, poidsac, self.poids_max, node.id+2);
                let limite2 = Sacados::sol_glouton_relax(&sorted,cost,  poidsac, self.poids_max, node.id+1);
                //println!("lim : {} {}",limite,  limite2);
                if limite > best_score as f64 {
                    pile.push(Node{id : node.id+1, used : false, cost, weight:poidsac, limit:limite});
                }
                if limite2 > best_score as f64 {
                    pile.push(Node{id : node.id+1, used : true, cost, weight:poidsac, limit:limite2});
                }
            }
        }
        let mut finalsc = 0;
        let mut finalv = vec![false;self.instance.len()];
        for (u, o) in sorted.iter().enumerate() {
            finalv[o.id as usize] = sol[u];
            if sol[u] {
                finalsc += o.valeur;
            }
        }
        self.sol = finalv;
        println!("best score : {best_score}");
        println!("{}", finalsc.to_string().yellow());
        println!("step : {}", step);
    }
}
fn test() {
    let paramr = [100, 1000, 10000];
    
    let t = Tinstance::NC;
    let tmax = 32;
    let towrite_org = String::from(
        "#Colonne 1: Taille des instances\n#Colonne 2: Times to solve\n"
    );
    for r in paramr {
        let mut towrite = towrite_org.clone();
        for i in 1..=10 {
            let tf = (tmax as f64 * i as f64 / 10.) as u64;
            println!("Taille d'instance : {}", tf);
            let mut sac = Sacados::gen_rand_instances(tf, r, t);
            println!("GEN DONE");
            let start = Instant::now();
            //sac.glouton();
            sac.arborescence();
            let time = start.elapsed().as_nanos();
            println!("r : {r} / {} msec ", time);
            let time = time.ilog2();
            towrite.push_str(tf.to_string().as_str());
            towrite.push(' ');
            towrite.push_str(time.to_string().as_str());
            towrite.push('\n');
        }
        let mut file_name = String::from("data");
        file_name.push_str(r.to_string().as_str());
        file_name.push_str(".dat");
        fs::write(file_name, towrite).unwrap();
    }
}
fn test_one() {
    let paramr = [100, 1000, 10000];
    let tf = 100_000_000;
    println!("Taille instance  : {tf}");
    let t = Tinstance::NC;
    for r in paramr {
        let sac = Sacados::gen_rand_instances(tf, r, t);
        println!("GEN DONE");
        let start = Instant::now();
        sac.glouton();
        let time = start.elapsed().as_millis();
        println!("r : {r} / {} msec ", time);
    }
}
fn compute_cost(sol : &[Objet], used : &[bool]) -> u64 {
    let mut res = 0;
    for (u, o) in sol.iter().enumerate() {
        if used[u] {
            res += o.valeur;
        }
    }
    res
}
fn main() {
    println!("TME35");
    //test();
    //test_one();
    let init: u64 = 1500;
    let t = match env::args().count() == 2 {
        true => env::args().nth(1).unwrap().parse().unwrap(),
        false => init,
    };
    println!("{t} objects");
    //println!("Tot node : {}", 2u64.pow(t+1) -1);
    let mut sac = Sacados::gen_rand_instances(t, t/2, Tinstance::NC);
    //let mut sac = Sacados::get_know_instance();
    //println!("{}", sac);
    let start = Instant::now();
    /*sac.arborescence();
    let arbsol = sac.sol.clone();
    sac.branch_and_bound();
    println!("same : {}", arbsol == sac.sol);
    println!("arb {:?}", arbsol);
    println!("bb  {:?}", sac.sol);
    let cost_same = compute_cost(&sac.instance, &arbsol) == compute_cost(&sac.instance, &sac.sol);
    match cost_same {
        true => println!("{} {} {}", compute_cost(&sac.instance, &arbsol),"vs".green(), compute_cost(&sac.instance, &sac.sol)),
        false => println!("{} {} {}", compute_cost(&sac.instance, &arbsol), "vs".red(), compute_cost(&sac.instance, &sac.sol)),
    }*/
    sac.branch_and_bound();
    println!("{} msec", start.elapsed().as_millis());
}