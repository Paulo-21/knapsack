use std::{fmt::{self, Display}, fs, time::Instant, vec};
use rand::Rng;

#[derive(Clone, Copy)]
enum Tinstance {
    NC,
    FC,
    SS
}
struct Node {
    id : usize,
    used : bool,
    cost : u32,
    weight : u32
}
impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "id : {}, used : {}", self.id, self.used)
    }
}
struct Sacados {
    instance : Vec<Objet>,
    poids_max : u32,
    sol : Vec<bool>
}
#[derive(PartialEq, PartialOrd,  Clone)]
struct Objet {
    poids : u32,
    valeur : u32,
    ratio : f32,
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
        res.sort_by(|x, y| x.ratio.partial_cmp(&y.ratio).unwrap());
        res
    }
    fn glouton(&self) -> Vec<Objet> {
        let mut instances = self.sort_instances();
        let mut poidsac: u32 = 0;
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
    fn sol_glouton(sorted : &[Objet], poids_max: u32) -> f32 {
        let mut poidsac = 0;
        let mut valmax = 0;
        for obj in sorted.iter() {
            if poidsac+obj.poids <= poids_max {
                poidsac += obj.poids;
                valmax += obj.valeur;
            }
        }
        valmax as f32
    }
    fn sol_glouton_relax(sorted : &[Objet], poids_max: u32, level : usize, mut poidsac: u32) -> f32 {
        
        let mut valmax: u32 = 0;
        for obj in sorted[level..].iter() {
            if poidsac > poids_max {
                return valmax as f32 + (obj.valeur as f32 / 2.);
            }
            poidsac += obj.poids;
            valmax += obj.valeur;
        }
        valmax as f32
    }
    fn get_know_instance() -> Self {
        let obj0 = Objet {valeur : 3, poids : 2, ratio : 0.};
        let obj1 = Objet {valeur : 4, poids : 3, ratio : 0.};
        let obj2= Objet {valeur : 8, poids : 4, ratio : 0.};
        let obj3 = Objet {valeur : 8, poids : 5, ratio : 0.};
        let obj4 = Objet {valeur : 10, poids : 9, ratio : 0.};
        let instance = Vec::from([obj0, obj1, obj2, obj3, obj4]);
        Self { instance , poids_max: 10, sol : Vec::new()}
    }
    fn gen_rand_instances(nb_inst : u32, r: u32, type_instances : Tinstance ) -> Self {
        let mut instances = Vec::with_capacity(nb_inst as usize);
        let mut rng = rand::thread_rng();
        let mut pmax: u32 = 0;
        for _ in 0..nb_inst {
            let (u, p) = match type_instances {
                Tinstance::NC => {
                    let p: u32 = rng.gen_range(1..r);
                    let u: u32 = rng.gen_range(1..r);
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
            let obj = Objet {valeur : u, poids : p, ratio : u as f32 / p as f32};
            pmax += p;
            instances.push(obj);
        }
        Self { instance : instances, poids_max:  pmax/2, sol : Vec::new()}
    }
    fn arborescence(&mut self) {
        let mut pile = Vec::with_capacity(self.instance.len());
        let mut used = vec![false;self.instance.len()];
        let mut sol = Vec::new();
        let mut poidsac: u32 = 0;
        let mut cost: u32 = 0;
        let mut best_score = 0;
        let mut step: u64 = 0;
        pile.push(Node{id : 0, used : false, cost :0, weight:0});
        pile.push(Node{id : 0, used : true, cost:0, weight:0});
        while !pile.is_empty() {
            let mut id_taken = 0;
            {   step +=1;
                let node = pile.last().unwrap();
                //println!("{}", node);
                id_taken = node.id;
                
                if node.used {
                    if poidsac + self.instance[node.id].poids as u32 <= self.poids_max as u32 {
                        used[node.id] = node.used;
                        poidsac += self.instance[node.id].poids as u32;
                        cost += self.instance[node.id].valeur as u32;
                        if cost > best_score {
                            sol = used.clone();
                            best_score = cost;
                            //println!("{} {step}", cost);
                        }
                    }
                    else {
                        unsafe { pile.set_len(pile.len()-1); }
                        continue;
                    }
                }
                else {
                    if used[node.id] {
                        used[node.id] = false;
                        poidsac -= self.instance[node.id].poids as u32;
                        cost -= self.instance[node.id].valeur as u32;
                    }
                    /*println!("-----------------------------------");
                    println!("{}", self.instance[node.id].valeur);
                    println!("{cost} {step} {}",node.id);*/
                    
                }
            }
                //unsafe { pile.set_len(pile.len()-1);}
                if id_taken < self.instance.len()-1 {
                    let r = pile.len();
                    pile[r-1] = Node{id : id_taken+1, used : false, cost:0, weight:0};
                    pile.push(Node{id : id_taken+1, used : true,cost:0, weight:0});
                }
                else {
                    unsafe { pile.set_len(pile.len()-1); }
                }
        }
        //println!("{:?}", sol);
        self.sol = sol;
        println!("step : {}", step);
    }
    fn branch_and_bound(&mut self) {
        let mut pile = Vec::with_capacity(self.instance.len());
        let mut used = vec![false;self.instance.len()];
        let mut sol = Vec::with_capacity(self.instance.len());
        unsafe { sol.set_len(self.instance.len()); }
        
        let mut best_score = 0;
        let mut step: u64 = 0;
        let sorted = Sacados::sort_instances(self);
        pile.push(Node{id : 0, used : false, cost:0,weight:0});
        pile.push(Node{id : 0, used : true,cost:0,weight:0});
        best_score = Sacados::sol_glouton(&sorted, self.poids_max) as u32;

        while !pile.is_empty() {
            step +=1;
            let node = pile.pop().unwrap();
            let mut poidsac = node.weight;
            let mut cost = node.cost;
            used[node.id] = node.used;
            if node.used {
                if poidsac + sorted[node.id].poids <= self.poids_max {
                    used[node.id] = true;
                    poidsac += sorted[node.id].poids;
                    cost += sorted[node.id].valeur;
                    if cost > best_score {
                        sol.copy_from_slice(&used);
                        best_score = cost;
                    }
                }
                else { continue; }
            }
            if node.id < sorted.len()-1 {
                let limite = Sacados::sol_glouton_relax(&sorted, self.poids_max, node.id+2, poidsac);
                let limite2 = Sacados::sol_glouton_relax(&sorted, self.poids_max, node.id+1, poidsac);
                if limite as f32 > best_score as f32 {
                    pile.push(Node{id : node.id+1, used : false, cost, weight:poidsac});
                }
                if limite2 as f32 > best_score as f32 {
                    pile.push(Node{id : node.id+1, used : true,cost, weight:poidsac});
                }
            }
        }
        //println!("{:?}", sol);
        self.sol = sol;
        println!("{best_score}");
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
            let tf = (tmax as f32 * i as f32 / 10.) as u32;
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

fn main() {
    println!("TME35");
    //test();
    //test_one();
    let t = 1700;
    let mut sac = Sacados::gen_rand_instances(t, t/2, Tinstance::NC);
    //let mut sac = Sacados::get_know_instance();
    println!("{}", sac);
    let start = Instant::now();
    //sac.arborescence();
    sac.branch_and_bound();
    println!("{} msec", start.elapsed().as_millis());
    println!("{:?}", sac.sol);
}