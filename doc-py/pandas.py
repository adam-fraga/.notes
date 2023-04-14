import pandas as pd

# Lire un fichier (Supporte plusieurs type de fichiers 
# csv, excel, xml, json html, txt....)
df = pd.read_csv("p2-arbres-fr.csv", sep=";")

# Vérifier les data du dataframe
df.shape

# Retourne les lignes d'entête du dataframe
print(df)

# Supprimer les clonnes d'un dataframe (axis = 1 -> abscisse)
df.drop(["NomCol1", "NomCol2","NomCol6"], axis=1)

# Fournis les statistique de base pour chaque colonne
# Mean étant la moyenne, valeur min et valeur max...
# Count représente les lignes si elles ne sont pas toutes égal il manque des data
# Si les valeur contenue dans la moyenen sont des booléenne le resultat sera une proportion et donc un pourcentage
df.describe()

# Fournis des informations sur les types de données
df.info()

# Si il manque des data 2 options

# 1 - Remplacer les case vide par une valeur aléatoire dans le cas ci dessous l'age moyen 
# (Le risque est de corrompre le dataset original)
df = df.fillna(data["age"].mean())

# 2 - Supprimer les lignes comportant des valeurs vide (la valeur inplace fixé à true modifie la variable du df
# Pas besoin de l'assigné en valeur à une nouvelle variable)
# (Perte de donnée et donc d'effiscience)
df.dropna(axis=0, inplace=True)
# On affiche le nombre de ligne ensuite avec shape pour voir le nombre de data
df.shape

# Compte les repetitions
df["numero"].values_counts()
# Obtenir la repartition de ces information sous forme graphique
# plot, scatter, bar, hist pour histogramme, 
df["numero"].values_counts().plot.bar()
df["numero"].values_counts().plot.hist(bins=)
df["numero"].values_counts().plot.scatter(x= , y=)
df["numero"].values_counts().plot.bar()
pd.ploting.scatter_matrix(df)

# Seaborn marche aussi sur jupyter (plus de performance)

# Analyse par groupe de donnée
df.groupby(["sexe", "age"]).mean()

# On peut assemblé des series entre elles quand elle partage un même axe (index)
# Remplace les index numérique du dataframe par des données
df = df.set_index("name")
df["age"]

# Les Dataframe sont une succession de colonnes identifié par un index ce sont plus simplement
# Des dictionnaires qui contiennent des series

# Panda incorpore un systeme de series et de dataframe, les series sont constitués d'un tableau classique
# colonne clés et colonne valeur
df["âge"]

# Indexing récupère le 10 premiere ligne dans la series âge
df["âge"][0:10]

# Mask (Renvoi dans ce cas un boolean de si oui ou non majeur)
df["âge"] < 18
# Injecte le Mask dans le dataframe, Récupère donc uniquement les passagers mineur
df = [df["âge"] < 18]

# Applique le masque et genère un nouveau Dataset avec le nombre des différents passagers mineur par class
df = [df["âge"] < 18]["pclass"].values_counts()

# Regroupe les passager mineur selon leur class et leur genre et définit leur moyenne en pourcentage
df = [df["âge"] < 18]["pclass"].groupby(["sex","pclass"]).mean()

# Fonctions d'indexing ILOC et LOC

# A la façon de numpy on peux acceder aux index avec panda au lieu de data[0][0]:
df.illoc[0, 0]
# On peux ainsi faire de l'indexing, du slicing, appliqué le pas etc...
df.illoc[0:5, 0:5]

# Loc proccède de la même manière mais permet de travailler avec les colonnes
df.loc[0:3, 'age', "sex"]
