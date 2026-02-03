---
header-includes:
  - \usepackage{tabularx}
  - \usepackage{fontspec}
---

\begin{titlepage}
\centering

\vspace*{0.22\textheight}

{\Huge \textbf{Concept of Operations} \par}

\vspace{2cm}

{\Large
MAXIME DRAPEAU \\[0.5cm]
MATHIS HURTUBISE \\[0.5cm]
ÉMILE LOCAS \par}

\vspace{2cm}

{\large
Projet de sudoku \\[0.5cm]
INM5151, gr. 51 \par}

\vspace{2cm}

Travail présenté à \\[0.3cm]
M. Johnny Tsheke Shele \\[0.3cm]
Département d’informatique \\[0.3cm]
Université du Québec à Montréal

\vfill

Le 10 octobre 2025

\end{titlepage}

# 1. INTRODUCTION

## 1.1 Identification du système

Le nouveau système présenté dans ce document est une application complète
de Sudoku.

Cette application peut générer et valider des grilles de Sudoku
pour l'utilisateur en plus de lui permettre de remplir des grilles de Sudoku
(soit générées ou importées).

## 1.2 Objectifs du document

L'objectif de ce document est de définir les attentes et les besoins des
usagers de ladite application aux personnes concernées par son développement.

Ceci sera fait en décrivant le mieux possible: la vision d'ensemble du projet,
son contexte d'utilisation et des besoins qu'il tente de remplir.

Voici la liste des acteurs concernées par ce projet:

  - **Développeurs :** Les développeurs se chargent de la conception et le
                       développement de cette application. Ce document leur
                       sert de guide pour répondre le plus fidèlement possible
                       aux besoins et aux attentes des usagers qui utiliseront
                       l'application.

  - **Usagers** : Les usagers sont les utilisateurs finaux de l'appllication de
                  Sudoku proposé dans ce document. Ces usagers sont présumés
                  des amateurs de Sudoku à la recherche d'une application
                  complète qui permet de générer, valider et remplir des
                  grilles de Sudoku. Ce document a pour but de décrire, le
                  plus fidèlement possible, leurs attentes et besoins envers
                  le développement de cette application.

## 1.3 Vue d’ensemble du système

*Complete Sudoku* est une application destinée à la génération procédurale,
la validation et la résolution automatiques de grilles de Sudoku, ainsi qu'au
jeu interactif *Sudoku* sur des grilles importées ou générées.

Le système fonctionne localement de l’utilisateur (soit avec une interface
graphique ou directement sur la console) et ne requiert aucune connexion
externe obligatoire. En effet, l'utilisateur n'a qu'à télécharger
l'application pour l'utiliser en local sur sa machine.

**Voici une vue d'ensemble des fonctionalitées :**

- **Génération procédurale** de grilles de Sudoku 
- **Résolution automatique** de grilles de Sudoku 
- **Validation automatique** de grilles de Sudoku
- **Importation** par l'utilisateur de ses propres grilles
- Permet de **jouer au Sudoku** avec ses grilles importées ou ceux générées
  par l'application
