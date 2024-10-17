import random
from itertools import combinations

class Objet:
    def __init__(self, poids, valeur):
        self.poids = poids
        self.valeur = valeur

class Tinstance:
    NC = "NC"  # Cas normal
    FC = "FC"  # Cas fortement corrélé
    SS = "SS"  # Cas super fortement corrélé

# Fonction pour générer des instances de sac à dos aléatoires
def gen_rand_instances(nb_inst, r, type_instances):
    instances = []
    pmax = 0

    for _ in range(nb_inst):
        if type_instances == Tinstance.NC:
            p = random.randint(1, r)
            u = random.randint(1, r)
        elif type_instances == Tinstance.FC:
            p = random.randint(1, r)
            u = random.randint(max(1, p - r // 10), p + r // 10)
        elif type_instances == Tinstance.SS:
            p = random.randint(1, r)
            u = p

        obj = Objet(valeur=u, poids=p)
        instances.append(obj)
        pmax += p

    poids_max = pmax // 2
    return instances, poids_max

# Algorithme brute force pour le sac à dos 0/1
def sac_a_dos_bruteforce(objets, C):
    n = len(objets)
    meilleure_valeur = 0
    meilleure_combinaison = []

    # Explorer toutes les combinaisons possibles d'objets
    for k in range(1, n + 1):
        for combinaison in combinations(range(n), k):
            poids_total = sum(objets[i].poids for i in combinaison)
            valeur_totale = sum(objets[i].valeur for i in combinaison)

            if poids_total <= C and valeur_totale > meilleure_valeur:
                meilleure_valeur = valeur_totale
                meilleure_combinaison = list(combinaison)

    return meilleure_valeur, meilleure_combinaison

# Exemple d'utilisation
if __name__ == "__main__":
    # Générer des instances aléatoires
    nb_instances = 23
    r = 100
    type_instances = Tinstance.NC  # Choisir entre Tinstance.NC, Tinstance.FC, Tinstance.SS

    objets, capacite_sac = gen_rand_instances(nb_instances, r, type_instances)

    # Résoudre le problème du sac à dos
    meilleure_valeur, meilleure_combinaison = sac_a_dos_bruteforce(objets, capacite_sac)

    print("Meilleure valeur obtenue :", meilleure_valeur)
    print("Objets pris (indices) :", meilleure_combinaison)
