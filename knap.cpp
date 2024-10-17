#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

// Structure représentant un objet dans le problème du sac à dos
struct Objet {
    int poids;
    int valeur;
};

// Fonction pour générer des instances de sac à dos aléatoires
vector<Objet> gen_rand_instances(int nb_inst, int r, string type_instances) {
    vector<Objet> instances;
    int pmax = 0;

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

        instances.push_back({p, u});
        pmax += p;
    }

    return instances;
}

// Fonction brute force pour le sac à dos 0/1
pair<int, vector<int>> sac_a_dos_bruteforce(const vector<Objet>& objets, int C) {
    int n = objets.size();
    int meilleure_valeur = 0;
    vector<int> meilleure_combinaison;

    // Essayer toutes les combinaisons possibles d'objets (2^n combinaisons)
    for (int i = 0; i < (1 << n); ++i) {
        int poids_total = 0;
        int valeur_totale = 0;
        vector<int> objets_pris;

        for (int j = 0; j < n; ++j) {
            if (i & (1 << j)) {  // Si l'objet j est pris
                poids_total += objets[j].poids;
                valeur_totale += objets[j].valeur;
                objets_pris.push_back(j);
            }
        }

        // Si le poids total est inférieur à la capacité et la valeur est meilleure, on met à jour
        if (poids_total <= C && valeur_totale > meilleure_valeur) {
            meilleure_valeur = valeur_totale;
            meilleure_combinaison = objets_pris;
        }
    }

    return {meilleure_valeur, meilleure_combinaison};
}

int main() {
    // Générer des instances aléatoires
    int nb_instances = 24;
    int r = 100;
    string type_instances = "NC";  // Choisir entre "NC", "FC", "SS"

    vector<Objet> objets = gen_rand_instances(nb_instances, r, type_instances);

    // Définir la capacité du sac à dos
    int capacite_sac = 150;  // Exemple de capacité

    // Résoudre le problème du sac à dos
    auto [meilleure_valeur, meilleure_combinaison] = sac_a_dos_bruteforce(objets, capacite_sac);

    // Afficher les résultats
    cout << "Meilleure valeur obtenue : " << meilleure_valeur << endl;
    cout << "Objets pris (indices) : ";
    for (int index : meilleure_combinaison) {
        cout << index << " ";
    }
    cout << endl;

    return 0;
}