- **Retour en temps réel** sur la saisie lors du gameplay
  (**détection d"erreurs** et **indices**)

Le développement de ce système sera fait dans le cadre du cours **INM-5151**
(UQAM).

\newpage

# 2. RÉFÉRENCES

**1. Exemple d’implémentation ESTM sur GitHub**  
- **Version :** —  
- **Date :** —  
- **Lien :** [https://github.com/robert/wavefunction-collapse](https://github.com/robert/wavefunction-collapse)

---

**2. Implémentation originale de l’algorithme “mxgmn/WaveFunctionCollapse”**  
- **Version :** 1.00  
- **Date :** 21 juin 2021  
- **Lien :** [https://github.com/mxgmn/WaveFunctionCollapse](https://github.com/mxgmn/WaveFunctionCollapse)

---

**3. Règles du Sudoku**
- **Lien :** [https://sudoku.com/fr/comment-jouer/regles-de-sudoku-pour-les-debutants-complets](https://sudoku.com/fr/comment-jouer/regles-de-sudoku-pour-les-debutants-complets)

\newpage

# 3. LA SITUATION OU LE SYSTÈME ACTUEL

## 3.1 Contexte, objectifs et portée du système actuel

Actuellement, pour jouer au Sudoku, les utilisateurs utilisent soit des
méthodes manuelles (papier/crayon) ou des applications souvent incomplètes.
En effet, il existe beaucoup d'applications pour jouer et résoudre des Sudokus,
mais celles-ci n'ont pas toutes les fonctionnalités de bases que nous recherchons.

C'est dans le contexte d'offrir une expérience complète et adaptée que nous
développons cette nouvelle application.

## 3.2 Politiques opérationnelles et contraintes

La plupart des jeux de Sudoku en ligne requièrent une connexion internet.
Il y a aussi, parfois, des contraintes en fonction du navigateur utilisé et sa
version.

## 3.3 Description du système actuel ou de la situation courante

La situation courante repose sur l'utilisation d'applications locales ou en
ligne. Ces applications offrent souvent la plupart des fonctionnalités de
bases attendues, mais rarement tous à la fois. Cela fait en sorte que les
utilisateurs doivent alterner entre plusieurs applications différentes pour
pouvoir accomplir certaines tâches. En effet, ces applications n'offrent
souvent pas la possibilité à l'utilisateur d'importer ses grilles ou le
retour en temps réel.

Dans toutes les applications qui ont été testeés, les règles utilisés pour
la complétion d'une grille de Sudoku sont les règles de bases avec lesquelles
vous êtes familiées (voir la 3e référence à la section 2). En voici un aperçu:

  - La partie se déroule sur une grille 9x9
  - Cette grille est aussi séparé en 9 sous-grilles (3x3)
  - La grille 9x9 est partiellement remplie de chiffres (1-9)
  - les chiffres (1-9) doivent apparaître une fois par colonne
  - les chiffres (1-9) doivent apparaître une fois par ligne
  - les chiffres (1-9) doivent apparaître une fois par sous-grille

Certaines applications utilisent un minuteur pour ajouter un autre défi dans
la complétion de la grille: le temps.

## 3.4 Les modes d’opération du système ou de la situation actuelle

Certaines applications en ligne ont des mode de jeux différents. Par exemple,
sur *sudoku.com*, il y a 4 mode de jeu pour le *Sudoku*: Tournoi, Monde de la
Jungle et Défis quotidiens. Tournoi est un mode ou différents usagers
compétitionnent dans un classement, Monde de la Jungle est une aventure
individuelle avec des récompenses et Défis quotidiens est une grille de
Sudoku commune à tous les utilisateurs à chaque jours. Ce dernier mode
(Défi quotidien) est très populaire pour ce genre de jeux en lignes et
popularisé par Wordle.

Il y a aussi, dans la plupart des applications, un moyen de séléctionner la
difficulté des grilles données à l'utilisateur.

## 3.5 Les classes d’utilisateurs et les autres personnels impliqués

### 3.5.1 La structure organisationnelle

Dans l'état actuel, il n'y a aucune structure organisationnelle. Les joueurs
interagissent avec les applications locales en et ligne individuellement et
leur position n'est pas importante.

### 3.5.2 Le profil de chaque classe d’utilisateurs

Les **joueurs** sont ceux qui utilisent l'application pour générer, résoudre
ou jouer au jeu. Ils ont l'option de choisir le mode de jeu, la difficulté, de
demander des indices ou d'abandonner/recommencer une grille.

### 3.5.3 Les interactions entre les utilisateurs

Les **joueurs** n'intéragissent pas directement entre eux durant le jeu. Dans
certains cas, ils intéragissent indirectement dans des modes de jeux
multijoueurs (tournoi/classé) par interim d'un classement par exemple.

### 3.5.4 Autre personnel impliqué

Certaines **applications en ligne** permettent aux utilisateurs de contacter
un département de support technique qui répond possiblement aux questions des
utilisateurs.

## 3.6 L’environnement de support

Les jeux de Sudoku existent sur la très grande majorité des systèmes
d'exploitations (ordinateurs personnels et téléphones). Il existe une
multitude de distributeurs différents, qui ont leurs propres forums ou pages
de support.

Pour les applications en lignes, la maintenance et la gestion de ceux-ci sont
effectuées par les compagnies qui hébèrgent les sites/serveurs. Certaines de
ses applications ont des moyens de contacter des équipes de supports pour
signaler des bugs.

\newpage

# 4. LA JUSTIFICATION ET LA NATURE DES CHANGEMENTS

## 4.1 Justifications des changements

Les solutions actuelles pour jouer au Sudoku sont nombreuses mais souvent
fragmentées. Certaines se limite au simple jeu, d'autres à la résolution,
etc. Ces applications ont rarement l'ensemble des fonctionalitées que nous
considérons essentielles pour une application complète. De plus, la fonction
d'import de grilles personnalisées est plutôt rare dans l'écosystème actuel.

Par conséquent, le changemenet proposé répond à ses lacunes avec une
application locale complète. Celle-ci regroupera les fonctionnalités
essentielles que nous avons identifiés: la **génération/résolution** de
grilles avec **WFC**, la **validation continue** et des **indices en temps
réel** et l'**import de grilles personnalisées**.

## 4.2 Description des changements

Notre application n'offre pas de changement structurant à l'expérience
Sudoku, mais consolide dans un même outil toutes les fonctionnalités utiles
au Sudoku pour créer une expérience complète et cohérante.

**L'import de grilles personnalisées** pour validation, résolution et faire
une partie avec est le plus près d'une innovation que notre application
offrera. Cette fonctionnalité est souvent réservé aux outils de résolution
automatique. En laissant jouer les utilisateurs avec leurs propres grilles,
ont leur permet d'intégrer leurs livres de Sudoku à notre expérience
complète de Sudoku.

## 4.3 L’ordre de priorité dans les changements

**Essentiels**:

  - Génération et résolution avec WFC
  - Import de grilles personnalisées (format texte simple)
  - Pouvoir jouer sur des grilles générées et importées 
  - Validation continue (détection d'erreurs/indices)

**Souhaitable**:

  - Contrôle de difficulté lors de la génération
  - Détection de la difficulté lors de l'import
  - Minuteur et statistiques

**Optionnel**:

  - Interface graphique riche
  - Importation de grille à partir de l'interface graphique
  - Permettre la sauvegarde locale des parties
  - Permettre l'utilisation en ligne
  - Rendre accessible sur téléphones

\newpage

## 4.4 Les changements considérés mais non-inclus

Rendre notre application disponible sur les téléphones et en ligne ont été
considérés. Mais en raison des délais court pour le développement de
l'application, ces fonctionnalités ont été écartés pour prioriser des
fonctionnalités plus importantes.

## 4.5 Hypothèses et contraintes
Le temps alloué au développement étant restreint, certaines fonctionnalités
pourraient ne pas être implantées dans la version initiale, conformément à
la liste de priorités de la section 4.3.

\newpage

# 5. LE CONCEPT DU SYSTÈME PROPOSÉ

## 5.1 Contexte, objectifs et portée du système proposé

#### Contexte

Le Sudoku est un jeu de logique largement répandu, qui est interessant pour sa
capacite a nous faire penser de facon logique 
- la génération de grilles de qualité, adaptées au niveau de l’utilisateur,
  peut avoir le problème de creer des grilles insolvable,  
- la résolution automatique des grilles, lorsqu’elle existe, pose un danger
  que la resolution peut etre fait de facon inneficient  
- les outils d’assistance au joueur sont dure a implementer si la resolution
  n'est pas efficace.  

L’algorithme de **Wave Function Collapse (WFC)**, utilise pour la génération
procédurale de contenus, offre une nouvelle approche pour résoudre et générer
des grilles de Sudoku en respectant les contraintes logiques du sudoku.

#### Mission

Le système proposé a pour mission de fournir une application complète de
Sudoku qui combine :  
- la **résolution automatique** de grilles grâce à l’algorithme WFC,  
- la **génération procédurale** de grilles variées, adaptées au niveau du
  joueur,  
- des **outils interactifs** permettant de jouer, recevoir des indices, et
  progresser dans la résolution de Sudoku.

#### Objectifs

Les principaux objectifs du système sont :  
1. **Mécaniser la résolution de Sudoku** à l’aide du WFC pour garantir
   rapidité et fiabilité.  
2. **Prévenir les erreurs** grâce à des mécanismes de validation et
   d’indication en temps réel.  
3. **Offrir multiple facon de jouer**, en offrant des configurations avec une
   solution unique ou plusieurs solutions.  
4. **Améliorer l’expérience utilisateur** par des fonctionnalités d’aide
   et d’apprentissage (indices, détection d’erreurs).  
5. **Assurer la performance et la robustesse** grâce à une implémentation en
   Rust, garantissant rapidité et sûreté mémoire.

Les stratégies et techniques utilisées incluent :  
- l’adaptation de l’algorithme WFC aux contraintes spécifiques du Sudoku,  
- La programmation parallele pour la vérification
- une architecture modulaire séparant le moteur de résolution/génération,
  l’interface utilisateur et les services d’assistance,  
- des tests automatiques afin d’assurer la validité des solutions et la
  stabilité de l’application.

#### Portée du système

- **Modes d’opération** :  
  - Résolution automatique de grilles données.  
  - Jeu interactif avec suivi, assistance et difficultés.  
  - Génération de nouvelles grilles selon divers paramètres.  

- **Classes d’utilisateurs** :  
  - **Joueurs occasionnels** recherchant un divertissement et une aide simple.  
  - **Joueurs avancés** désirant relever des défis complexes et tester des
    variantes.  
  - **Enseignants ou formateurs** utilisant le système pour démontrer une
    implémentation de l'algorithme wfc.  

- **Interfaces à l’environnement opérationnel** :  
  - Une interface utilisateur interactive (console dans un premier temps, avec
    possibilité d’extension vers une interface graphique).  
  - Un moteur interne de résolution/génération (basé sur Rust) relié à
    l’interface.  

Ainsi, le système proposé se situe à l’intersection entre un outil ludique
et pédagogique, et une démonstration technique de l’adaptation de
l’algorithme WFC à un problème logique concret.

## 5.2 Politiques opérationnelles et contraintes

#### Politiques opérationnelles

- **Disponibilité** :  
  Le système est conçu pour être disponible en continu (24/7), sans
  restriction horaire, puisque son utilisation se fait de manière autonome
  par l’utilisateur sur son poste de travail.  
- **Mode d’exploitation** :  
  L’application ne nécessite pas de supervision humaine spécifique : un seul
  utilisateur peut l’installer et l’utiliser directement.  
- **Soutien et maintenance** :  
  Des mises à jour logicielles peuvent être distribuées périodiquement pour
  corriger des bugs, améliorer la performance ou ajouter des fonctionnalités.  

#### Contraintes opérationnelles

- **Nombre d’utilisateurs simultanés** :  
  Le système fonctionne en mode individuel. Chaque instance du logiciel est
  destinée à être utilisée par une seule personne à la fois.  
- **Ressources humaines** :  
  Aucune ressource dédiée n’est nécessaire pour faire fonctionner le système.  
- **Environnement matériel requis** :  
  - Ordinateur de bureau ou portable équipé d’un processeur moderne (x86_64
    ou ARM).  
  - Système d’exploitation supportant l’exécution de binaires Rust
    compilés (Linux, Windows ou macOS).  
- **Environnement physique** :  
  Aucune infrastructure particulière n’est nécessaire. Le système peut
  fonctionner sur un poste standard.  
- **Performance attendue** :  
  Le temps de génération ou de résolution d’une grille doit rester
  inférieur à 1 seconde sur un matériel conforme aux spécifications
  minimales.  

En résumé, le système proposé est léger en termes de ressources
opérationnelles : il ne requiert pas de personnel dédié, peut être
exécuté sur des environnements informatiques standards, et offre une
disponibilité continue.  

## 5.3 Description du système proposé

Le système proposé est une application permettant de **jouer au Sudoku**, de
**recevoir de l’assistance**, et de **générer ou résoudre automatiquement des grilles**
grâce à l’algorithme de Wave Function Collapse (WFC).  
Cette section en présente l’environnement, les composants, les interfaces,
les principales fonctionnalités, ainsi que ses caractéristiques opérationnelles.

---

### Environnement opérationnel

- **Utilisateur cible** : personne souhaitant jouer, apprendre ou s’entraîner
                          avec le Sudoku.  
- **Support matériel** : ordinateurs portables ou de bureau standards
                         (Windows, Linux, macOS).  
- **Conditions d’utilisation** : le système fonctionne localement sur
                                 l’ordinateur de l’utilisateur.  
- **Disponibilité** : le système est accessible en tout temps.  

---

### Principaux composants et interconnexion

#### Schéma des composants

Le diagramme suivant illustre les principaux composants du système Sudoku WFC
et leurs interactions.  
Il montre les flux d’informations entre l’utilisateur, les modules internes
du système (génération, résolution, interface, indices) et la base de
données centrale.

![Diagramme des composants du système Sudoku WFC](diagramme1.pdf)

### Interfaces avec systèmes/procédures externes

- **Entrées** :  
  - Grilles de Sudoku fournies par l’utilisateur.  
  - Commandes de jeu (saisie de valeurs, demande d’indice, lancement d’une
    génération).  
- **Sorties** :  
  - Grille complétée (résolution automatique).  
  - Indices et suggestions pour aider l’utilisateur.  
  - Nouvelles grilles générées selon le niveau choisi.  

Le système n’a pas d’interfaçage obligatoire avec d’autres logiciels
externes, ce qui simplifie son intégration.

### Fonctionnalités principales

- Résolution automatique de grilles incomplètes.  
- Génération de nouvelles grilles (solutions uniques ou multiples).  
- Jeu interactif avec validation des entrées.  
- Assistance en temps réel (indices, erreurs, suggestions).  
- Paramétrage des niveaux de difficulté. 

![Diagramme du système Sudoku WFC](diagramme2.pdf)


Utilisateur :
  Interagit avec le système pour jouer, demander des indices ou résoudre une
  grille.

Système Sudoku WFC :
  Coeur du système – exécute la génération, la validation et la
  résolution.

Entrées utilisateur :
  Grilles partielles, niveau de difficulté, requêtes de génération ou de
  résolution.

Résultats Sudoku :
  Grilles générées, solutions complètes ou indices fournis à
  l’utilisateur.

Flux de données :
  Représentent les échanges d’informations entre l’utilisateur et le
  système.

![Flux de données du système Sudoku WFC](diagramme3.pdf)

Interface Utilisateur :
  Gère les interactions avec le joueur (saisie, affichage, requêtes).

Générateur de grille (WFC) :
  Crée de nouvelles grilles valides selon la difficulté choisie à l’aide de
  WFC.

Validateur logique :
  Vérifie la cohérence des grilles et leur conformité aux règles du Sudoku.

Moteur de résolution :
  Applique WFC pour résoudre ou compléter les grilles de manière optimisée.

Module d’assistance :
  Fournit des indices, détecte les erreurs et assiste le joueur pendant la
  partie.

Intrants :
  Données saisies par le joueur ou paramètres de génération.

Extrants :
  Solutions, grilles générées ou indices affichés à l’utilisateur.

Flux de contrôle :
  Transferts automatiques entre modules pour exécuter les requêtes
  utilisateur.

### Coûts d’opération

- **Coûts humains** : inexistants (le système ne nécessite pas
  d’opérateur dédié).  
- **Coûts techniques** : négligeables, le système s’exécute sur des
  environnements matériels standards.  
- **Maintenance** : limitée à l’application de mises à jour logicielles.  

### Facteurs de risques opérationnels

- Risque de **mauvaise génération** de grilles (grilles insolubles ou
  contradictoires) si l’algorithme est mal paramétré.  
- Risque lié aux **bugs logiciels**, pouvant compromettre l’expérience de
  jeu. 
- Risque que l'algorithme de resolution est trop lent.

### Caractéristiques de performance

- Temps de génération et de résolution attendu : < 1 seconde par grille
  standard.  
- Nombre d’utilisateurs simultanés : usage individuel, monoutilisateur.  
- Volumes : stockage léger, uniquement des grilles de Sudoku (données
  numériques minimales).  

### Attributs de qualité

- **Disponibilité** : 24/7 sur le poste local.  
- **Précision** : les solutions produites doivent être exactes et valides
  selon les règles du Sudoku.  
- **Efficience** : temps de calcul réduit grâce à Rust.  
- **Flexibilité** : possibilité d’ajouter des variantes de Sudoku.  
- **Facilité d’entretien** : code Rust modulaire et testé, simplifiant les
  évolutions.  
- **Fiabilité** : moteur algorithmique robuste, testé par des scénarios
  automatiques.  
- **Réutilisabilité** : le moteur WFC pourrait être réutilisé dans
  d’autres contextes logiques.  
- **Utilisabilité** : interface simple, adaptée aux débutants comme aux
  experts.  

### Sécurité, confidentialité et continuité

- **Sécurité** : aucune donnée sensible n’est manipulée.  
- **Confidentialité** : aucune information personnelle n’est stockée ou
  transmise.  
- **Intégrité** : mécanismes de validation évitent la génération de
  grilles incorrectes.  
- **Continuité** : en cas de fermeture du programme, l’utilisateur peut
  sauvegarder/reprendre sa partie (fonctionnalité optionnelle).  


## 5.4 Les modes d’opération du système proposé

Le système de Sudoku proposé fonctionne principalement en mode normal, mais
prévoit également des modes secondaires pour couvrir les situations
particulières. Les différents modes sont décrits ci-dessous.

#### Mode normal

- **Description** : c’est le mode d’utilisation standard du système.  
- **Fonctionnalités disponibles** :  
  - Génération de nouvelles grilles (solutions uniques ou multiples).  
  - Jeu interactif avec validation des entrées.  
  - Résolution automatique des grilles fournies par l’utilisateur.  
  - Assistance en temps réel (indices, erreurs, suggestions).  
- **Utilisateurs concernés** : tous les utilisateurs finaux (joueurs débutants
  ou avancés).

### Mode Visuel (Implementation optionnelle)
- **Description** : Ce mode permet d'utiliser toutes les fonctionnalités du
  mode normal mais avec un interface graphique
- **Utilisateurs concernés** : tous les utilisateurs finaux (joueurs
  débutants ou avancés).

#### Mode dégradé (Implementation optionnelle)

- **Description** : en cas de problème avec l'inteface graphique (si
  l'interface graphique est implemente)
- **Exemple** : l'interface graphique devient gelée
- **Utilisateurs concernés** : les utilisateurs avancés.

#### Mode urgence / backup (implementation optionnelle)

- **Description** : permet de restaurer une partie interrompue après une
  fermeture accidentelle ou un crash.  
- **Fonctionnalités disponibles** :  
  - Rechargement automatique de la dernière grille sauvegardée.  
  - Conservation de la progression de l’utilisateur.  
- **Utilisateurs concernés** : tout utilisateur ayant subi une interruption
  imprévue.  


## 5.5 Les classes d’utilisateurs et les autres personnels impliqués

#### 5.5.1 Structure organisationnelle

Le système proposé est principalement destiné à des **utilisateurs finaux
individuels**.  
La structure organisationnelle est donc relativement simple :  
- **Utilisateurs joueurs** : utilisent directement l’application pour jouer,
  apprendre, générer ou résoudre des grilles.  
- **Personnel de support/maintenance** : développeurs ou techniciens
  responsables des mises à jour et du support technique.  
- **Enseignants/formateurs** (optionnels) : utilisent le système comme outil
  pédagogique auprès de leurs étudiants.  

#### 5.5.2 Profil de chaque classe d’utilisateurs

1. **Joueurs débutants**  
   - **Responsabilités** : utiliser l’application pour apprendre ou se
     divertir.  
   - **Niveau de compétences** : faible à moyen en logique et en Sudoku.  
   - **Tâches principales** : jouer, demander des indices, comprendre leurs
     erreurs.  
   - **Mode d’interaction** : interface simple, recours fréquent au mode
     « formation ».  

2. **Joueurs avancés / experts**  
   - **Responsabilités** : relever des défis plus complexes et explorer des
     variantes de Sudoku.  
   - **Niveau de compétences** : élevé.  
   - **Tâches principales** : générer des grilles difficiles, résoudre sans
     assistance, tester la robustesse du système.  
   - **Mode d’interaction** : utilisation du mode normal, parfois sans aide
     ni indices.  

4. **Personnel de support/maintenance**  
   - **Responsabilités** : assurer le bon fonctionnement du système.  
   - **Niveau de compétences** : techniques (programmation en Rust, gestion
     de versions, tests).  
   - **Tâches principales** : corriger des bogues, publier des mises à jour,
     optimiser la performance.  
   - **Mode d’interaction** : accès au code source et outils de
     développement.  

---

#### 5.5.3 Interactions entre les utilisateurs

- Les **joueurs** n’interagissent pas entre eux dans le système, qui est
  orienté **usage individuel**.  
- Le **personnel de maintenance** interagit uniquement avec le système
  (via mises à jour), pas avec les joueurs finaux.  
- En dehors du système, des interactions informelles peuvent exister
  (partage de grilles générées, discussions pédagogiques).  

---

#### 5.5.4 Autre personnel impliqué

- **Clients finaux** : toute personne souhaitant pratiquer le Sudoku (joueur
  individuel, étudiant, amateur de logique).  

## 5.6 L’environnement de support

Le système proposé (application de Sudoku basée sur WFC) bénéficie d’un
environnement de support adapté aux besoins des utilisateurs finaux et du
personnel technique.

#### Agents et agences de support

- **Développeurs/équipe technique** : responsables du maintien du code, des
  mises à jour correctives et évolutives.  
- **Communauté d’utilisateurs** (forums, documentation en ligne) : offre un
  support informel pour l’entraide et le partage de grilles.  

#### Facilités offertes

- **Centre de support en ligne** : documentation technique et guide
  d’utilisation accessibles via le gitlab du projet.  
- **FAQ intégrée** : réponses aux problèmes fréquents directement
  accessibles dans l’application.  
- **Canal de communication (ex. courriel ou gitlab Issues)** pour signaler des
  anomalies ou suggérer des améliorations.  

#### Équipement et logiciels de support

- **Équipement requis** : ordinateur personnel standard, sans besoin
  spécifique de matériel.  
- **Logiciels de support** :  
  - Rust (langage et compilateur) pour la maintenance et les mises à jour.  
  - Outils de gestion de versions (ex. gitlab) pour le suivi des évolutions.  
  - Librairies d’interface graphique standards pour compatibilité
    multiplateforme.  

#### Critères de réparation et de remplacement

- **Réparation logicielle** : correction de bogues via mises à jour.  
- **Remplacement logiciel** : si une version devient obsolète ou incompatible,
                              une mise à niveau vers une version stable est
                              fournie.  

#### Niveaux et cycles de maintenance

- **Maintenance corrective** : correction des bogues rapportés par les
                               utilisateurs.  
- **Maintenance adaptative** : ajustement du logiciel pour compatibilité
                               avec de nouveaux systèmes d’exploitation.  
- **Maintenance évolutive** : ajout de nouvelles fonctionnalités.  
- **Cycle de maintenance** : publication de mises à jour mineures et
                             révisions majeures selon les besoins
                             pédagogiques ou techniques.  

#### Méthodes d’entreposage, de distribution et de fourniture des services

- **Entreposage** : le code source est conservé dans un dépôt GitLab
                    sécurisé et versionné.  
- **Distribution** : l’application est distribuée sous forme source et
                     construit par l'utilisateur il peut aussi etre fournit
                     sous format binaire l'executable.  
- **Fourniture des services** : le support est principalement assuré en ligne
                                sur gitlab (téléchargements, documentation,
                                mises à jour automatiques ou manuelles).  

---

En résumé, l’environnement de support repose sur une combinaison de
**documentation claire**, **mises à jour régulières** et **canaux de
communication directs** entre utilisateurs et développeurs, garantissant
la continuité et l’efficacité du système proposé.

\newpage

# 6. LES SCÉNARIOS D’OPÉRATION

Cette section présente différents scénarios d’utilisation du système de
Sudoku.  

## 6.1 Résolution automatique d’un Sudoku

**Nom** : Résoudre une grille de Sudoku  
**Niveau** : Objectif usager  
**Acteurs** :  
- Acteur primaire : joueur
- Acteur secondaire : Système de résolution automatique  

**Préconditions** :  
- Le joueur fournit une grille valide (9x9, avec chiffres 1–9 et cases
  vides).  
- Les règles du Sudoku doivent être respectées (pas de contradiction
  initiale).  

**Postconditions** :  
- Le système affiche une solution unique (si elle existe) ou informe de
  l’absence/ambigüité de solution.  

**Scénario principal** :  
1. L’utilisateur charge ou saisit une grille partiellement remplie.  
2. Le système analyse la validité de la grille.  
3. Le système applique son algorithme de résolution.  
4. Le système affiche la grille complétée.  

**Scénarios alternatifs** :  
- **Erreur de saisie** : Si la grille est invalide (contradictions), le
                         système notifie l’utilisateur et refuse la
                         résolution.  

**Fréquence d’occurrence** : Fréquente, utilisé par la majorité des
                             usagers.  
**Autres commentaires** : Fonction de base, doit être rapide et fiable.  

---

## 6.2 Génération de nouvelles grilles

**Nom** : Générer une grille de Sudoku  
**Niveau** : Objectif usager  
**Acteurs** :  
- Acteur primaire : Utilisateur  
- Acteur secondaire : Générateur intégré  

**Préconditions** :  
- Le système est en état de fonctionnement normal.  

**Postconditions** :  
- Une nouvelle grille valide et unique est proposée à l’utilisateur, selon
  un niveau de difficulté choisi.  

**Scénario principal** :  
1. L’utilisateur choisit une option de difficulté (facile, moyen, difficile).  
2. Le système génère une grille respectant les règles du Sudoku.  
3. La grille est affichée à l’écran pour être jouée ou exportée.  

**Scénarios alternatifs** :  
- **Surcharge système** : si le générateur échoue à trouver une grille
                          dans un délai raisonnable, le système propose une
                          difficulté différente ou notifie un échec
                          temporaire.  

**Fréquence d’occurrence** : Fréquent pour les utilisateurs joueurs.  
**Autres commentaires** : Le temps de génération doit rester acceptable
                          (< 2 s).  

---

## 6.3 Jouer interactivement au Sudoku

**Nom** : Jouer une partie de Sudoku  
**Niveau** : Objectif usager  
**Acteurs** :  
- Acteur primaire : Utilisateur joueur  
- Acteur secondaire : Interface utilisateur (console, GUI ou web)  

**Préconditions** :  
- Une grille est chargée (générée ou importée).  

**Postconditions** :  
- L’utilisateur peut saisir des chiffres, obtenir de l’aide, et
  éventuellement compléter la grille.  

**Scénario principal** :  
1. L’utilisateur sélectionne une case vide et saisit un chiffre.  
2. Le système valide l’entrée (respect des règles locales).  
3. La progression est sauvegardée automatiquement.  

**Scénarios alternatifs** :  
- **Erreur utilisateur** : si le chiffre viole une règle, un message
                           d’erreur s’affiche.  
- **Mode aide** : l’utilisateur demande des indices ou la mise en évidence
                  des erreurs.  

**Fréquence d’occurrence** : Très fréquent pour les utilisateurs joueurs.  
**Autres commentaires** : Interface ergonomique souhaitée, avec options de
                          sauvegarde/reprise.  

---

## 6.4 Fonctions complémentaires

**Nom** : Exporter/Imprimer une grille  
**Niveau** : Sous-fonction  
**Acteurs** :  
- Acteur primaire : Utilisateur  
- Acteur secondaire : Système d’export  

**Préconditions** :  
- Une grille est ouverte.  

**Postconditions** :  
- Le fichier PDF ou image est généré et sauvegardé.  

**Scénario principal** :  
1. L’utilisateur demande l’exportation.  
2. Le système formate la grille et produit le fichier.  

**Scénarios alternatifs** :  
- Problème de droits d’écriture : message d’erreur, l’utilisateur doit
  changer d’emplacement.  

**Fréquence d’occurrence** : Occasionnel.  

---

\newpage


# 7. LE SOMMAIRE DES IMPACTS

## 7.1 Les impacts opérationnels

L'introduction de notre système proposé entraînera certains changements dans
les pratiques des utilisateurs amateurs de Sudoku traditionnel, ainsi que des
ajustements mineurs pour les développeurs et le personnel de support.  

**Impacts sur les utilisateurs**  
Le principal changement pour les utilisateurs est le passage d’une pratique
du Sudoku sur papier ou via des applications statiques vers un environnement
numérique interactif capable d’offrir une assistance dynamique. 
L’utilisateur doit désormais interagir avec une interface logicielle qui lui
permet :

- de jouer avec des grilles générées automatiquement
- de recevoir des suggestions ou indices calculés par le système
- de demander la résolution complète d’une grille donnée

Cela implique une familiarité minimale avec les interfaces informatiques
(souris, clavier, navigation par menus). L’expérience utilisateur demeurera
simple et intuitive, mais le joueur devra tout de même s’habituer à la
présence de nouvelles fonctionnalités mentionnées précédemment.

Les changements de procédure concernent surtout la manière dont l’utilisateur
crée ou résout ses grilles. Sans notre système, il doit construire
manuellement une grille ou la copier depuis une source externe. L'application
propose désormais une génération automatique et un enregistrement local.
Cela simplifie l’accès au contenu, mais réduit la flexibilité pour ceux
qui préférent composer leurs propres grilles sur papier.

Aucun changement majeur n’est prévu par rapport aux données saisies. Les
seules entrées nécessaires demeurent les chiffres placés dans la grille et
les paramètres de difficulté pour la génération de nouvelles parties.

**Impacts sur le développement**  
Du côté des développeurs, l’intégration de l’algorithme WFC introduit
de nouvelles considérations techniques.

La maintenance du code relié à la génération automatique demandera des
connaissances en algorithmes et en optimisation, notamment pour garantir
que les grilles produites respectent les contraintes du Sudoku.

Les développeurs devront également s’assurer que le système fonctionne de
manière performante sur divers systèmes d’exploitation (Windows, Linux,
macOS). Les tests de compatibilité et les mises à jour devront donc être
planifiés pour limiter les risques de régression.

**Impacts sur le support et l’entretien**  
Comme l’application est locale, aucun lien permanent avec un centre de traitement
ou un serveur distant n’est requis. Par conséquent, les exigences de support
en continu sont minimes.

Le support technique consistera principalement à fournir :

- des correctifs en cas de bogue
- des mises à jour de compatibilité
- un canal de communication (ex. : courriel ou forum) pour recueillir les
  suggestions et signalements d’erreurs.

Aucune nouvelle source de données externe n’est utilisée. L’application
repose entièrement sur des algorithmes internes et des entrées de
l’utilisateur.

Les exigences de rétention de données se limitent au stockage local des
grilles sauvegardées ou générées par l’utilisateur, qui peut les conserver
ou les supprimer à sa convenance.

**Impacts sur les opérations et la gestion du risque**  
Étant donné que le système ne dépend pas d’une infrastructure en ligne,
les risques opérationnels liés à la disponibilité du service ou à la perte
de connectivité sont pratiquement nuls.

En cas de bris matériel ou de panne du système, l’utilisateur peut simplement
réinstaller l’application.

Les coûts opérationnels demeurent faibles : aucun frais d’hébergement, de
bande passante ou de maintenance serveur n’est requis en dehors du dépôt
GitLab.

Un mode d’opération en cas de désastre n’est pas nécessaire, mais il est
recommandé d’offrir à l’utilisateur la possibilité de sauvegarder ses
grilles sous forme de fichiers exportables (par exemple en format .json ou .txt)
afin de prévenir toute perte accidentelle de progression.

## 7.2 Les impacts organisationnels

L’introduction du système proposé aura des impacts organisationnels mineurs,
principalement liés à la formation de base des utilisateurs et à la
répartition des responsabilités en matière de maintenance et de support
technique.

Comme le système est autonome et local, aucun changement majeur dans la
structure ou le personnel n’est requis.

**Impacts sur les utilisateurs**  
Les utilisateurs devront acquérir une brève formation afin de se familiariser
avec l’interface du système, notamment pour comprendre les fonctions
d’assistance, de génération et de sauvegarde de grilles.

Cette formation pourrait se faire par le biais des facilités offertes
mentionnées précédemment.
Aucune modification de rôle ni ajout de poste est donc nécessaire.

**Impacts sur les développeurs**  
Les développeurs auront la responsabilité accrue d’assurer la maintenance
du code de génération (algorithme WFC) et la compatibilité du système avec
les différents environnements d’exploitation.

Cela implique un suivi périodique du dépôt GitLab, la gestion des versions,
ainsi que la correction des anomalies signalées par les utilisateurs.
Aucun ajout de poste n’est requis, mais les compétences en algorithmique et
en optimisation devront être présentes au sein de l’équipe.

**Impacts sur le support technique**  
Le personnel de support devra être en mesure de fournir une assistance
ponctuelle aux utilisateurs pour la résolution de problèmes d’installation
ou d’utilisation.

Un petit volume de demandes est anticipé, étant donné la simplicité du
système et son exécution locale.

Un canal de communication (adresse courriel ou forum) suffira pour le suivi.
Aucun poste supplémentaire ni changement de localisation du personnel n’est requis.

**Impacts en situation d’urgence**  
Puisque l’application fonctionne localement et ne dépend pas d’une
infrastructure distante, aucun personnel dédié à la gestion d’urgence
n’est requis.

En cas de défaillance ou de désastre (ex. : panne matérielle), la
réinstallation du logiciel et la récupération des fichiers sauvegardés
suffisent à restaurer le système.
Aucune formation spécifique ou plan de relève n’est donc nécessaire.

## 7.3 Les impacts durant le développement

Les impacts opérationnels anticipés pendant cette phase demeurent limités,
compte tenu de la simplicité du système et de son fonctionnement local.

**Impacts sur les utilisateurs**  
Il n'y aura pas d'utilisateurs qui seront impliqués au développement de
l'application.

**Impacts sur les développeurs**  
Les développeurs seront les principaux acteurs durant cette phase.

Ils auront la responsabilité :

- de concevoir, tester et améliorer l’algorithme de génération WFC
- de réaliser les essais de performance et de stabilité du système sur
  différents systèmes d’exploitation (Windows, Linux, macOS)

Cette période exigera une charge de travail accrue, notamment lors des tests de
cohérence et de validation fonctionnelle.

**Impacts sur le support et l’entretien**  
Le personnel de support sera impliqué à titre consultatif pendant le
développement, principalement pour :

- assister les développeurs dans la documentation technique et la mise en
  place des outils de suivi des bogues
- préparer les procédures de support qui seront utilisées une fois le
  système déployé
- tester le processus d’installation afin d’anticiper les problèmes
  courants d’utilisation.

Aucun service de support continu n’est nécessaire pendant la phase de
développement, puisque le système n’est pas encore en production.

**Opération parallèle et phase de tests**  
Aucune opération parallèle avec un système existant n’est requise, le
système étant entièrement nouveau.

Cependant, une phase de tests internes permettra de comparer les résultats
produits par l’algorithme WFC avec des grilles de Sudoku standards afin de
valider la conformité et la qualité des solutions générées.

Ces essais se feront localement et n’auront aucun impact sur les opérations
ou les utilisateurs finaux.

\newpage

# 8. L’ANALYSE DU SYSTÈME PROPOSÉ

## 8.1 Le sommaire des améliorations

**Nouvelles fonctionnalités**  
Comme mentionnée précédemment, nous avons les fonctionalitées suivantes:

- **Génération procédurale** de grilles de Sudoku
- **Résolution automatique** de grilles de Sudoku (grâce à une
  implémentation de WFC)
- **Validation automatique** de grilles de Sudoku
- **Importation** par l'utilisateur de ses propres grilles
- Permet de **jouer au Sudoku** avec ses grilles importées ou ceux générées
  par l'application
- **Retour en temps réel**

**Fonctionnalités améliorées**

- Expérience interactif : la combinaison de grilles générées et importées
  avec la validation en temps réel et les indices rend l’expérience de jeu
  plus fluide et pédagogique.

- Retour instantané sur la saisie : l’utilisateur bénéficie d’une
  assistance immédiate, détectant erreurs et incohérences dès qu’elles
  surviennent, ce qui améliore la qualité et la satisfaction lors du jeu.

**Améliorations de performance**

- Le système fonctionne localement, avec un temps de réponse presque
  instantané pour la génération et la validation des grilles.

- La gestion des données est optimisée, nécessitant un stockage minimal
  uniquement pour les grilles sauvegardées par l’utilisateur.

- La qualité des grilles générées est garantie par l’algorithme WFC,
  assurant que toutes les grilles sont valides, uniques et résolubles, ce qui
  réduit les risques d’erreurs ou de frustrations pour l’utilisateur.

## 8.2 Les inconvénients et limites

Bien que l’application de Sudoku proposée apporte de nombreux bénéfices,
certains inconvénients et limites sont à considérer.

**Inconvénients pour les utilisateurs**

- Réduction de flexibilité pour les utilisateurs avancés : les utilisateurs
  qui préfèrent composer leurs propres grilles sur papier peuvent trouver
  l’expérience numérique moins flexible, notamment pour la création et
  la personnalisation des grilles.

- Formation légère nécessaire : bien que l’interface soit intuitive, les
  utilisateurs doivent se familiariser avec les nouvelles fonctionnalités
  (validation automatique, indices, génération WFC), ce qui peut nécessiter
  un tutoriel ou un guide d’utilisation.

**Inconvénients pour le développement et le support**

- Complexité de l’algorithme WFC : l’intégration et la maintenance de
  l’algorithme de génération peuvent représenter un défi pour les
  développeurs, nécessitant des compétences en algorithmique et en
  optimisation.

- Tests et validation supplémentaires : la garantie que toutes les grilles
  générées sont valides et résolubles implique un effort accru de tests, ce
  qui peut prolonger le temps de développement.

## 8.3 Les alternatives et compromis considérés

Afin de satisfaire les utilisateurs avancés et régler la réduction de la
flexibilité, l'implémentation de l'importation des grilles personnalisées
a été planifié.

L'ajout de la FAQ intégrée dans l'application offrira la formation légère
que certains utilisateurs auront besoin sans les forcer à quitter
l'application pour accèder la documentation complète.

\newpage

# 9. NOTES

N/A

# 10. ANNEXES

N/A

# 11. GLOSSAIRE

\large
\noindent
\begin{tabularx}{\textwidth}{|l|X|}
\hline
FAQ & Foire aux questions \\ \hline
\end{tabularx}
