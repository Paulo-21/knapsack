import random
import itertools

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

    # Essayer toutes les combinaisons possibles d'objets (2^n combinaisons)
    for combinaison in itertools.product([0, 1], repeat=n):
        poids_total = 0
        valeur_totale = 0
        objets_pris = []

        for i in range(n):
            if combinaison[i] == 1:  # Si l'objet i est pris
                poids_total += objets[i].poids
                valeur_totale += objets[i].valeur
                objets_pris.append(i)

        # Si le poids total est inférieur à la capacité et la valeur est meilleure, on met à jour
        if poids_total <= C and valeur_totale > meilleure_valeur:
            meilleure_valeur = valeur_totale
            meilleure_combinaison = objets_pris

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
