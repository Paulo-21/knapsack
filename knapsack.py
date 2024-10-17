import random

class Objet:
    def __init__(self, poids, valeur):
        self.poids = poids
        self.valeur = valeur
        self.ratio = valeur / poids  # Ratio valeur/poids

class Noeud:
    def __init__(self, niveau, valeur, poids, borne_sup, objets_pris):
        self.niveau = niveau
        self.valeur = valeur
        self.poids = poids
        self.borne_sup = borne_sup
        self.objets_pris = objets_pris

class Tinstance:
    NC = "NC"  # Cas normal
    FC = "FC"  # Cas fortement corrélé
    SS = "SS"  # Cas super fortement corrélé

# Fonction pour calculer la borne supérieure (upper bound)
def calculer_borne_sup(niveau, poids, valeur, C, objets):
    if poids >= C:
        return 0

    borne_sup = valeur
    total_poids = poids

    for i in range(niveau, len(objets)):
        if total_poids + objets[i].poids <= C:
            total_poids += objets[i].poids
            borne_sup += objets[i].valeur
        else:
            borne_sup += (C - total_poids) * objets[i].ratio
            break

    return borne_sup

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

# Algorithme branch and bound pour le problème du sac à dos
def sac_a_dos_branch_and_bound(objets, C):
    objets.sort(key=lambda x: x.ratio, reverse=True)
    meilleure_valeur = 0
    meilleure_combinaison = []
    queue = []

    noeud_racine = Noeud(niveau=0, valeur=0, poids=0, borne_sup=calculer_borne_sup(0, 0, 0, C, objets), objets_pris=[])
    queue.append(noeud_racine)

    while queue:
        noeud = queue.pop(0)

        if noeud.niveau < len(objets):
            nouveau_poids = noeud.poids + objets[noeud.niveau].poids
            nouveau_valeur = noeud.valeur + objets[noeud.niveau].valeur

            if nouveau_poids <= C:
                if nouveau_valeur > meilleure_valeur:
                    meilleure_valeur = nouveau_valeur
                    meilleure_combinaison = noeud.objets_pris + [noeud.niveau]

                borne_sup = calculer_borne_sup(noeud.niveau + 1, nouveau_poids, nouveau_valeur, C, objets)
                if borne_sup > meilleure_valeur:
                    queue.append(Noeud(noeud.niveau + 1, nouveau_valeur, nouveau_poids, borne_sup, noeud.objets_pris + [noeud.niveau]))

            borne_sup = calculer_borne_sup(noeud.niveau + 1, noeud.poids, noeud.valeur, C, objets)
            if borne_sup > meilleure_valeur:
                queue.append(Noeud(noeud.niveau + 1, noeud.valeur, noeud.poids, borne_sup, noeud.objets_pris))

    return meilleure_valeur, meilleure_combinaison

# Exemple d'utilisation
if __name__ == "__main__":
    # Générer des instances aléatoires
    nb_instances = 35
    r = 100
    type_instances = Tinstance.NC  # Choisir entre Tinstance.NC, Tinstance.FC, Tinstance.SS

    objets, capacite_sac = gen_rand_instances(nb_instances, r, type_instances)
    
    # Résoudre le problème du sac à dos
    meilleure_valeur, meilleure_combinaison = sac_a_dos_branch_and_bound(objets, capacite_sac)
    
    print("Meilleure valeur obtenue :", meilleure_valeur)
    print("Objets pris (indices) :", meilleure_combinaison)
