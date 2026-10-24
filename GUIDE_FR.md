# [Rustlings](https://rustlings.rust-lang.org) 🦀 — Guide en français

Petits exercices pour vous habituer à lire et à écrire du code [Rust](https://www.rust-lang.org) — _à suivre en parallèle de la lecture du [livre officiel de Rust](https://doc.rust-lang.org/book) 📚️_ (en anglais ; une [traduction française](https://jimskapt.github.io/rust-book-fr/) non officielle existe).

> Traduction des instructions du dépôt : `README.md`, pages « Setup » et « Usage » du site, ainsi que les `README.md` de chaque thème d'exercices. Les liens pointent vers les ressources originales (en anglais).

---

## 1. Installation

### Installer Rust

Avant d'installer Rustlings, vous devez avoir installé la **dernière version de Rust**.
Rendez-vous sur [www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) pour les instructions.
Cela installe aussi _Cargo_, le gestionnaire de paquets/projets de Rust.

> 🐧 Sous **Linux**, assurez-vous que `gcc` est installé (_pour l'éditeur de liens_).
>
> Debian : `sudo apt install gcc`\
> Fedora : `sudo dnf install gcc`
>
> 🍎 Sous **MacOS**, assurez-vous que _Xcode et ses outils de développement_ sont installés : `xcode-select --install`

### Installer Rustlings

La commande suivante télécharge et compile Rustlings :

```bash
cargo install rustlings
```

**Si l'installation échoue…**

- Vérifiez que vous avez la dernière version de Rust en lançant `rustup update`
- Essayez avec l'option `--locked` : `cargo install rustlings --locked`
- Sinon, [signalez le problème](https://github.com/rust-lang/rustlings/issues/new)

### Initialisation

Une fois Rustlings installé, lancez la commande suivante pour initialiser le dossier `rustlings/` :

```bash
rustlings init
```

**Si la commande `rustlings` est introuvable…**

Vous êtes probablement sous Linux et avez installé Rust via votre gestionnaire de paquets.

Cargo installe les binaires dans le dossier `~/.cargo/bin`.
Malheureusement, les gestionnaires de paquets n'ajoutent souvent pas `~/.cargo/bin` à la variable d'environnement `PATH`.

- Soit vous ajoutez `~/.cargo/bin` manuellement au `PATH`
- Soit vous désinstallez Rust du gestionnaire de paquets et vous [l'installez de la manière officielle avec `rustup`](https://www.rust-lang.org/tools/install)

Ensuite, allez dans le dossier nouvellement initialisé et lancez Rustlings pour obtenir la suite des instructions sur la prise en main des exercices :

```bash
cd rustlings/
rustlings
```

### Environnement de travail

**Éditeur** — Notre recommandation générale est [VS Code](https://code.visualstudio.com/) avec l'[extension rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).
Mais tout éditeur compatible avec [rust-analyzer](https://rust-analyzer.github.io/) suffit pour travailler sur les exercices.

**Terminal** — Utilisez un terminal moderne pour une meilleure expérience.
Le terminal par défaut sous Linux et Mac est suffisant.
Sous Windows, nous recommandons le [Windows Terminal](https://aka.ms/terminal).

**Documentation hors ligne** — Si vous travaillez sans connexion, vous pouvez accéder à une copie locale du livre ou de la documentation de la bibliothèque standard avec `rustup doc --book` ou `rustup doc --std`.

---

## 2. Utilisation

### Faire les exercices

Les exercices sont classés par thème et se trouvent dans le sous-dossier `exercises/<thème>`.
Chaque thème contient un fichier `README.md` avec quelques ressources pour bien démarrer.
Nous vous recommandons vivement de les consulter avant de commencer 📚️

La plupart des exercices contiennent une erreur qui les empêche de compiler : c'est à vous de la corriger !
Certains exercices contiennent des tests qui doivent passer pour que l'exercice soit considéré comme réussi ✅

Cherchez `TODO` et `todo!()` pour savoir ce qu'il faut modifier.
Demandez un indice en saisissant `h` dans le _mode watch_ 💡

### Mode watch

Après l'[initialisation](#initialisation), Rustlings se lance simplement avec la commande `rustlings`.

Cela démarre le _mode watch_, qui vous guide à travers les exercices dans un ordre prédéfini (celui que nous pensons le meilleur pour les débutants).
L'exercice en cours est relancé automatiquement chaque fois que vous modifiez son fichier dans le dossier `exercises/`.

**Si la détection des modifications de fichiers dans `exercises/` ne fonctionne pas…**

Vous pouvez ajouter l'option **`--manual-run`** (`rustlings --manual-run`) pour relancer manuellement l'exercice en cours en saisissant `r` dans le mode watch.

Merci de [signaler le problème](https://github.com/rust-lang/rustlings/issues/new) en indiquant votre système d'exploitation et si vous exécutez Rustlings dans un conteneur ou une machine virtuelle (p. ex. WSL).

### Liste des exercices

Dans le [mode watch](#mode-watch) (après avoir lancé `rustlings`), saisissez `l` pour ouvrir la liste interactive des exercices.

La liste permet de…

- Voir l'état de tous les exercices (terminé ou en attente)
- `c` : Continuer à un autre exercice (en sauter temporairement ou revenir à un précédent)
- `r` : Réinitialiser l'état et le fichier de l'exercice sélectionné (il faut ensuite _recharger/rouvrir_ son fichier dans votre éditeur)

Consultez le pied de la liste pour connaître toutes les touches disponibles.

### Des questions ?

Si vous avez besoin d'aide pendant les exercices et que les indices intégrés ne suffisent pas, n'hésitez pas à poser votre question dans les discussions [_Q&A_](https://github.com/rust-lang/rustlings/discussions/categories/q-a?discussions_q=) si elle n'y a pas déjà été traitée 💡

### Et après ?

Une fois Rustlings terminé, mettez vos nouvelles connaissances à profit !
Continuez à vous entraîner en construisant vos propres projets, en contribuant à Rustlings ou en trouvant d'autres projets open source auxquels contribuer.

> Pour créer vos propres exercices Rustlings, consultez la page [**exercices communautaires**](https://rustlings.rust-lang.org/community-exercises/) 🏗️

### Solutions officielles

Tant que vous n'avez pas terminé un exercice, son fichier de solution (dans `solutions/`) ne contient qu'une fonction `main` vide.
Son contenu est automatiquement remplacé par la vraie solution une fois l'exercice terminé.

Notez que ces solutions ne sont souvent qu'_une possibilité_ parmi d'autres pour résoudre un exercice.

---

## 3. Correspondance exercices ↔ chapitres du livre

| Exercice         | Chapitre du livre |
| ---------------- | ----------------- |
| variables        | §3.1              |
| functions        | §3.3              |
| if               | §3.5              |
| primitive_types  | §3.2, §4.3        |
| vecs             | §8.1              |
| move_semantics   | §4.1-2            |
| structs          | §5.1, §5.3        |
| enums            | §6, §19.3         |
| strings          | §8.2              |
| modules          | §7                |
| hashmaps         | §8.3              |
| options          | §10.1             |
| error_handling   | §9                |
| generics         | §10               |
| traits           | §10.2             |
| lifetimes        | §10.3             |
| tests            | §11.1             |
| iterators        | §13.2-4           |
| smart_pointers   | §15, §16.3        |
| threads          | §16.1-3           |
| macros           | §20.5             |
| clippy           | Annexe D          |
| conversions      | n/a               |

---

## 4. Présentation de chaque thème

### 00 — Intro

Rust utilise les macros `print!` et `println!` pour afficher du texte dans la console.

- [Hello World](https://doc.rust-lang.org/rust-by-example/hello.html)
- [Affichage formaté](https://doc.rust-lang.org/rust-by-example/hello/print.html)

### 01 — Variables

En Rust, les variables sont immuables par défaut.
Quand une variable est immuable, une fois une valeur liée à un nom, vous ne pouvez plus changer cette valeur.
Vous pouvez les rendre mutables en ajoutant `mut` devant le nom de la variable.

- [Variables et mutabilité](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)

### 02 — Fonctions

Ici, vous apprendrez à écrire des fonctions et comment le compilateur Rust peut vous aider à déboguer les erreurs, même dans du code plus complexe.

- [Comment fonctionnent les fonctions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)

### 03 — If

Vous allez apprendre ici `if`, la structure de contrôle la plus basique (mais étonnamment polyvalente !).

- [Contrôle de flux — expressions `if`](https://doc.rust-lang.org/book/ch03-05-control-flow.html#if-expressions)

### 04 — Types primitifs

Rust possède quelques types de base directement implémentés dans le compilateur. Dans cette section, nous passerons en revue les plus importants.

- [Types de données](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Le type slice](https://doc.rust-lang.org/book/ch04-03-slices.html)

### 05 — Vecteurs

Les vecteurs sont l'une des structures de données les plus utilisées en Rust. Dans d'autres langages, on les appellerait simplement des tableaux, mais comme Rust travaille à un niveau un peu plus bas, un tableau (array) en Rust est stocké sur la pile (stack) — il ne peut donc ni grandir ni rétrécir, et sa taille doit être connue à la compilation — alors qu'un vecteur est stocké sur le tas (heap), où ces restrictions ne s'appliquent pas.

Les vecteurs arrivent plutôt tard dans le livre, mais nous pensons qu'ils sont assez utiles pour en parler plus tôt. Nous parlerons plus tard de l'autre structure de données utile : les tables de hachage (hash maps).

- [Stocker des listes de valeurs avec les vecteurs](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [`iter_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.iter_mut)
- [`map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)

### 06 — Sémantique de déplacement (Move semantics)

Ces exercices sont adaptés du [Rust Tutorial](https://pnkfelix.github.io/rust-examples-icfp2014/) de [pnkfelix](https://github.com/pnkfelix) — merci Felix !!!

Pour cette section, les liens vers le livre sont particulièrement importants.

- [Ownership (propriété)](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [Références et emprunts (borrowing)](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

### 07 — Structs

Rust possède trois types de structures : une struct classique à la C, une struct tuple et une struct unité.

- [Structures](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
- [Syntaxe des méthodes](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)

### 08 — Enums

Rust vous permet de définir des types appelés « enums » qui énumèrent les valeurs possibles.
Les enums existent dans de nombreux langages, mais leurs capacités diffèrent. Les enums de Rust ressemblent le plus aux types de données algébriques des langages fonctionnels comme F#, OCaml et Haskell.
Le « pattern matching » (filtrage par motifs) de Rust, qui permet d'exécuter facilement du code différent selon la valeur d'une énumération, est très utile en combinaison avec les enums.

- [Enums](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [Syntaxe des motifs](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html)

### 09 — Chaînes de caractères

Rust a deux types de chaînes : la slice de chaîne (`&str`) et la chaîne possédée (`String`).
Nous ne dicterons pas quand utiliser l'une ou l'autre, mais nous vous montrerons comment les identifier, les créer et les utiliser.

- [Strings (livre Rust)](https://doc.rust-lang.org/book/ch08-02-strings.html)
- [Méthodes de `str`](https://doc.rust-lang.org/std/primitive.str.html)
- [Méthodes de `String`](https://doc.rust-lang.org/std/string/struct.String.html)

### 10 — Modules

Dans cette section, nous vous présentons une introduction au système de modules de Rust.

- [Le système de modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

### 11 — Hashmaps

Une _table de hachage_ (hash map) permet d'associer une valeur à une clé donnée.
Vous la connaissez peut-être sous le nom d'[_unordered map_ en C++](https://en.cppreference.com/w/cpp/container/unordered_map), de [_dictionnaire_ en Python](https://docs.python.org/3/tutorial/datastructures.html#dictionaries) ou de _tableau associatif_ dans d'autres langages.

C'est l'autre structure de données dont nous avons parlé plus tôt à propos des Vec.

- [Stocker des clés avec des valeurs associées dans des Hash Maps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)

### 12 — Options

Le type `Option` représente une valeur optionnelle : chaque `Option` est soit `Some` et contient une valeur, soit `None` et n'en contient pas.
Les types Option sont très courants en Rust, car ils ont de nombreux usages :

- Valeurs initiales
- Valeurs de retour de fonctions non définies sur tout leur domaine d'entrée (fonctions partielles)
- Valeur de retour pour signaler de simples erreurs, `None` étant renvoyé en cas d'erreur
- Champs de struct optionnels
- Champs de struct qui peuvent être prêtés ou « pris » (taken)
- Arguments de fonction optionnels
- Pointeurs nullables
- Extraire des valeurs de situations délicates

Pour aller plus loin :

- [Format de l'enum Option](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-enum-definitions)
- [Documentation du module Option](https://doc.rust-lang.org/std/option/)
- [Documentation de l'enum Option](https://doc.rust-lang.org/std/option/enum.Option.html)
- [if let](https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html)
- [while let](https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html)

### 13 — Gestion des erreurs

La plupart des erreurs ne sont pas assez graves pour obliger le programme à s'arrêter complètement.
Parfois, quand une fonction échoue, c'est pour une raison que vous pouvez facilement interpréter et traiter.
Par exemple, si vous essayez d'ouvrir un fichier et que l'opération échoue parce qu'il n'existe pas, vous pouvez vouloir créer le fichier plutôt que de terminer le processus.

- [Gestion des erreurs](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
- [Génériques](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Result](https://doc.rust-lang.org/rust-by-example/error/result.html)
- [Boxer les erreurs](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html)

### 14 — Génériques

Les génériques consistent à généraliser des types et des fonctionnalités à des cas plus larges.
C'est extrêmement utile pour réduire la duplication de code de bien des façons, mais cela peut nécessiter une syntaxe assez élaborée.
En effet, être générique demande de préciser avec soin pour quels types un type générique est réellement considéré comme valide.
L'usage le plus simple et le plus courant des génériques est celui des paramètres de type.

- [Types de données génériques](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Bornes (bounds)](https://doc.rust-lang.org/rust-by-example/generics/bounds.html)

### 15 — Traits

Un trait est un ensemble de méthodes.

Les types de données peuvent implémenter des traits. Pour cela, on définit pour le type les méthodes qui composent le trait. Par exemple, le type `String` implémente le trait `From<&str>`, ce qui permet d'écrire `String::from("hello")`.

En ce sens, les traits ressemblent un peu aux interfaces Java et aux classes abstraites C++.

Parmi les autres traits courants de Rust :

- `Clone` (la méthode `clone`)
- `Display` (qui permet l'affichage formaté via `{}`)
- `Debug` (qui permet l'affichage formaté via `{:?}`)

Comme les traits expriment un comportement partagé entre types de données, ils sont utiles pour écrire du code générique.

- [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)

### 16 — Durées de vie (Lifetimes)

Les durées de vie indiquent au compilateur comment vérifier que les références vivent assez longtemps pour être valides dans une situation donnée. Par exemple, elles disent : « assure-toi que le paramètre 'a vit aussi longtemps que le paramètre 'b pour que la valeur de retour soit valide ».

Elles ne sont nécessaires que pour les emprunts, c.-à-d. les références, puisque les paramètres copiés ou déplacés sont possédés dans leur portée et ne peuvent pas être référencés à l'extérieur. Les durées de vie permettent de vérifier que le code appelant, par exemple celui de fonctions, fournit des arguments valides. Les durées de vie sont contraignantes pour les appelants.

Si vous souhaitez en savoir plus sur les annotations de durée de vie, le projet [lifetimekata](https://tfpk.github.io/lifetimekata/) propose des exercices dans un style similaire à Rustlings, entièrement consacrés à l'écriture d'annotations de durée de vie.

- [Lifetimes (dans Rust By Example)](https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime.html)
- [Valider les références avec les durées de vie](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)

### 17 — Tests

On sort ici de l'ordre du livre pour aborder les tests — beaucoup des exercices suivants vous demanderont de faire passer des tests !

- [Écrire des tests](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)

### 18 — Itérateurs

Cette section vous apprend à utiliser les itérateurs.

- [Iterator](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [Documentation d'Iterator](https://doc.rust-lang.org/stable/std/iter/)

### 19 — Pointeurs intelligents (Smart pointers)

En Rust, les pointeurs intelligents sont des variables qui contiennent une adresse mémoire et référencent d'autres données, mais qui possèdent aussi des métadonnées et des capacités supplémentaires.
Les pointeurs intelligents de Rust possèdent souvent les données vers lesquelles ils pointent, alors que les références ne font que les emprunter.

- [Pointeurs intelligents](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [Utiliser `Box` pour pointer vers des données sur le tas](https://doc.rust-lang.org/book/ch15-01-box.html)
- [`Rc<T>`, le pointeur intelligent à comptage de références](https://doc.rust-lang.org/book/ch15-04-rc.html)
- [Concurrence à état partagé](https://doc.rust-lang.org/book/ch16-03-shared-state.html)
- [Documentation de Cow](https://doc.rust-lang.org/std/borrow/enum.Cow.html)

### 20 — Threads

Dans la plupart des systèmes d'exploitation actuels, le code d'un programme exécuté tourne dans un processus, et le système gère plusieurs processus à la fois.
Au sein de votre programme, vous pouvez aussi avoir des parties indépendantes qui s'exécutent simultanément. Les fonctionnalités qui exécutent ces parties indépendantes s'appellent des threads.

- [Exemple du dîner des philosophes](https://doc.rust-lang.org/1.4.0/book/dining-philosophers.html)
- [Utiliser des threads pour exécuter du code simultanément](https://doc.rust-lang.org/book/ch16-01-threads.html)
- [Utiliser le passage de messages pour transférer des données entre threads](https://doc.rust-lang.org/book/ch16-02-message-passing.html)

### 21 — Macros

Le système de macros de Rust est très puissant, mais aussi assez difficile à appréhender. Nous n'allons pas vous apprendre à écrire des macros complètes. Nous vous montrerons plutôt comment les utiliser et en créer.

Si vous souhaitez en savoir plus sur l'écriture de vos propres macros, le projet [macrokata](https://github.com/tfpk/macrokata) propose des exercices dans un style similaire à Rustlings, entièrement consacrés à l'apprentissage des macros.

- [Le livre Rust — Macros](https://doc.rust-lang.org/book/ch20-05-macros.html)
- [The Little Book of Rust Macros](https://veykril.github.io/tlborm/)
- [Rust by Example — macro_rules!](https://doc.rust-lang.org/rust-by-example/macros.html)

### 22 — Clippy

L'outil Clippy est un ensemble de « lints » qui analysent votre code pour repérer les erreurs courantes et améliorer votre code Rust.

Si vous avez utilisé le script d'installation de Rustlings, Clippy devrait déjà être installé.
Sinon, vous pouvez l'installer manuellement avec `rustup component add clippy`.

- [Dépôt GitHub](https://github.com/rust-lang/rust-clippy)

### 23 — Conversions de types

Rust offre de multiples façons de convertir une valeur d'un type en un autre.

La forme la plus simple de conversion est l'expression de cast, notée avec l'opérateur binaire `as`. Par exemple, `println!("{}", 1 + 1.0);` ne compilerait pas, car `1` est un entier et `1.0` un flottant. En revanche, `println!("{}", 1 as f32 + 1.0)` devrait compiler. L'exercice `conversions1` couvre ce point.

Rust propose aussi des traits qui facilitent les conversions de types une fois implémentés. Ils se trouvent dans le module [`convert`](https://doc.rust-lang.org/std/convert/index.html) :

- `From` et `Into`, vus dans `conversions2`
- `TryFrom` et `TryInto`, vus dans `conversions4`
- `AsRef` et `AsMut`, vus dans `conversions5`

De plus, le module `std::str` offre un trait [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) qui aide à convertir des chaînes vers des types cibles via la méthode `parse`. Si `FromStr` est correctement implémenté pour un type `Person`, alors `let p: Person = "Mark,20".parse().unwrap()` doit compiler et s'exécuter sans panique.

Ce sont les principaux moyens ***au sein de la bibliothèque standard*** de convertir des données vers les types voulus.

Ces notions ne sont pas directement traitées dans le livre, mais la bibliothèque standard en a une excellente documentation :

- [conversions](https://doc.rust-lang.org/std/convert/index.html)
- [Trait `FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html)

### 24 — Async

La programmation asynchrone est un modèle où les tâches sont déléguées à un runtime qui les exécute de façon concurrente.
Elle est particulièrement efficace pour les applications qui effectuent beaucoup d'opérations d'E/S indépendantes, par exemple les serveurs web.

Rust fournit dans le langage les primitives nécessaires à la programmation asynchrone.
Cependant, la bibliothèque standard de Rust n'inclut pas de runtime.
Pour ces exercices, nous utiliserons le runtime courant `tokio`.

- [Fondamentaux de la programmation asynchrone](https://doc.rust-lang.org/book/ch17-00-async-await.html)
- [Documentation de Tokio](https://docs.rs/tokio/latest/tokio/)

### Quizzes

Après quelques sections, un quiz placé dans ce dossier testera vos connaissances sur plusieurs sections à la fois.

---

## 5. Premier exercice (`00_intro/intro1.rs`) — message d'accueil

Le premier exercice compile déjà. Les suivants contiennent une erreur de compilation ou de logique : tout le principe de Rustlings est de corriger ces erreurs pour résoudre les exercices. Bon courage !

Le fichier de cet exercice est `exercises/00_intro/intro1.rs`. Jetez-y un œil !
Le chemin de l'exercice en cours est toujours affiché sous la barre de progression, et vous pouvez cliquer dessus pour ouvrir le fichier dans votre éditeur.

Le fichier est rechargé dès que vous modifiez l'une de ses lignes : essayez d'ajouter un `println!` et regardez la sortie mise à jour dans le terminal. Quand vous avez tout fait fonctionner et êtes prêt pour la suite, saisissez `n` dans le terminal.
