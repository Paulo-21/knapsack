#include <iostream>
#include <vector>
#include <algorithm>
#include <queue>

using namespace std;

// Structure représentant un objet dans le problème du sac à dos
struct Objet {
    int poids;
    int valeur;
    double ratio; // valeur/poids
};

// Structure représentant un nœud dans l'arbre de recherche Branch and Bound
struct Noeud {
    int niveau; // Le niveau dans l'arbre (correspondant à l'objet examiné)
    int valeur; // Valeur accumulée de l'objet sélectionné
    int poids;  // Poids accumulé de l'objet sélectionné
    double limite; // Limite supérieure estimée de la valeur de ce nœud

    // Comparateur pour que la file de priorité priorise les nœuds avec la limite la plus haute
    bool operator<(const Noeud& autre) const {
        return limite < autre.limite;
    }
};

// Fonction pour calculer la limite supérieure (bound) d'un nœud
double calculer_limite(int niveau, int poids_total, int valeur_total, int C, const vector<Objet>& objets) {
    if (poids_total >= C) {
        return 0; // Si le poids dépasse la capacité, on retourne une borne nulle
    }

    // Commence avec la valeur accumulée
    double valeur_max = valeur_total;
    
    // Ajoute autant d'objets que possible en respectant la capacité
    int poids_restant = C - poids_total;
    while (niveau < objets.size() && objets[niveau].poids <= poids_restant) {
        poids_restant -= objets[niveau].poids;
        valeur_max += objets[niveau].valeur;
        niveau++;
    }

    // Si on ne peut pas ajouter un objet en entier, on prend une fraction (greedy)
    if (niveau < objets.size()) {
        valeur_max += objets[niveau].valeur * ((double) poids_restant / objets[niveau].poids);
    }

    return valeur_max;
}

// Fonction de comparaison pour le tri en fonction du ratio valeur/poids
bool comparer(const Objet& a, const Objet& b) {
    return a.ratio > b.ratio;
}

// Fonction pour générer une solution gloutonne comme borne inférieure initiale
int solution_gloutonne(int C, const vector<Objet>& objets) {
    int poids_total = 0;
    int valeur_total = 0;
    
    for (const auto& objet : objets) {
        if (poids_total + objet.poids <= C) {
            poids_total += objet.poids;
            valeur_total += objet.valeur;
        } else {
            break;
        }
    }

    return valeur_total;
}

// Algorithme Branch and Bound pour le sac à dos 0/1 avec optimisation
int sac_a_dos_branch_and_bound(int C, vector<Objet>& objets) {
    // Trie les objets selon le ratio valeur/poids décroissant
    sort(objets.begin(), objets.end(), comparer);

    // File de priorité pour explorer les nœuds avec la limite la plus élevée
    priority_queue<Noeud> file;
    Noeud u, v;
    
    // Noeud racine (niveau -1, poids 0, valeur 0)
    v.niveau = -1;
    v.poids = 0;
    v.valeur = 0;
    v.limite = calculer_limite(0, 0, 0, C, objets);
    file.push(v);

    // Meilleure solution initiale avec un algorithme glouton
    int meilleure_valeur = solution_gloutonne(C, objets);

    while (!file.empty()) {
        v = file.top(); // On récupère un nœud à traiter
        file.pop();

        // Si ce nœud a une meilleure valeur possible que la meilleure valeur actuelle
        if (v.limite > meilleure_valeur) {
            // Considérer le niveau suivant dans l'arbre de décision (niveau + 1)
            u.niveau = v.niveau + 1;

            // Cas où l'objet est inclus
            if (u.niveau < objets.size()) {
                u.poids = v.poids + objets[u.niveau].poids;
                u.valeur = v.valeur + objets[u.niveau].valeur;

                if (u.poids <= C && u.valeur > meilleure_valeur) {
                    meilleure_valeur = u.valeur; // Mettre à jour la meilleure valeur
                }

                u.limite = calculer_limite(u.niveau + 1, u.poids, u.valeur, C, objets);
                if (u.limite > meilleure_valeur) {
                    file.push(u); // Si la limite est meilleure, on explore cette branche
                }

                // Cas où l'objet n'est pas inclus
                u.poids = v.poids;
                u.valeur = v.valeur;
                u.limite = calculer_limite(u.niveau + 1, u.poids, u.valeur, C, objets);
                if (u.limite > meilleure_valeur) {
                    file.push(u); // Si la limite est meilleure, on explore cette branche
                }
            }
        }
    }

    return meilleure_valeur;
}

// Fonction pour générer des instances de sac à dos aléatoires
vector<Objet> gen_rand_instances(int nb_inst, int r, string type_instances, int& somme_poids) {
    vector<Objet> instances;
    somme_poids = 0;

    for (int i = 0; i < nb_inst; ++i) {
        int p, u;

        if (type_instances == "NC") { // Cas normal
            p = rand() % r + 1;
            u = rand() % r + 1;
        } else if (type_instances == "FC") { // Fortement corrélé
            p = rand() % r + 1;
            u = rand() % (p + r / 10) + max(1, p - r / 10);
        } else if (type_instances == "SS") { // Super fortement corrélé
            p = rand() % r + 1;
            u = p;
        }

        instances.push_back({p, u, (double)u / p});
        somme_poids += p;
    }

    return instances;
}

int main() {
    // Générer des instances aléatoires
    int nb_instances = 9996;
    int r = 100;
    string type_instances = "NC";  // Choisir entre "NC", "FC", "SS"
    int somme_poids = 0;

    vector<Objet> objets = gen_rand_instances(nb_instances, r, type_instances, somme_poids);

    // Définir la capacité du sac à dos comme la somme des poids divisée par deux
    int capacite_sac = somme_poids / 2;

    // Résoudre le problème du sac à dos avec Branch and Bound
    int meilleure_valeur = sac_a_dos_branch_and_bound(capacite_sac, objets);

    // Afficher les résultats
    cout << "Capacité du sac : " << capacite_sac << endl;
    cout << "Meilleure valeur obtenue : " << meilleure_valeur << endl;

    return 0;
}
